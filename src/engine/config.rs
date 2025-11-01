//! Defines configuration traits and structs for the game engine.
//!
//! This module provides a `Config` trait to standardize engine configuration behavior,
//! focusing on resolution settings. The `EngineConfig` struct implements this trait,
//! encapsulating screen resolution management.

use crate::Resolution;

/// Trait defining configuration interface for engine settings.
///
/// Allows setting and retrieving the screen resolution.
/// The `new` method constructs a configuration instance with an initial resolution.
pub trait Config {
    /// Sets the resolution width and height.
    ///
    /// # Parameters
    /// - `width`: Screen width in pixels.
    /// - `height`: Screen height in pixels.
    fn set_resolution(&mut self, width: u32, height: u32);

    /// Gets the current resolution.
    ///
    /// # Returns
    /// A `Resolution` struct representing the current screen dimensions.
    fn get_resolution(&self) -> Resolution;

    /// Creates a new configuration instance with the given resolution.
    ///
    /// # Parameters
    /// - `resolution`: Initial resolution settings.
    ///
    /// # Returns
    /// A new implementing instance of the Config trait.
    fn new(resolution: Resolution) -> Self
    where
        Self: Sized;
}

/// Concrete implementation of the engine configuration.
///
/// Stores screen resolution and provides access using the `Config` trait.
#[derive()]
pub struct EngineConfig {
    /// The current resolution settings.
    resolution: Resolution,
}

impl Config for EngineConfig {
    /// Sets the resolution width and height.
    fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution.width = width;
        self.resolution.height = height;
    }

    /// Returns a copy of the current resolution.
    fn get_resolution(&self) -> Resolution {
        Resolution {
            width: self.resolution.width,
            height: self.resolution.height,
        }
    }

    /// Creates a new `EngineConfig` with the specified resolution.
    fn new(resolution: Resolution) -> Self {
        EngineConfig { resolution }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_engine_config() {
        let resolution = Resolution::new(1920, 1080);
        let config = EngineConfig::new(resolution);

        assert_eq!(config.resolution.width, 1920);
        assert_eq!(config.resolution.height, 1080);
    }

    #[test]
    fn test_get_resolution() {
        let resolution = Resolution::new(1280, 720);
        let config = EngineConfig::new(resolution);

        let resolution = config.get_resolution();
        assert_eq!(resolution.width, 1280);
        assert_eq!(resolution.height, 720);
    }

    #[test]
    fn test_set_resolution() {
        let initial_resolution = Resolution::new(800, 600);
        let mut config = EngineConfig::new(initial_resolution);

        config.set_resolution(2560, 1440);

        let updated = config.get_resolution();
        assert_eq!(updated.width, 2560);
        assert_eq!(updated.height, 1440);
    }
}
