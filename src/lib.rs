pub mod engine;
pub mod render;
pub mod screen;

/// Struct representing screen resolution.
#[derive(Copy, Clone)]
pub struct Resolution {
    width: u32,
    height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
