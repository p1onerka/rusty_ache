use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

pub const WIDTH: u32 = 200;
pub const HEIGHT: u32 = 200;

/// Struct representing screen resolution.
#[derive(Copy, Clone)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

/// Struct representing Screen, on which a game will be drawn
pub struct Screen<'a> {
    pixels: Pixels<'a>,
}

/// Implementation of Screen for main GUI application
impl<'a> Screen<'a> {
    pub fn new(window: Arc<Window>, resolution: Resolution) -> Result<Self, pixels::Error> {
        let surface_texture =
            SurfaceTexture::new(resolution.width, resolution.height, window.clone());
        let pixels = Pixels::new(resolution.width, resolution.height, surface_texture)?;
        Ok(Self { pixels })
    }

    /*/// Draw new frame on Screen
    pub fn update(&mut self, pixel_colors: &[u8]) {
        let cur_frame = self.pixels.frame_mut();
        cur_frame.copy_from_slice(pixel_colors);
        let _ = self.pixels.render();
    }*/

    /// Draw new RGB frame on Screen
    pub fn update(&mut self, pixel_colors: &[u32]) {
        let cur_frame = self.pixels.frame_mut();
        for (i, &color) in pixel_colors.iter().enumerate() {
            let base = i * 4;
            cur_frame[base] = ((color >> 16) & 0xFF) as u8; // red
            cur_frame[base + 1] = ((color >> 8) & 0xFF) as u8; // green
            cur_frame[base + 2] = (color & 0xFF) as u8; // blue
            cur_frame[base + 3] = 0xFF;
        }
        let _ = self.pixels.render();
    }
}

/// GUI for game
pub struct App {
    /// Main window object
    window: Arc<RwLock<Option<Arc<Window>>>>,
    /// Screen for representing the game inside the window
    screen: Option<Screen<'static>>,
    /// Pixel colors provided by Renderer
    pixel_data: Arc<RwLock<Vec<u32>>>,

    /// Currrent FPS accumulator (probably delete later)
    frame_count: u32,
    /// For FPS compitation (probably delete later)
    last_fps_report_time: Instant,
}

/// Basic App implementation
impl App {
    pub fn new(
        pixel_data: Arc<RwLock<Vec<u32>>>,
        window: Arc<RwLock<Option<Arc<Window>>>>,
    ) -> Self {
        App {
            screen: None,
            pixel_data,
            window,

            frame_count: 0,
            last_fps_report_time: Instant::now(),
        }
    }
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
                    //let fps = self.frame_count as f64 / elapsed.as_secs_f64();
                    //if let Some(window_arc) = self.window.read().unwrap().as_ref() {
                    //     window_arc.set_title(&format!("FPS: {:.2}", fps));
                    //}
                    //dbg!(fps);
                    self.frame_count = 0;
                    self.last_fps_report_time = Instant::now();
                }
            }
            _ => (),
        }
    }
}

/// To run example, replace /bin/main.rs with:
/// ```rust
///use rusty_ache::screen::utils::example;
///
///fn main() {
///    example();
///}
/// ```
pub fn example() {
    let initial_resolution = Resolution {
        width: WIDTH,
        height: HEIGHT,
    };
    let initial_pixels =
        vec![0x000000u32; (initial_resolution.width * initial_resolution.height) as usize];
        
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
            {
                let mut pixels = shared_pixel_data_clone
                    .write()
                    .expect("Producer couldn't get lock to write new pixel data into App");

                for i in 0..screen_size {
                    pixels[i] = new_color; // recolor all pixels into new color
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
