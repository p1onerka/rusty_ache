pub mod config;
pub mod input;
pub mod scene;
pub mod scene_manager;

use crate::Resolution;
use crate::engine::config::Config;
use crate::engine::scene::Scene;
use crate::engine::scene::game_object::Object;
use crate::engine::scene_manager::SceneManager;
use crate::render::renderer::DEFAULT_BACKGROUND_COLOR;
use crate::render::renderer::Renderer;
use crate::screen::{App, HEIGHT, WIDTH};
use std::io::Error;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use winit::event_loop::{ControlFlow, EventLoop};
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
    //config: Box<dyn Config + Send>,
    render: Arc<RwLock<Renderer>>,
}

impl Engine for GameEngine {
    fn set_active_scene(&mut self, new_scene: Scene) -> Result<(), Error> {
        self.render.write().unwrap().scene_manager = SceneManager::new(new_scene);

        Ok(())
    }

    fn render(&mut self) -> Result<(), Error> {
        self.render.write().unwrap().render();
        Ok(())
    }

    fn new(config: Box<dyn Config + 'static + Send>, scene: Scene) -> Self
    where
        Self: Sized,
    {
        let res = config.get_resolution();
        GameEngine {
            //config,
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
        let initial_pixels = vec![
            DEFAULT_BACKGROUND_COLOR;
            (initial_resolution.width * initial_resolution.height) as usize
        ];

        let shared_pixel_data = Arc::new(RwLock::new(initial_pixels));
        let shared_window = Arc::new(RwLock::new(None));

        let shared_pixel_data_clone = shared_pixel_data.clone();
        let shared_window_clone = shared_window.clone();

        let mut app = App::new(shared_pixel_data, shared_window);
        //let key_pressed_clone = app.key_pressed.clone();
        let keys_pressed_clone = app.keys_pressed.clone();
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
                /*let vector_move = match *key_pressed_clone.read().unwrap() {
                    Some(KeyCode::KeyW) => (0, 1),
                    Some(KeyCode::KeyA) => (-1, 0),
                    Some(KeyCode::KeyS) => (0, -1),
                    Some(KeyCode::KeyD) => (1, 0),
                    _ => (0, 0),
                };*/
                let dx = (keys_pressed_clone.d.load(Ordering::Relaxed) as i32)
                    - (keys_pressed_clone.a.load(Ordering::Relaxed) as i32);
                let dy = (keys_pressed_clone.w.load(Ordering::Relaxed) as i32)
                    - (keys_pressed_clone.s.load(Ordering::Relaxed) as i32);

                let vector_move = (dx, dy);
                //println!("{:?}", vector_move);
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
                            *p = colors[idx];
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

// #[cfg(test)]
// mod tests {
//     use crate::{
//         Resolution,
//         engine::scene::game_object::{Position, components::Component},
//     };

//     use super::*;

//     fn create_config_with_resolution(width: u32, height: u32) -> Box<dyn Config> {
//         Box::new(EngineConfig::new(Resolution::new(width, height)))
//     }

//     fn create_empty_scene() -> Scene {
//         Scene::new(
//             vec![],
//             vec![],
//             Position {
//                 x: 0,
//                 y: 0,
//                 z: 0,
//                 is_relative: false,
//             },
//         )
//     }

//     #[test]
//     fn test_new_engine_creates_with_resolution() {
//         let config = create_config_with_resolution(1024, 768);
//         let scene = create_empty_scene();
//         let engine = GameEngine::new(config, scene);
//         assert_eq!(engine.config.get_resolution().width, 1024);
//         assert_eq!(engine.config.get_resolution().height, 768);
//         assert_eq!(
//             engine
//                 .render
//                 .scene_manager
//                 .active_scene
//                 .main_object
//                 .position
//                 .x,
//             0
//         );
//     }

//     #[test]
//     fn test_render_multiple_calls_return_ok() {
//         let config = create_config_with_resolution(800, 600);
//         let scene = create_empty_scene();
//         let mut engine = GameEngine::new(config, scene);
//         for _ in 0..5 {
//             assert!(engine.render().is_ok());
//         }
//     }

//     #[test]
//     fn test_set_active_scene_returns_ok_and_replaces_scene() {
//         let config = create_config_with_resolution(800, 600);
//         let scene1 = create_empty_scene();
//         let mut engine = GameEngine::new(config, scene1);

//         let scene2 = Scene::new(
//             vec![],
//             vec![],
//             Position {
//                 x: 100,
//                 y: 100,
//                 z: 100,
//                 is_relative: false,
//             },
//         );
//         let result = engine.set_active_scene(scene2);
//         assert!(result.is_ok());
//         let pos = engine
//             .render
//             .scene_manager
//             .active_scene
//             .main_object
//             .position;
//         assert_eq!(pos.x, 100);
//         assert_eq!(pos.y, 100);
//         assert_eq!(pos.z, 100);
//     }

//     #[test]
//     fn test_set_active_scene_multiple_times() {
//         let config = create_config_with_resolution(800, 600);
//         let scene1 = create_empty_scene();
//         let mut engine = GameEngine::new(config, scene1);

//         for i in 0..10 {
//             let scene = Scene::new(
//                 vec![],
//                 vec![],
//                 Position {
//                     x: i,
//                     y: i,
//                     z: i,
//                     is_relative: false,
//                 },
//             );
//             assert!(engine.set_active_scene(scene).is_ok());
//             let pos = engine
//                 .render
//                 .scene_manager
//                 .active_scene
//                 .main_object
//                 .position;
//             assert_eq!(pos.x, i);
//         }
//     }

//     #[test]
//     fn test_new_engine_with_scene_with_main_components() {
//         // Здесь можете подставить настоящие компоненты из вашего проекта
//         let main_components: Vec<Box<dyn Component>> = vec![];
//         let scene = Scene::new(
//             vec![],
//             main_components,
//             Position {
//                 x: 5,
//                 y: 6,
//                 z: 7,
//                 is_relative: false,
//             },
//         );
//         let config = create_config_with_resolution(1280, 720);
//         let engine = GameEngine::new(config, scene);
//         assert_eq!(
//             engine
//                 .render
//                 .scene_manager
//                 .active_scene
//                 .main_object
//                 .position
//                 .x,
//             5
//         );
//         assert_eq!(
//             engine
//                 .render
//                 .scene_manager
//                 .active_scene
//                 .main_object
//                 .components
//                 .len(),
//             0
//         );
//     }

//     #[test]
//     fn test_render_after_setting_new_active_scene() {
//         let config = create_config_with_resolution(640, 480);
//         let scene1 = create_empty_scene();
//         let mut engine = GameEngine::new(config, scene1);

//         let scene2 = Scene::new(
//             vec![],
//             vec![],
//             Position {
//                 x: 15,
//                 y: 15,
//                 z: 15,
//                 is_relative: false,
//             },
//         );
//         engine.set_active_scene(scene2).unwrap();

//         for _ in 0..3 {
//             assert!(engine.render().is_ok());
//         }
//     }
// }
