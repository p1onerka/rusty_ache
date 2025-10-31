use crate::engine::scene::game_object::Object;
use crate::engine::scene::game_object::components::{Component, ComponentType};
use crate::engine::scene::game_object::{GameObject, Position};
use crate::engine::scene::object_manager::GameObjectManager;
use image::DynamicImage;

pub mod game_object;

mod object_manager;

pub struct Scene {
    manager: GameObjectManager,
    pub main_object: GameObject,
}

impl Scene {
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
                        component.get_shadow_unchecked()
                    ));
                }
            }
        }
        renderable_objects.sort_by(|a, b| a.0.position.z.cmp(&b.0.position.z));

        for component in self.main_object.components.iter() {
            if component.get_component_type() == ComponentType::Sprite {
                /*if let Some(img) = component.get_shadow_unchecked() {
                    renderable_objects.push((
                        &self.main_object,
                        &img.0,
                        (
                            component.get_sprite_offset_unchecked().unwrap_or((0, 0)).0 + img.1.0,
                            component.get_sprite_offset_unchecked().unwrap_or((0, 0)).1 + img.1.1,
                        ),
                    ));
                }*/

                if let Some(sprite_img) = component.get_sprite_unchecked().as_ref() {
                    renderable_objects.push((
                        &self.main_object,
                        sprite_img,
                        component.get_sprite_offset_unchecked().unwrap_or((0, 0)),
                        component.get_shadow_unchecked()
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
