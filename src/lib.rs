pub mod engine;
pub mod render;
pub mod screen;

/// Struct representing screen resolution.
#[derive(Copy, Clone)]
pub struct Resolution {
    width: u32,
    height: u32,
}
