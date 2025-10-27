pub mod config;
pub mod input;
pub mod scene;
pub mod scene_manager;

use crate::Resolution;
use crate::engine::config::{Config, EngineConfig};
use crate::engine::scene::Scene;
use crate::engine::scene_manager::SceneManager;
use crate::render::renderer::Renderer;
use crate::screen::{App, HEIGHT, Screen, WIDTH};
use std::io::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::Window;

/// A trait for describing entity for main engine logic
pub trait Engine {
    fn set_active_scene(&mut self, new_scene: Scene) -> Result<(), Error>;
    fn render(&mut self) -> Result<(), Error>;

    fn run(&mut self) -> Result<(), Error>;

    fn new(config: Box<dyn Config + Send>, scene: Scene) -> Self
    where
        Self: Sized;
}

pub struct GameEngine {
    config: Box<dyn Config + Send>,
    render: Arc<RwLock<Renderer>>,
}

impl Engine for GameEngine {
    fn set_active_scene(&mut self, new_scene: Scene) -> Result<(), Error> {
        self.render.write().unwrap().scene_manager = SceneManager::new(new_scene);

        // Can return Err if scene isn't found. Not implemented.
        Ok(())
    }

    fn render(&mut self) -> Result<(), Error> {
        self.render.write().unwrap().render();
        Ok(())
    }

    fn new(config: Box<(dyn Config + 'static + Send)>, scene: Scene) -> Self
    where
        Self: Sized,
    {
        let res = config.get_resolution();
        GameEngine {
            config,
            render: Arc::new(RwLock::from(Renderer::new(
                res,
                None,
                SceneManager::new(scene),
            ))),
        }
    }

    fn run(&mut self) -> Result<(), Error> {
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
        let key_pressed_clone = app.key_pressed.clone();
        let renderer = self.render.clone();
        // Producer thread
        thread::spawn(move || {
            let window_arc: Arc<Window> = loop {
                if let Some(arc) = shared_window_clone.read().unwrap().clone() {
                    break arc;
                }
                thread::sleep(Duration::from_millis(50));
            };

            //dbg!("Producer has started");

            let screen_size = (WIDTH * HEIGHT) as usize;
            loop {
                let vector_move = match *key_pressed_clone.read().unwrap() {
                    Some(KeyCode::KeyW) => (0, 1),
                    Some(KeyCode::KeyA) => (-1, 0),
                    Some(KeyCode::KeyS) => (0, -1),
                    Some(KeyCode::KeyD) => (1, 0),
                    _ => (0, 0),
                };
                println!("{:?}", vector_move);
                renderer
                    .write()
                    .unwrap()
                    .scene_manager
                    .active_scene
                    .main_object
                    .add_position((vector_move.0, vector_move.1));
                //renderer.clear_poison();
                renderer.write().unwrap().render();
                match renderer.write().unwrap().emit() {
                    Some(colors) => {
                        let mut pixels = shared_pixel_data_clone
                            .write()
                            .expect("Producer couldn't lock pixel data");

                        for (idx, p) in pixels.iter_mut().take(screen_size).enumerate() {
                            *p = colors[idx].clone();
                        }

                        window_arc.request_redraw();
                    }
                    None => {
                        //println!("Emition failed");
                        continue;
                    }
                }
            }
        });

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);

        let _ = event_loop.run_app(&mut app);
        Ok(())
    }
}
