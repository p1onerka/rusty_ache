//! Defines the main application window, input handling, and rendering update loop using winit and pixels crates.
//!
//! This module implements the `App` struct, which manages the window, screen buffer, pixel data,
//! and keyboard input state. It integrates with the winit event loop to handle window events,
//! update pixel frames, and process user keyboard input.
//!
//! The example function demonstrates initializing shared pixel data and window, spawning a producer thread
//! to modify pixel data dynamically, and running the event loop to render changes to the screen.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::Resolution;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::KeyEvent;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

/// Screen dimensions constants.
pub const WIDTH: u32 = 300;
pub const HEIGHT: u32 = 300;

/// Represents the screen on which game frames are drawn.
///
/// Wraps the `Pixels` buffer and provides methods for pixel frame updates.
pub struct Screen<'a> {
    pixels: Pixels<'a>,
}

impl Screen<'_> {
    /// Creates a new `Screen` attached to the specified window and resolution.
    ///
    /// # Errors
    /// Returns a `pixels::Error` if pixel buffer initialization fails.
    pub fn new(window: Arc<Window>, resolution: Resolution) -> Result<Self, pixels::Error> {
        let surface_texture =
            SurfaceTexture::new(resolution.width, resolution.height, window.clone());
        let pixels = Pixels::new(resolution.width, resolution.height, surface_texture)?;
        Ok(Self { pixels })
    }

    /// Updates the pixel frame with new RGBA color data and renders it.
    ///
    /// # Parameters
    /// - `pixel_colors`: Slice of RGBA tuples representing new frame pixel data.
    pub fn update(&mut self, pixel_colors: &[(u8, u8, u8, u8)]) {
        let cur_frame = self.pixels.frame_mut();
        for (i, &(r, g, b, a)) in pixel_colors.iter().enumerate() {
            let base = i * 4;
            cur_frame[base] = r;
            cur_frame[base + 1] = g;
            cur_frame[base + 2] = b;
            cur_frame[base + 3] = a;
        }
        let _ = self.pixels.render();
    }
}

/// Type alias for pixel color data vectors.
type PixelData = Vec<(u8, u8, u8, u8)>;

/// Holds the pressed state of movement keys (WASD) via atomic booleans for thread-safe access.
pub struct Keys {
    pub w: AtomicBool,
    pub a: AtomicBool,
    pub s: AtomicBool,
    pub d: AtomicBool,
}

/// Main GUI application struct.
///
/// Holds references to the window, screen, pixel buffer, and keyboard input state.
/// Tracks frame count and timing for optional FPS measurements.
pub struct App {
    /// Reference to the main window, inside a read-write lock.
    window: Arc<RwLock<Option<Arc<Window>>>>,
    /// The `Screen` object rendering pixel frames.
    screen: Option<Screen<'static>>,
    /// Shared pixel data provided by the renderer.
    pixel_data: Arc<RwLock<PixelData>>,
    /// Atomic flags indicating pressed state for WASD keys.
    pub(crate) keys_pressed: Arc<Keys>,

    /// Frame count for FPS calculation.
    frame_count: u32,
    /// Timestamp of last FPS measurement.
    last_fps_report_time: Instant,
}

impl App {
    /// Constructs a new App with shared pixel data and window references.
    pub fn new(
        pixel_data: Arc<RwLock<PixelData>>,
        window: Arc<RwLock<Option<Arc<Window>>>>,
    ) -> Self {
        App {
            screen: None,
            pixel_data,
            window,
            //key_pressed: Arc::new(RwLock::new(None)),
            keys_pressed: Arc::new(Keys {
                w: AtomicBool::new(false),
                a: AtomicBool::new(false),
                s: AtomicBool::new(false),
                d: AtomicBool::new(false),
            }),
            frame_count: 0,
            last_fps_report_time: Instant::now(),
        }
    }

    /// Placeholder run method; main loop handled by `winit` event loop.
    pub fn run(&mut self) {}
}

impl ApplicationHandler for App {
    /// Called when the application is resumed or started.
    ///
    /// Creates the window and initializes the `Screen`.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_size = LogicalSize::new(WIDTH / 2, HEIGHT / 2);
        let window_attributes = WindowAttributes::default()
            /*.with_title("rusty_ache")*/
            .with_inner_size(window_size)
            .with_min_inner_size(window_size)
            .with_max_inner_size(window_size);
        let window = event_loop.create_window(window_attributes).unwrap();

        let arc = Arc::new(window);
        {
            let mut shared_window_lock = self.window.write().unwrap();
            *shared_window_lock = Some(arc.clone());
        }

        let resolution = Resolution {
            width: WIDTH,
            height: HEIGHT,
        };
        match Screen::new(arc, resolution) {
            Ok(screen) => {
                self.screen = Some(screen);
            }
            Err(e) => eprintln!("Screen object initialization error: {:?}", e),
        }
    }

    /// Handles window events such as close requests, redraw requests, and keyboard input.
    ///
    /// - CloseRequested: exits event loop.
    /// - RedrawRequested: updates the screen with new pixels and optionally calculates FPS.
    /// - KeyboardInput: updates atomic key states for WASD keys.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let screen = match self.screen.as_mut() {
            Some(s) => s,
            None => return,
        };
        match event {
            WindowEvent::CloseRequested => {
                //dbg!("Bye bye");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let pixel_data = match self.pixel_data.read() {
                    Ok(data) => data,
                    Err(_) => {
                        eprintln!("Couldn't get data from provider");
                        return;
                    }
                };

                screen.update(&pixel_data);

                // FPS computation
                self.frame_count += 1;
                let elapsed = self.last_fps_report_time.elapsed();
                if elapsed >= Duration::from_secs(1) {
                    self.frame_count = 0;
                    self.last_fps_report_time = Instant::now();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        ..
                    },
                ..
            } => {
                let pressed = state.is_pressed();

                match key_code {
                    KeyCode::KeyW => self.keys_pressed.w.store(pressed, Ordering::Relaxed),
                    KeyCode::KeyA => self.keys_pressed.a.store(pressed, Ordering::Relaxed),
                    KeyCode::KeyS => self.keys_pressed.s.store(pressed, Ordering::Relaxed),
                    KeyCode::KeyD => self.keys_pressed.d.store(pressed, Ordering::Relaxed),
                    _ => {}
                }
            }
            _ => (),
        }
    }
}

/// Example function demonstrating app initialization and running.
///
/// Sets up shared pixel buffers and windows, spawns a producer thread that
/// dynamically updates pixel colors in a loop, and runs the event loop.
pub fn example() {
    let initial_resolution = Resolution {
        width: WIDTH,
        height: HEIGHT,
    };
    let initial_pixels = vec![
        (0x00, 0x00, 0x00, 0xFF);
        (initial_resolution.width * initial_resolution.height) as usize
    ];

    let shared_pixel_data = Arc::new(RwLock::new(initial_pixels));
    let shared_window = Arc::new(RwLock::new(None));

    let shared_pixel_data_clone = shared_pixel_data.clone();
    let shared_window_clone = shared_window.clone();

    // Producer thread
    thread::spawn(move || {
        let window_arc: Arc<Window> = loop {
            if let Some(arc) = shared_window_clone.read().unwrap().clone() {
                break arc;
            }
            thread::sleep(Duration::from_millis(50));
        };

        //dbg!("Producer has started");

        let mut frame_count: u32 = 0; // for generating new color
        let screen_size = (WIDTH * HEIGHT) as usize;
        loop {
            let new_color = (frame_count * 0x10001 + 0x010000) % 0xFFFFFF; // gradient for testing
            let r = ((new_color >> 16) & 0xFF) as u8;
            let g = ((new_color >> 8) & 0xFF) as u8;
            let b = (new_color & 0xFF) as u8;
            let a = 0xFF_u8;
            {
                let mut pixels = shared_pixel_data_clone
                    .write()
                    .expect("Producer couldn't get lock to write new pixel data into App");

                for i in 0..screen_size {
                    pixels[i] = (r, g, b, a); // recolor all pixels into new color
                }
            }
            window_arc.request_redraw();
            frame_count = frame_count.wrapping_add(1);
        }
    });

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new(shared_pixel_data, shared_window);
    let _ = event_loop.run_app(&mut app);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    use std::sync::{Arc, RwLock};
    use std::thread;

    #[test]
    fn test_keys_new_all_false() {
        let keys = Keys {
            w: AtomicBool::new(false),
            a: AtomicBool::new(false),
            s: AtomicBool::new(false),
            d: AtomicBool::new(false),
        };

        assert_eq!(keys.w.load(Ordering::Relaxed), false);
        assert_eq!(keys.a.load(Ordering::Relaxed), false);
        assert_eq!(keys.s.load(Ordering::Relaxed), false);
        assert_eq!(keys.d.load(Ordering::Relaxed), false);
    }

    #[test]
    fn test_keys_store_and_load() {
        let keys = Keys {
            w: AtomicBool::new(false),
            a: AtomicBool::new(false),
            s: AtomicBool::new(false),
            d: AtomicBool::new(false),
        };

        keys.w.store(true, Ordering::Relaxed);
        keys.a.store(true, Ordering::SeqCst);

        assert_eq!(keys.w.load(Ordering::Relaxed), true);
        assert_eq!(keys.a.load(Ordering::SeqCst), true);
        assert_eq!(keys.s.load(Ordering::Relaxed), false);
        assert_eq!(keys.d.load(Ordering::Relaxed), false);
    }

    #[test]
    fn test_keys_toggle_operations() {
        let keys = Keys {
            w: AtomicBool::new(false),
            a: AtomicBool::new(false),
            s: AtomicBool::new(false),
            d: AtomicBool::new(false),
        };

        keys.w.store(true, Ordering::Relaxed);
        assert_eq!(keys.w.load(Ordering::Relaxed), true);
        keys.w.store(false, Ordering::Relaxed);
        assert_eq!(keys.w.load(Ordering::Relaxed), false);
    }

    #[test]
    fn test_keys_all_true() {
        let keys = Keys {
            w: AtomicBool::new(true),
            a: AtomicBool::new(true),
            s: AtomicBool::new(true),
            d: AtomicBool::new(true),
        };

        assert_eq!(keys.w.load(Ordering::Relaxed), true);
        assert_eq!(keys.a.load(Ordering::Relaxed), true);
        assert_eq!(keys.s.load(Ordering::Relaxed), true);
        assert_eq!(keys.d.load(Ordering::Relaxed), true);
    }

    #[test]
    fn test_keys_compare_and_swap() {
        let keys = Keys {
            w: AtomicBool::new(false),
            a: AtomicBool::new(false),
            s: AtomicBool::new(false),
            d: AtomicBool::new(false),
        };

        let old = keys.w.swap(true, Ordering::Relaxed);
        assert_eq!(old, false);
        assert_eq!(keys.w.load(Ordering::Relaxed), true);
    }

    #[test]
    fn test_width_constant() {
        assert_eq!(WIDTH, 300);
        assert!(WIDTH > 0);
    }

    #[test]
    fn test_height_constant() {
        assert_eq!(HEIGHT, 300);
        assert!(HEIGHT > 0);
    }

    #[test]
    fn test_app_new_initialization() {
        let pixel_data = Arc::new(RwLock::new(vec![(0, 0, 0, 0); 100]));
        let window = Arc::new(RwLock::new(None));

        let app = App::new(pixel_data.clone(), window.clone());

        assert_eq!(app.keys_pressed.w.load(Ordering::Relaxed), false);
        assert_eq!(app.keys_pressed.a.load(Ordering::Relaxed), false);
        assert_eq!(app.keys_pressed.s.load(Ordering::Relaxed), false);
        assert_eq!(app.keys_pressed.d.load(Ordering::Relaxed), false);

        assert_eq!(app.frame_count, 0);
    }

    #[test]
    fn test_app_run_method() {
        let pixel_data = Arc::new(RwLock::new(vec![(0, 0, 0, 0); 100]));
        let window = Arc::new(RwLock::new(None));

        let mut app = App::new(pixel_data, window);
        app.run();
    }

    #[test]
    fn test_app_keys_simulation() {
        let pixel_data = Arc::new(RwLock::new(vec![(0, 0, 0, 0); 100]));
        let window = Arc::new(RwLock::new(None));

        let app = App::new(pixel_data, window);

        app.keys_pressed.w.store(true, Ordering::Relaxed);
        app.keys_pressed.d.store(true, Ordering::Relaxed);

        assert_eq!(app.keys_pressed.w.load(Ordering::Relaxed), true);
        assert_eq!(app.keys_pressed.a.load(Ordering::Relaxed), false);
        assert_eq!(app.keys_pressed.s.load(Ordering::Relaxed), false);
        assert_eq!(app.keys_pressed.d.load(Ordering::Relaxed), true);

        app.keys_pressed.w.store(false, Ordering::Relaxed);
        app.keys_pressed.d.store(false, Ordering::Relaxed);

        assert_eq!(app.keys_pressed.w.load(Ordering::Relaxed), false);
        assert_eq!(app.keys_pressed.d.load(Ordering::Relaxed), false);
    }

    #[test]
    fn test_app_frame_counting_simulation() {
        let pixel_data = Arc::new(RwLock::new(vec![(0, 0, 0, 0); 100]));
        let window = Arc::new(RwLock::new(None));

        let mut app = App::new(pixel_data, window);

        for _ in 0..10 {
            app.frame_count += 1;
        }

        assert_eq!(app.frame_count, 10);
    }

    #[test]
    fn test_empty_pixel_data() {
        let pixel_data: PixelData = vec![];
        assert_eq!(pixel_data.len(), 0);
    }

    #[test]
    fn test_single_pixel_data() {
        let pixel_data: PixelData = vec![(1, 2, 3, 4)];
        assert_eq!(pixel_data.len(), 1);
        assert_eq!(pixel_data, vec![(1, 2, 3, 4)]);
    }
}
