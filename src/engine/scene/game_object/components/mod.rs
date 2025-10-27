//! A trait describing an entity of game object component, such as Sprite, Camera etc.

use image::DynamicImage;
use std::any::Any;
use std::rc::Rc;

pub mod sprite;
mod velocity;

pub enum ComponentError {
    Exist(Box<dyn Component>),
    CannotApply(String),
    UnknownError(String),
    InvalidIndex(String),
}

#[derive(Eq, PartialEq, Clone)]
pub enum ComponentType {
    Sprite,
    Velocity,
}

pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;
    fn get_component_type(&self) -> ComponentType;

    fn get_sprite_unchecked(&self) -> &Option<DynamicImage> {
        &None
    }
}
