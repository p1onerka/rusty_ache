//! Root module for the game engine project.
//!
//! This module exposes submodules for the core engine logic, user interface,
//! rendering subsystem, and screen management.
pub mod engine;
pub mod interface;
pub mod render;
pub mod screen;

/// Represents screen resolution.
///
/// Holds the width and height in pixels, encapsulating display dimensions.
#[derive(Copy, Clone)]
pub struct Resolution {
    /// Width of the screen or rendering area in pixels.
    width: u32,
    /// Height of the screen or rendering area in pixels.
    height: u32,
}

impl Resolution {
    /// Creates a new `Resolution` instance with specified width and height.
    ///
    /// # Parameters
    /// - `width`: Width in pixels.
    /// - `height`: Height in pixels.
    ///
    /// # Returns
    /// A new `Resolution` struct with given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_resolution_with_correct_values() {
        let resolution = Resolution::new(1920, 1080);

        assert_eq!(resolution.width, 1920);
        assert_eq!(resolution.height, 1080);
    }

    #[test]
    fn test_new_with_zero_values() {
        let resolution = Resolution::new(0, 0);

        assert_eq!(resolution.width, 0);
        assert_eq!(resolution.height, 0);
    }

    #[test]
    fn test_new_with_maximum_values() {
        let resolution = Resolution::new(u32::MAX, u32::MAX);

        assert_eq!(resolution.width, u32::MAX);
        assert_eq!(resolution.height, u32::MAX);
    }
}
