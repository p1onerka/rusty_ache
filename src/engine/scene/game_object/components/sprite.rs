use std::any::Any;

use crate::engine::scene::game_object::components::Component;
use image::DynamicImage;

pub struct Sprite {
    pub image: Option<DynamicImage>,
}

impl Sprite {
    pub fn new(image: Option<DynamicImage>) -> Self {
        Sprite { image }
    }
}

impl Component for Sprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
