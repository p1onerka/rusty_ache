//! A struct describing any entity that can be rendered

use crate::render::utils::Resolution;

pub trait Renderable {}

/// A struct describing entity for:
/// * Choosing which pixels to recolor based on info from Engine.
/// * Forming recolored frame and sending it to Screen.
pub struct  Renderer {
    resolution: Resolution,
    background: image::Rgb<u8>,
}

impl Renderer {
    /// Form new frame based on previous one and info from Engine
    fn render() {

    }

    /// Emit new frame to Screen
    fn emit() {

    }
}
