//! Represents a position in 3D space within the game world.
//!
//! The `Position` struct holds coordinates along the x, y, and z axes,
//! as well as a flag indicating whether the position is relative or absolute.
//!
//! This struct is used to track and manipulate game object spatial placement.

/// A 3D position with optional relativity.
///
/// - `x`, `y`, `z`: Coordinates in the game world's 3D space.
/// - `is_relative`: Flag indicating if the position is relative (true) or absolute (false).
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub is_relative: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position {
            x: 10,
            y: 20,
            z: 30,
            is_relative: false,
        };
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 20);
        assert_eq!(pos.z, 30);
        assert_eq!(pos.is_relative, false);
    }

    #[test]
    fn test_position_modification() {
        let mut pos = Position {
            x: 0,
            y: 0,
            z: 0,
            is_relative: true,
        };
        pos.x = 5;
        pos.y = -5;
        pos.z = 10;
        pos.is_relative = false;

        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, -5);
        assert_eq!(pos.z, 10);
        assert_eq!(pos.is_relative, false);
    }
}
