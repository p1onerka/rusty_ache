//! Manages different scenes within the game engine, allowing for scene switching and initialization.
//!
//! The `SceneManager` struct holds the currently active scene and provides methods to access
//! and initialize its renderable components for rendering purposes.

use crate::engine::scene::Scene;
use crate::engine::scene::game_object::GameObject;
use image::DynamicImage;

/// Manages active scene and provides scene-related operations.
pub struct SceneManager {
    /// The scene currently active in the engine.
    pub(crate) active_scene: Scene,
    pub(crate) end_scene: EndScene,
}

impl SceneManager {
    /// Creates a new `SceneManager` with the specified main scene.
    ///
    /// # Parameters
    /// - `main_scene`: The initial scene to set as active.
    ///
    /// # Returns
    /// A new `SceneManager` instance with the provided scene.
    pub fn new(main_scene: Scene, end_scene: EndScene) -> Self {
        SceneManager {
            active_scene: main_scene,
            end_scene: end_scene,
        }
    }

    /// Returns a reference to the currently active scene.
    ///
    /// # Returns
    /// A reference to the active `Scene`.
    pub fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    /// Returns a mut reference to the currently active scene.
    ///
    /// # Returns
    /// A mut reference to the active `Scene`.
    pub fn ref_mut_by_uid(&mut self, uid: usize) -> &mut GameObject {
        self.active_scene.ref_mut_by_uid(uid)
    }

    /// Initializes and retrieves all renderable objects from the active scene.
    ///
    /// This method calls the `init()` method of the current scene, which prepares
    /// sprite components sorted by their z-position for rendering.
    ///
    /// # Returns
    /// A vector of tuples, each containing references to game objects,
    /// their sprite images, positional offsets, and shadow flags.
    pub fn init_active_scene(&self) -> Vec<(usize, &GameObject, &DynamicImage, (i32, i32), bool)> {
        self.active_scene.init()
    }
}

#[derive(Clone)]
pub struct EndScene {
    pub(crate) background: Option<DynamicImage>,
    pub(crate) timeout_ms: Option<u64>,
}

impl EndScene {
    pub fn new(background: Option<DynamicImage>, timeout_ms: Option<u64>) -> Self {
        EndScene {
            background,
            timeout_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::scene::game_object::{
        Object, Position,
        components::{Component, sprite::Sprite},
    };

    use super::*;

    fn _create_test_position(x: i32, y: i32, z: i32, is_relative: bool) -> Position {
        Position {
            x,
            y,
            z,
            is_relative,
        }
    }

    fn _create_test_components() -> Vec<Box<dyn Component + Send + Sync>> {
        vec![Box::new(Sprite::new(None, false, (0, 0)))]
    }

    fn _create_simple_scene() -> Scene {
        Scene::new(
            vec![],
            _create_test_components(),
            _create_test_position(0, 0, 0, false),
            &mut vec![],
        )
    }

    fn _create_scene_with_sprites(sprite_count: usize) -> Scene {
        let mut objects = vec![];

        for i in 0..sprite_count {
            let obj = GameObject::new(
                _create_test_components(),
                None,
                _create_test_position(i as i32, i as i32, i as i32, false),
            );
            objects.push(obj);
        }

        Scene::new(
            objects,
            vec![],
            _create_test_position(0, 0, 0, false),
            &mut vec![],
        )
    }
}

// #[test]
// fn test_init_active_scene_returns_sprites() {
//     let scene = create_scene_with_sprites(1);
//     let manager = SceneManager::new(scene);

//     let renderable = manager.init_active_scene();

//     assert_eq!(renderable.len(), 1);
// }

// #[test]
// fn test_scene_manager_with_scene_containing_objects_without_sprites() {
//     let obj1 = GameObject::new(
//         create_test_components(),
//         create_test_position(0, 0, 0, false),
//     );
//     let obj2 = GameObject::new(
//         create_test_components(),
//         create_test_position(10, 10, 10, false),
//     );

//     let scene = Scene::new(
//         vec![obj1, obj2],
//         vec![],
//         create_test_position(0, 0, 0, false),
//     );
//     let manager = SceneManager::new(scene);

//     let renderable = manager.init_active_scene();

//     assert_eq!(renderable.len(), 0);
// }
