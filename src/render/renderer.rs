//! A struct describing any entity that can be rendered

use crate::Resolution;

pub trait Renderable {}

/// A struct describing entity for:
/// * Choosing which pixels to recolor based on info from Engine.
/// * Forming recolored frame and sending it to Screen.
pub struct Renderer {
    resolution: Resolution,
    background: image::Rgb<u8>,
}

impl Renderer {
    pub(crate) fn new(resolution: Resolution, background: (u8, u8, u8)) -> Self {
        Renderer {
            resolution,
            background: image::Rgb([background.0, background.1, background.2]),
        }
    }

    /// Form new frame based on previous one and info from Engine
    fn render() {}

    /// Emit new frame to Screen
    fn emit() {}
}
