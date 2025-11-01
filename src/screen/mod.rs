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

pub const WIDTH: u32 = 300;
pub const HEIGHT: u32 = 300;

/// Struct representing Screen, on which a game will be drawn
pub struct Screen<'a> {
    pixels: Pixels<'a>,
}

/// Implementation of Screen for main GUI application
impl Screen<'_> {
    pub fn new(window: Arc<Window>, resolution: Resolution) -> Result<Self, pixels::Error> {
        let surface_texture =
            SurfaceTexture::new(resolution.width, resolution.height, window.clone());
        let pixels = Pixels::new(resolution.width, resolution.height, surface_texture)?;
        Ok(Self { pixels })
    }

    /// Draw new RGB frame on Screen
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

type PixelData = Vec<(u8, u8, u8, u8)>;

pub struct Keys {
    pub w: AtomicBool,
    pub a: AtomicBool,
    pub s: AtomicBool,
    pub d: AtomicBool,
}

/// GUI for game
pub struct App {
    /// Main window object
    window: Arc<RwLock<Option<Arc<Window>>>>,
    /// Screen for representing the game inside the window
    screen: Option<Screen<'static>>,
    /// Pixel colors provided by Renderer
    pixel_data: Arc<RwLock<PixelData>>,
    /// currently pressed key for Engine
    //pub(crate) key_pressed: Arc<RwLock<Option<KeyCode>>>,
    pub(crate) keys_pressed: Arc<Keys>,

    frame_count: u32,
    last_fps_report_time: Instant,
}

/// Basic App implementation
impl App {
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
    pub fn run(&mut self) {}
}

impl ApplicationHandler for App {
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

    /// Treats window events correspondingly:
    /// 1. CloseRequest -- stop the application
    /// 2. RedrawRequest -- take new data from producer (eg. Renderer), update the screen. Also counts FPS (probably delete later)
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
                //println!("pressed {:?}", key_code);
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

/// To run example, replace /bin/main.rs with:
/// rust
///use rusty_ache::screen::utils::example;
///
///fn main() {
///    example();
///}
///
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

/*pub fn example_keys() {
    let initial_resolution = Resolution {
        width: WIDTH,
        height: HEIGHT,
    };
    const DEFAULT_COLOR: (u8, u8, u8, u8) = (0xF5, 0xDE, 0xB3, 0xFF);
    const RED: (u8, u8, u8, u8) = (0xFF, 0x00, 0x00, 0xFF);
    const BLUE: (u8, u8, u8, u8) = (0x00, 0x00, 0xFF, 0xFF);
    const GREEN: (u8, u8, u8, u8) = (0x00, 0xFF, 0x00, 0xFF);
    const PURPLE: (u8, u8, u8, u8) = (0x80, 0x00, 0x80, 0xFF);
    let initial_pixels =
        vec![DEFAULT_COLOR; (initial_resolution.width * initial_resolution.height) as usize];

    let shared_pixel_data = Arc::new(RwLock::new(initial_pixels));
    let shared_window = Arc::new(RwLock::new(None));

    let shared_pixel_data_clone = shared_pixel_data.clone();
    let shared_window_clone = shared_window.clone();

    let mut app = App::new(shared_pixel_data, shared_window);
    //let key_pressed_clone = app.key_pressed.clone();
    let keys_pressed_clone = app.keys_pressed.clone();
    // Producer thread
    thread::spawn(move || {
        let window_arc: Arc<Window> = loop {
            if let Some(arc) = shared_window_clone.read().unwrap().clone() {
                break arc;
            }
            thread::sleep(Duration::from_millis(100));
        };

        //dbg!("Producer has started");

        let screen_size = (WIDTH * HEIGHT) as usize;
        loop {
            let cur_keys_state = keys_pressed_clone.read().unwrap();
            let color = if cur_keys_state.contains(&KeyCode::KeyW) {
                RED
            } else if cur_keys_state.contains(&KeyCode::KeyA) {
                BLUE
            }
            else if cur_keys_state.contains(&KeyCode::KeyS) {
                GREEN
            }
            else if cur_keys_state.contains(&KeyCode::KeyD) {
                PURPLE
            }
            else {
                DEFAULT_COLOR
            };
            {
                let mut pixels = shared_pixel_data_clone
                    .write()
                    .expect("Producer couldn't lock pixel data");

                for p in pixels.iter_mut().take(screen_size) {
                    *p = color;
                }
            }
            window_arc.request_redraw();
        }
    });

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let _ = event_loop.run_app(&mut app);
}*/
