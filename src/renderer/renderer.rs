/// A trait describing any entity that can be rendered
pub trait Renderable {}

/// A trait describing entity for:
/// 1. Choosing which pixels to recolor based on info from Engine.
/// 2. Forming recolored frame and sending it to Screen.
pub trait Renderer {
    /// Form new frame based on previous one and info from Engine
    fn render();
    /// Emit new frame to Screen
    fn emit();
}
