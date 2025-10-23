use crate::engine::scene::game_object::components::Component;
use image::DynamicImage;

pub struct Sprite {
    image: Option<DynamicImage>,
}

impl Sprite {
    pub fn new(image: Option<DynamicImage>) -> Self {
        Sprite { image }
    }
}

impl Component for Sprite {}
