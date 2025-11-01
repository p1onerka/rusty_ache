//! Represents a game scene containing multiple game objects and managing rendering order.
//!
//! The `Scene` struct manages a collection of game objects via a `GameObjectManager`
//! and holds a reference to the main game object. It allows initializing a list
//! of renderable entities sorted by z-position for rendering purposes.
//!
//! This module abstracts the coordination of game objects and prepares sprite data
//! for the rendering pipeline.

use crate::engine::scene::game_object::Object;
use crate::engine::scene::game_object::components::{Component, ComponentType};
use crate::engine::scene::game_object::{GameObject, Position};
use crate::engine::scene::object_manager::GameObjectManager;
use image::DynamicImage;

pub mod game_object;

mod object_manager;

/// Represents the game scene containing game objects and main entity.
pub struct Scene {
    /// Manager responsible for storing and controlling multiple game objects.
    manager: GameObjectManager,
    /// The main game object within this scene.
    pub main_object: GameObject,
}

impl Scene {
    /// Creates a new `Scene` instance with provided game objects, main object's components, and position.
    ///
    /// # Parameters
    /// - `objects`: Vector of existing game objects to manage in this scene.
    /// - `main_components`: Components of the main game object.
    /// - `main_position`: Position of the main game object.
    ///
    /// # Returns
    /// A new scene instance managing game objects and main entity.
    pub fn new(
        objects: Vec<GameObject>,
        main_components: Vec<Box<dyn Component + Send + Sync>>,
        main_position: Position,
    ) -> Self {
        let mut obj_manager = GameObjectManager::new(256);
        for obj in objects {
            obj_manager.add_game_object(obj.components, obj.position)
        }
        Scene {
            manager: obj_manager,
            main_object: GameObject::new(main_components, None, main_position),
        }
    }

    /// Initializes and collects all renderable sprite objects in the scene.
    ///
    /// Returns a vector of tuples containing references to game objects and their
    /// sprite images, positional offsets, and shadow flags. The returned vector
    /// is sorted by the `z` value of the game object's position to maintain correct rendering order.
    pub fn init(&self) -> Vec<(&GameObject, &DynamicImage, (i32, i32), bool)> {
        let mut renderable_objects: Vec<(&GameObject, &DynamicImage, (i32, i32), bool)> = vec![];
        for obj in self.manager.game_objects.values() {
            for component in obj.components.iter() {
                if component.get_component_type() == ComponentType::Sprite {
                    /*match &component.get_shadow_unchecked() {
                        None => {}
                        Some(img) => {
                            renderable_objects.push((
                                obj,
                                &img.0,
                                (
                                    component.get_sprite_offset_unchecked().unwrap().0 + img.1.0,
                                    component.get_sprite_offset_unchecked().unwrap().1 + img.1.1,
                                ),
                            ));
                        }
                    };*/
                    renderable_objects.push((
                        obj,
                        component.get_sprite_unchecked().as_ref().unwrap(),
                        component.get_sprite_offset_unchecked().unwrap(),
                        component.get_shadow_unchecked(),
                    ));
                }
            }
        }
        renderable_objects.sort_by(|a, b| a.0.position.z.cmp(&b.0.position.z));

        for component in self.main_object.components.iter() {
            if component.get_component_type() == ComponentType::Sprite {
                if let Some(sprite_img) = component.get_sprite_unchecked().as_ref() {
                    renderable_objects.push((
                        &self.main_object,
                        sprite_img,
                        component.get_sprite_offset_unchecked().unwrap_or((0, 0)),
                        component.get_shadow_unchecked(),
                    ));
                }
            }
        }

        renderable_objects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scene_with_empty_objects_and_main() {
        let scene = Scene::new(
            vec![],
            vec![],
            Position {
                x: 1,
                y: 2,
                z: 3,
                is_relative: false,
            },
        );
        assert_eq!(scene.manager.game_objects.len(), 0);
        assert_eq!(scene.main_object.components.len(), 0);
        assert_eq!(scene.main_object.position.x, 1);
        assert_eq!(scene.main_object.position.y, 2);
        assert_eq!(scene.main_object.position.z, 3);
    }

    #[test]
    fn test_new_scene_with_multiple_objects() {
        let obj1 = GameObject::new(
            vec![],
            None,
            Position {
                x: 5,
                y: 5,
                z: 0,
                is_relative: false,
            },
        );
        let obj2 = GameObject::new(
            vec![],
            None,
            Position {
                x: 7,
                y: 8,
                z: 1,
                is_relative: false,
            },
        );

        let scene = Scene::new(
            vec![obj1, obj2],
            vec![],
            Position {
                x: 0,
                y: 0,
                z: 0,
                is_relative: false,
            },
        );
        assert_eq!(scene.manager.game_objects.len(), 2);
    }

    #[test]
    fn test_scene_manager_handles_main_object_components() {
        let scene = Scene::new(
            vec![],
            vec![],
            Position {
                x: 2,
                y: 2,
                z: 2,
                is_relative: false,
            },
        );
        assert_eq!(scene.main_object.components.len(), 0);
    }

    #[test]
    fn test_init_returns_empty_when_no_sprite_components() {
        let obj = GameObject::new(
            vec![],
            None,
            Position {
                x: 1,
                y: 2,
                z: 3,
                is_relative: false,
            },
        );
        let scene = Scene::new(
            vec![obj],
            vec![],
            Position {
                x: 0,
                y: 0,
                z: 0,
                is_relative: false,
            },
        );
        let result = scene.init();
        assert_eq!(result.len(), 0);
    }
}
