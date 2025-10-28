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

#[cfg(test)]
mod tests {
    use crate::{Resolution, engine::scene::game_object::{Position, components::Component}};

    use super::*;

    fn create_config_with_resolution(width: u32, height: u32) -> Box<dyn Config> {
        Box::new(EngineConfig::new(Resolution::new(width, height)))
    }

    fn create_empty_scene() -> Scene {
        Scene::new(vec![], vec![], Position { x: 0, y: 0, z: 0, is_relative: false })
    }

    #[test]
    fn test_new_engine_creates_with_resolution() {
        let config = create_config_with_resolution(1024, 768);
        let scene = create_empty_scene();
        let engine = GameEngine::new(config, scene);
        assert_eq!(engine.config.get_resolution().width, 1024);
        assert_eq!(engine.config.get_resolution().height, 768);
        assert_eq!(engine.render.scene_manager.active_scene.main_object.position.x, 0);
    }

    #[test]
    fn test_render_multiple_calls_return_ok() {
        let config = create_config_with_resolution(800, 600);
        let scene = create_empty_scene();
        let mut engine = GameEngine::new(config, scene);
        for _ in 0..5 {
            assert!(engine.render().is_ok());
        }
    }

    #[test]
    fn test_set_active_scene_returns_ok_and_replaces_scene() {
        let config = create_config_with_resolution(800, 600);
        let scene1 = create_empty_scene();
        let mut engine = GameEngine::new(config, scene1);

        let scene2 = Scene::new(
            vec![],
            vec![],
            Position { x: 100, y: 100, z: 100, is_relative: false },
        );
        let result = engine.set_active_scene(scene2);
        assert!(result.is_ok());
        let pos = engine.render.scene_manager.active_scene.main_object.position;
        assert_eq!(pos.x, 100);
        assert_eq!(pos.y, 100);
        assert_eq!(pos.z, 100);
    }

    #[test]
    fn test_set_active_scene_multiple_times() {
        let config = create_config_with_resolution(800, 600);
        let scene1 = create_empty_scene();
        let mut engine = GameEngine::new(config, scene1);

        for i in 0..10 {
            let scene = Scene::new(vec![], vec![], Position { x: i , y: i , z: i, is_relative: false });
            assert!(engine.set_active_scene(scene).is_ok());
            let pos = engine.render.scene_manager.active_scene.main_object.position;
            assert_eq!(pos.x, i );
        }
    }

    #[test]
    fn test_new_engine_with_scene_with_main_components() {
        // Здесь можете подставить настоящие компоненты из вашего проекта
        let main_components: Vec<Box<dyn Component>> = vec![];
        let scene = Scene::new(
            vec![],
            main_components,
            Position { x: 5, y: 6, z: 7, is_relative: false },
        );
        let config = create_config_with_resolution(1280, 720);
        let engine = GameEngine::new(config, scene);
        assert_eq!(engine.render.scene_manager.active_scene.main_object.position.x, 5);
        assert_eq!(engine.render.scene_manager.active_scene.main_object.components.len(), 0);
    }

    #[test]
    fn test_render_after_setting_new_active_scene() {
        let config = create_config_with_resolution(640, 480);
        let scene1 = create_empty_scene();
        let mut engine = GameEngine::new(config, scene1);

        let scene2 = Scene::new(
            vec![],
            vec![],
            Position { x: 15, y: 15, z: 15, is_relative: false },
        );
        engine.set_active_scene(scene2).unwrap();

        for _ in 0..3 {
            assert!(engine.render().is_ok());
        }
    }
}
