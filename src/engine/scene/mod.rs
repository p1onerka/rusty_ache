use crate::engine::scene::game_object::components::{Component, ComponentType};
use crate::engine::scene::game_object::{GameObject, Object, Position};
use crate::engine::scene::object_manager::GameObjectManager;
use crate::render::renderer::{Rectangle, Renderable};
use crate::screen::Screen;
use image::DynamicImage;
use std::cmp::PartialEq;
use std::collections::HashMap;

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
            main_object: GameObject::new(main_components, main_position),
        }
    }

    pub fn init(&self) -> Vec<(&GameObject, &DynamicImage, (i32, i32))> {
        let mut renderable_objects: Vec<(&GameObject, &DynamicImage, (i32, i32))> = vec![];
        for obj in self.manager.game_objects.values() {
            for component in obj.components.iter() {
                if component.get_component_type() == ComponentType::Sprite {
                    match &component.get_shadow_unchecked() {
                        None => {}
                        Some(img) => {
                            renderable_objects.push((
                                obj,
                                &img.0,
                                (
                                    component.get_sprite_offset_unchecked().unwrap().0 + &img.1.0,
                                    component.get_sprite_offset_unchecked().unwrap().1 + &img.1.1,
                                ),
                            ));
                        }
                    };
                    renderable_objects.push((
                        obj,
                        &component.get_sprite_unchecked().as_ref().unwrap(),
                        component.get_sprite_offset_unchecked().unwrap(),
                    ));
                }
            }
        }
        renderable_objects.sort_by(|a, b| a.0.position.z.cmp(&b.0.position.z));
        for component in self.main_object.components.iter() {
            if component.get_component_type() == ComponentType::Sprite {
                match &component.get_shadow_unchecked() {
                    None => {}
                    Some(img) => {
                        renderable_objects.push((
                            &self.main_object,
                            &img.0,
                            (
                                component.get_sprite_offset_unchecked().unwrap().0 + &img.1.0,
                                component.get_sprite_offset_unchecked().unwrap().1 + &img.1.1,
                            ),
                        ));
                    }
                };
                renderable_objects.push((
                    &self.main_object,
                    &component.get_sprite_unchecked().as_ref().unwrap(),
                    component.get_sprite_offset_unchecked().unwrap(),
                ));
            }
        }
        renderable_objects
    }
}
