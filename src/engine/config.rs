/// use crate::engine::Scene;
/// A trait describing entity for Engine configuring
use crate::Resolution;

pub trait Config {
    fn set_resolution(&mut self, width: u32, height: u32);

    fn get_resolution(&self) -> Resolution;

    fn new(resolution: Resolution) -> Self
    where
        Self: Sized;
}

#[derive()]
pub struct EngineConfig {
    resolution: Resolution,
}

impl Config for EngineConfig {
    fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution.width = width;
        self.resolution.height = height;
    }
    fn get_resolution(&self) -> Resolution {
        Resolution {
            width: self.resolution.width,
            height: self.resolution.height,
        }
    }

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
