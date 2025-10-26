#[derive(Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub is_relative: bool,
}
