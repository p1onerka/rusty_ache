//! A trait describing an entity of game object component, such as Sprite, Camera etc.

pub enum ComponentError {
    Exist(Box<dyn Component>),
    CannotApply(String),
    UnknownError(String),
}

pub trait Component {}
