//! A trait describing an entity of game object component, such as Sprite, Camera etc.

use image::DynamicImage;
use std::any::Any;
use std::rc::Rc;

pub mod sprite;
mod velocity;

pub mod script;

pub enum ComponentError {
    Exist(Box<dyn Component>),
    CannotApply(String),
    UnknownError(String),
}

#[derive(Eq, PartialEq, Clone)]
pub enum ComponentType {
    Sprite,
    Velocity,
    Action,
}

pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;
    fn get_component_type(&self) -> ComponentType;

    fn get_sprite_unchecked(&self) -> &Option<DynamicImage> {
        &None
    }
    fn get_shadow_unchecked(&self) -> &Option<(DynamicImage, (i32, i32))> {
        &None
    }
    fn get_sprite_offset_unchecked(&self) -> Option<(i32, i32)> {
        None
    }
}
