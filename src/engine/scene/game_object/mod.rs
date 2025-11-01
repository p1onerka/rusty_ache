//! Defines the core game object entity and its behavior within the engine.
//!
//! This module provides the `Object` trait which outlines essential operations
//! such as component management, position control, and script execution.
//! The `GameObject` struct implements this trait as a concrete entity holding
//! components, an optional script, and its current position.
//!
//! Error enums encapsulate possible failure modes in component handling,
//! unique identifier issues, position updates, and unknown errors.

use crate::engine::scene::game_object::components::script::Script;
use crate::engine::scene::game_object::components::{Component, ComponentError, ComponentType};
pub(crate) use crate::engine::scene::game_object::position::Position;

pub mod components;
pub mod position;

/// Errors that can arise at the GameObject level.
pub enum GameObjectError {
    /// Represents an error originating from a component operation.
    ComponentError(ComponentError),
    /// Error related to unique identifier (UID) management.
    UIDError(String),
    /// Position-related errors.
    PositionError(String),
    /// Catch-all variant for unknown or unexpected errors.
    UnknownError(String),
}

/// Defines an interface for game objects.
///
/// Game objects are entities with components, position, and optional behavior scripts.
/// This trait enumerates constructor and methods for component management,
/// position queries and mutations, and script-driven actions.
pub trait Object {
    fn new(
        components: Vec<Box<dyn Component + Send + Sync>>,
        script: Option<Box<dyn Script + Send + Sync>>,
        position: Position,
    ) -> Self;

    fn add_component(
        &mut self,
        component: Box<dyn Component + Send + Sync>,
    ) -> Result<(), GameObjectError>;

    fn remove_component(&mut self, component_id: usize) -> Result<(), GameObjectError>;

    fn get_position(&self) -> Result<&Position, GameObjectError>;

    fn update_position(&mut self, position: Position) -> Result<(), GameObjectError>;

    fn add_position(&mut self, vec: (i32, i32));

    fn run_action(&self);
}

/// The primary game object structure holding components, optional script, and position.
/// Maximum 256 objects per 1 scene
pub struct GameObject {
    pub components: Vec<Box<dyn Component + Send + Sync>>,
    pub script: Option<Box<dyn Script + Send + Sync>>,
    pub position: Position,
}

impl Object for GameObject {
    /// Constructs a new game object from components, script, and position.
    /// Checks for sprite components to call related accessors.
    fn new(
        components: Vec<Box<dyn Component + Send + Sync>>,
        script: Option<Box<dyn Script + Send + Sync>>,
        position: Position,
    ) -> Self {
        for component in &components {
            if component.get_component_type() == ComponentType::Sprite {
                component.get_sprite_unchecked();
            }
        }
        GameObject {
            components,
            script,
            position,
        }
    }

    /// Adds a component, performing sprite-specific checks if applicable.
    /// Always returns Ok currently.
    fn add_component(
        &mut self,
        component: Box<dyn Component + Send + Sync>,
    ) -> Result<(), GameObjectError> {
        if component.get_component_type() == ComponentType::Sprite {
            component.get_sprite_unchecked();
        }
        self.components.push(component);
        Ok(())
    }

    /// Gets a reference to the current position.
    fn get_position(&self) -> Result<&Position, GameObjectError> {
        Ok(&self.position)
    }

    /// Removes the component at the given index, or returns error if index is invalid.
    fn remove_component(&mut self, component_id: usize) -> Result<(), GameObjectError> {
        if component_id >= self.components.len() {
            return Err(GameObjectError::ComponentError(
                ComponentError::InvalidIndex(format!(
                    "Component ID {} is out of bounds (length: {})",
                    component_id,
                    self.components.len()
                )),
            ));
        }
        self.components.remove(component_id);
        Ok(())
    }

    /// Updates the position of the game object.
    fn update_position(&mut self, position: Position) -> Result<(), GameObjectError> {
        self.position = position;
        Ok(())
    }

    /// Adds a relative (x, y) offset to the current position.
    fn add_position(&mut self, vec: (i32, i32)) {
        self.position.x += vec.0;
        self.position.y += vec.1;
    }

    /// Runs the associated script action on the game object.
    ///
    /// Currently a stub; should be implemented to invoke `script.action`.
    fn run_action(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::engine::scene::game_object::components::sprite::Sprite;

    use super::*;

    fn create_test_game_object() -> GameObject {
        let position = Position {
            x: 0,
            y: 0,
            z: 0,
            is_relative: false,
        };
        let components: Vec<Box<dyn Component + Send + Sync>> =
            vec![Box::new(Sprite::new(None, false, (0, 0)))];
        GameObject::new(components, None, position)
    }

    #[test]
    fn test_new() {
        let position = Position {
            x: 10,
            y: 20,
            z: 30,
            is_relative: true,
        };
        let components: Vec<Box<dyn Component + Send + Sync>> =
            vec![Box::new(Sprite::new(None, false, (0, 0)))];

        let game_object = GameObject::new(components, None, position);

        assert_eq!(game_object.components.len(), 1);
        assert_eq!(game_object.position.x, 10);
        assert_eq!(game_object.position.y, 20);
        assert_eq!(game_object.position.z, 30);
    }

    #[test]
    fn test_add_component_increases_component_count() {
        let mut game_object = create_test_game_object();
        let initial_count = game_object.components.len();

        let new_component = Box::new(Sprite::new(None, false, (0, 0)));
        let result = game_object.add_component(new_component);

        assert!(result.is_ok());
        assert_eq!(game_object.components.len(), initial_count + 1);
    }

    #[test]
    fn test_remove_component_valid_index() {
        let mut game_object = create_test_game_object();
        let initial_count = game_object.components.len();

        let result = game_object.remove_component(0);

        assert!(result.is_ok());
        assert_eq!(game_object.components.len(), initial_count - 1);
    }

    #[test]
    fn test_remove_component_invalid_index() {
        let mut game_object = create_test_game_object();
        let component_count = game_object.components.len();

        let result = game_object.remove_component(component_count + 10);

        assert!(result.is_err());
        match result {
            Err(GameObjectError::ComponentError(ComponentError::InvalidIndex(msg))) => {
                assert!(msg.contains("out of bounds"));
            }
            _ => panic!("Expected InvalidIndex error"),
        }
    }

    #[test]
    fn test_get_position() {
        let position = Position {
            x: 15,
            y: 25,
            z: 35,
            is_relative: false,
        };
        let game_object = GameObject::new(vec![], None, position);

        let result = game_object.get_position();

        assert!(result.is_ok());
        if let Ok(pos) = result {
            assert_eq!(pos.x, 15);
            assert_eq!(pos.y, 25);
            assert_eq!(pos.z, 35);
        }
    }

    #[test]
    fn test_update_position() {
        let mut game_object = create_test_game_object();
        let new_position = Position {
            x: 100,
            y: 200,
            z: 300,
            is_relative: false,
        };

        let result = game_object.update_position(new_position);

        assert!(result.is_ok());
        assert_eq!(game_object.position.x, 100);
        assert_eq!(game_object.position.y, 200);
        assert_eq!(game_object.position.z, 300);
    }

    #[test]
    fn test_new_empty_components_list() {
        let position = Position {
            x: 0,
            y: 0,
            z: 0,
            is_relative: true,
        };
        let game_object = GameObject::new(vec![], None, position);

        assert_eq!(game_object.components.len(), 0);
    }

    #[test]
    fn test_remove_all_components() {
        let mut game_object = create_test_game_object();
        let component_count = game_object.components.len();

        for _ in 0..component_count {
            let result = game_object.remove_component(0);
            assert!(result.is_ok());
        }

        assert_eq!(game_object.components.len(), 0);
    }
}
