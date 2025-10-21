/// A trait describing entity for coloring game frames.
/// Takes Vec of pixels colors and paints the main screen of the game correspondingly.

pub trait Screen {
    fn color_frame();
}
