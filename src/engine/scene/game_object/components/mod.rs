//! A trait describing an entity of game object component, such as Sprite, Camera etc.

use std::any::Any;

pub(crate) mod sprite;
mod velocity;

pub enum ComponentError {
    Exist(Box<dyn Component>),
    CannotApply(String),
    UnknownError(String),
}

pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;
}
