//! Represents a velocity component attached to a game object.
//!
//! The `Velocity` struct holds positional delta values typically used
//! to update an entity's movement or position each frame.
//! It implements the `Component` trait to integrate with the component system.

use super::*;
use std::any::Any;

/// Component storing velocity in x and y directions.
pub struct Velocity {
    _x: usize,
    _y: usize,
    _component_type: ComponentType,
}

impl Velocity {
    /// Constructs a new `Velocity` component with zero initial velocity.
    ///
    /// # Returns
    /// A `Velocity` instance with both `_x` and `_y` set to zero.
    pub fn _new() -> Self {
        Velocity {
            _x: 0,
            _y: 0,
            _component_type: ComponentType::Velocity,
        }
    }

    /// Updates the velocity components to new values.
    ///
    /// # Parameters
    /// - `x`: New horizontal velocity.
    /// - `y`: New vertical velocity.
    pub fn _update(&mut self, x: usize, y: usize) {
        self._x = x;
        self._y = y;
    }
}

impl Component for Velocity {
    /// Returns a reference to this component as a dynamic Any for downcasting.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Returns the component type identifier as `ComponentType::Velocity`.
    fn get_component_type(&self) -> ComponentType {
        ComponentType::Velocity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creation {
        use super::*;

        #[test]
        fn test_new_velocity_values() {
            let velocity = Velocity::_new();
            assert_eq!(velocity._x, 0);
            assert_eq!(velocity._y, 0);
        }

        #[test]
        fn test_velocity_component_type() {
            let velocity = Velocity::_new();
            assert_eq!(velocity._component_type, ComponentType::Velocity);
        }
    }

    mod component_trait {
        use super::*;

        #[test]
        fn test_update_changes_values() {
            let mut velocity = Velocity::_new();
            velocity._update(100, 200);

            assert_eq!(velocity._x, 100);
            assert_eq!(velocity._y, 200);
        }

        #[test]
        fn test_get_component_type() {
            let velocity = Velocity::_new();
            assert_eq!(velocity.get_component_type(), ComponentType::Velocity);
        }

        #[test]
        fn test_as_any_correct_type() {
            let velocity = Velocity::_new();
            let any = velocity.as_any();

            assert!(any.is::<Velocity>());
            assert!(any.downcast_ref::<Velocity>().is_some());
        }

        #[test]
        fn test_as_any_downcasting() {
            let velocity = Velocity::_new();
            let any = velocity.as_any();

            let downcasted = any.downcast_ref::<Velocity>();
            assert!(downcasted.is_some());

            if let Some(v) = downcasted {
                assert_eq!(v._x, 0);
                assert_eq!(v._y, 0);
            }
        }

        #[test]
        fn test_velocity_does_not_have_sprite() {
            let velocity = Velocity::_new();
            let sprite = velocity.get_sprite_unchecked();
            assert!(sprite.is_none());
        }
    }
}
