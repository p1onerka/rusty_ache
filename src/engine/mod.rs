//! Main engine module defining engine traits and implementation.
//!
//! Provides abstractions for engine lifecycle and rendering, as well as
//! a concrete `GameEngine` implementation which manages rendering, scene management,
//! input handling, and the event loop.
//!
//! This module integrates configurations, scenes, rendering, and input processing
//! to provide the core game engine loop and functionality.

pub mod config;
pub mod input;
pub mod scene;
pub mod scene_manager;
pub mod scripts;

use crate::engine::config::Config;
use crate::engine::scene::Scene;
use crate::engine::scene::game_object::{Object, Position};
use crate::engine::scene_manager::{EndScene, SceneManager};
use crate::engine::scripts::main_obj_script;
use crate::interface::{
    ObjectWithImage, create_gameobj_vec, create_obj_with_img, init_end_scene, init_scene,
};
use crate::render::renderer::{DEFAULT_BACKGROUND_COLOR, Renderer};
use crate::screen::{App, HEIGHT, WIDTH};
use crate::{Resolution, engine};
// use crate::end_scene::EndScene;
//use image::ImageReader;
use image::{DynamicImage, ImageReader};
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::{thread, vec};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub const EMPTY: &'static str = "src/bin/resources/empty.png";

/// Trait defining essential engine behavior.
///
/// Abstracts an engine capable of managing an active scene, performing rendering,
/// running its main loop, and supporting dynamic configuration.
pub trait Engine {
    /// Sets the currently active scene within the engine.
    fn set_active_scene(&mut self, new_scene: Scene, end_scene: EndScene) -> Result<(), Error>;

    /// Performs a rendering pass.
    fn render(&mut self) -> Result<(), Error>;

    /// Starts and runs the engine main loop.
    fn run(&mut self) -> Result<(), Error>;

    /// Creates a new engine instance from configuration and initial scene.
    fn new(config: Box<dyn Config + Send>, scene: Scene, end_scene: EndScene) -> Self
    where
        Self: Sized;
}

/// Concrete implementation of the game engine.
///
/// Holds a thread-safe renderer reference, manages scenes and input handling,
/// runs the main event loop and coordinates rendering.
pub struct GameEngine {
    //config: Box<dyn Config + Send>,
    render: Arc<RwLock<Renderer>>,
    pub main_pos: Arc<RwLock<(i32, i32)>>,
    pub is_end_scene_active: Arc<AtomicBool>,
}

impl Engine for GameEngine {
    /// Sets the active scene inside the renderer's scene manager.
    fn set_active_scene(&mut self, new_scene: Scene, end_scene: EndScene) -> Result<(), Error> {
        self.render.write().unwrap().scene_manager = SceneManager::new(new_scene, end_scene);

        Ok(())
    }

    /// Delegates rendering to the internal Renderer instance.
    fn render(&mut self) -> Result<(), Error> {
        self.render.write().unwrap().render();
        Ok(())
    }

    /// Creates a new GameEngine using provided config and scene.
    ///
    /// Initializes the Renderer with the resolution and the scene manager.
    fn new(config: Box<dyn Config + 'static + Send>, scene: Scene, end_scene: EndScene) -> Self
    where
        Self: Sized,
    {
        let res = config.get_resolution();
        let main_obj_x = scene.main_object.position.x;
        let main_obj_y = scene.main_object.position.y;
        GameEngine {
            //config,
            render: Arc::new(RwLock::from(Renderer::new(
                res,
                /*Some(ImageReader::open("src/bin/resources/tile2.png")
                .unwrap()
                .decode()
                .unwrap())*/
                None,
                SceneManager::new(scene, end_scene),
            ))),
            main_pos: Arc::new(RwLock::from((main_obj_x, main_obj_y))),
            is_end_scene_active: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Runs the game engine event loop.
    ///
    /// Sets up shared state for pixel data, window, and input keys.
    /// Spawns a producer thread that updates the main object's position based on key input
    /// and triggers rendering updates.
    /// Runs the `winit` event loop with the associated GUI application.
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
        let main_pos_arc = self.main_pos.clone();

        let is_end_scene_active = self.is_end_scene_active.clone();
        let start_scene = self
            .render
            .read()
            .unwrap()
            .scene_manager
            .active_scene
            .clone();
        let end_scene = self
            .render
            .read()
            .unwrap()
            .scene_manager
            .end_scene
            .scene
            .clone();
        let new_background = renderer
            .read()
            .unwrap()
            .scene_manager
            .end_scene
            .background
            .clone();

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
                if is_end_scene_active.load(Ordering::SeqCst) {
                    let prev_background = renderer
                        .write()
                        .unwrap()
                        .set_background(new_background.clone());
                    let empty_object = create_obj_with_img(EMPTY, 0, 0, false);
                    let empty_object2 = create_obj_with_img(EMPTY, 0, 0, false);
                    let scene = init_scene(&[], empty_object);
                    let scene2 = init_scene(&[], empty_object2);
                    let timeout_ms = renderer.read().unwrap().scene_manager.end_scene.timeout_ms;
                    renderer.write().unwrap().scene_manager = SceneManager::new(
                        scene,
                        EndScene::new(scene2, new_background.clone(), timeout_ms),
                    );

                    if timeout_ms.is_none() {
                        loop {
                            renderer.write().unwrap().render();
                            match renderer.write().unwrap().emit() {
                                Some(colors) => {
                                    let mut pixels = shared_pixel_data_clone
                                        .write()
                                        .expect("Producer couldn't lock pixel data");

                                    for (idx, p) in pixels.iter_mut().take(screen_size).enumerate()
                                    {
                                        *p = colors[idx];
                                    }

                                    window_arc.request_redraw();
                                }
                                None => {
                                    continue;
                                }
                            }
                        }
                    } else {
                        // self.render().unwrap();
                        let pause_until =
                            Instant::now() + Duration::from_millis(timeout_ms.unwrap());
                        loop {
                            renderer.write().unwrap().render();
                            match renderer.write().unwrap().emit() {
                                Some(colors) => {
                                    let mut pixels = shared_pixel_data_clone
                                        .write()
                                        .expect("Producer couldn't lock pixel data");

                                    for (idx, p) in pixels.iter_mut().take(screen_size).enumerate()
                                    {
                                        *p = colors[idx];
                                    }

                                    window_arc.request_redraw();
                                }
                                None => {
                                    continue;
                                }
                            }
                            // self.render().unwrap();
                            if Instant::now() >= pause_until {
                                break;
                            }
                        }
                    }
                    // renderer.write().unwrap().scene_manager = SceneManager::new(
                    //     start_scene.clone(),
                    //     renderer.read().unwrap().scene_manager.end_scene.clone(),
                    // );
                    renderer
                        .write()
                        .unwrap()
                        .set_background(prev_background)
                        .unwrap();
                    is_end_scene_active.store(true, std::sync::atomic::Ordering::SeqCst);
                }
                /*let vector_move = match *key_pressed_clone.read().unwrap() {
                    Some(KeyCode::KeyW) => (0, 1),
                    Some(KeyCode::KeyA) => (-1, 0),
                    Some(KeyCode::KeyS) => (0, -1),
                    Some(KeyCode::KeyD) => (1, 0),
                    _ => (0, 0),
                };*/
                let dx_keys = (keys_pressed_clone.d.load(Ordering::Relaxed) as i32)
                    - (keys_pressed_clone.a.load(Ordering::Relaxed) as i32);
                let dy_keys = (keys_pressed_clone.w.load(Ordering::Relaxed) as i32)
                    - (keys_pressed_clone.s.load(Ordering::Relaxed) as i32);

                let (dx_script, dy_script) = main_obj_script();
                let vector_move = (dx_keys + dx_script, dy_keys + dy_script);

                renderer
                    .write()
                    .unwrap()
                    .scene_manager
                    .active_scene
                    .main_object
                    .add_position((vector_move.0, vector_move.1));

                {
                    let pos = renderer
                        .read()
                        .unwrap()
                        .scene_manager
                        .active_scene
                        .main_object
                        .position;

                    *main_pos_arc.write().unwrap() = (pos.x, pos.y);
                }

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

#[cfg(test)]
mod tests {
    use crate::{
        Resolution,
        engine::{config::EngineConfig, scene::game_object::Position},
    };

    use super::*;

    fn create_config_with_resolution(
        width: u32,
        height: u32,
    ) -> Box<dyn config::Config + Send + 'static> {
        Box::new(EngineConfig::new(Resolution::new(width, height)))
    }

    fn create_empty_scene() -> Scene {
        Scene::new(
            vec![],
            vec![],
            Position {
                x: 0,
                y: 0,
                z: 0,
                is_relative: false,
            },
        )
    }
}
