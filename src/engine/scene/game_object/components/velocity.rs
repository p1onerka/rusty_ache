use super::*;
use image::DynamicImage;

pub struct Velocity {
    x: usize,
    y: usize,
}

impl Velocity {
    pub fn new() -> Self {
        Velocity { x: 0, y: 0 }
    }
}

impl Component for Velocity {}
