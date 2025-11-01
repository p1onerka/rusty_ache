//! A trait describing a game object component entity, such as Sprite, Camera, etc.
//!
//! This module defines the core Component trait representing various components
//! that can be attached to game entities. It also defines the component types
//! and error types related to component handling.

use image::DynamicImage;
use std::any::Any;
use std::fmt::Debug;

pub mod script;
pub mod sprite;
mod velocity;

/// Errors that can occur when handling components.
pub enum ComponentError {
    /// Component already exists.
    Exist(Box<dyn Component>),
    /// Operation could not be applied on component.
    CannotApply(String),
    /// An unknown error occurred.
    UnknownError(String),
    /// Invalid index was provided.
    InvalidIndex(String),
}

/// Enum identifying types of components supported.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ComponentType {
    Sprite,
    Velocity,
    Action,
}

/// Trait that defines behavior of any game component.
///
/// Components represent modular features or data attached to an entity,
/// such as a visual sprite, motion velocity, or behavior scripts.
///
/// Implementors must specify their component type and can optionally
/// override accessors to retrieve related data.
pub trait Component: Any {
    /// Returns the component as a dynamic Any reference, allowing downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Returns the component's type identifier.
    fn get_component_type(&self) -> ComponentType;

    /// Returns the sprite image if available.
    ///
    /// Default returns None; override in Sprite component implementations.
    fn get_sprite_unchecked(&self) -> &Option<DynamicImage> {
        &None
    }

    /// Indicates whether the component casts shadows.
    ///
    /// Default returns true; override if shadow behavior differs.
    fn get_shadow_unchecked(&self) -> bool {
        true
    }

    /// Returns the sprite offset if applicable.
    ///
    /// Default returns None; override if component supports sprite offsetting.
    fn get_sprite_offset_unchecked(&self) -> Option<(i32, i32)> {
        None
    }
}
