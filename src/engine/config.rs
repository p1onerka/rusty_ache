/// use crate::engine::Scene;

/// A trait describing entity for Engine configuring
pub trait Config {
    fn set_resolution(&mut self, width: u32, height: u32);
    fn new(resolution: Resolution) -> Self
    where
        Self: Sized;
}

pub struct Resolution {
    width: u32,
    height: u32,
}

pub struct EngineConfig {
    resolution: Resolution,
}

impl Config for EngineConfig {
    fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution.width = width;
        self.resolution.height = height;
    }
    fn new(resolution: Resolution) -> Self {
        EngineConfig { resolution }
    }
}
