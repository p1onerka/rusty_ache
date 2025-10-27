pub mod config;
pub mod input;
pub mod scene;
pub mod scene_manager;

use crate::engine::config::{Config, EngineConfig};
use crate::engine::scene::Scene;
use crate::engine::scene_manager::SceneManager;
use crate::render::renderer::Renderer;
use std::io::Error;

/// A trait for describing entity for main engine logic
pub trait Engine {
    fn set_active_scene(&mut self, new_scene: Scene) -> Result<(), Error>;
    fn render(&mut self) -> Result<(), Error>;

    fn new(config: Box<dyn Config>, scene: Scene) -> Self
    where
        Self: Sized;
}

pub struct GameEngine {
    config: Box<dyn Config>,
    render: Renderer,
}

impl Engine for GameEngine {
    fn set_active_scene(&mut self, new_scene: Scene) -> Result<(), Error> {
        self.render.scene_manager = SceneManager::new(new_scene);

        // Can return Err if scene isn't found. Not implemented.
        Ok(())
    }

    fn render(&mut self) -> Result<(), Error> {
        self.render.render();
        Ok(())
    }

    fn new(config: Box<(dyn Config + 'static)>, scene: Scene) -> Self
    where
        Self: Sized,
    {
        let res = config.get_resolution();
        GameEngine {
            config,
            render: Renderer::new(res, None, SceneManager::new(scene)),
        }
    }
}
