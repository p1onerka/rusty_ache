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
