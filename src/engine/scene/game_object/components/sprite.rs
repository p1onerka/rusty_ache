use std::any::Any;

use crate::engine::scene::game_object::components::{Component, ComponentType};
use image::DynamicImage;

pub struct Sprite {
    pub image: Option<DynamicImage>,
    pub shadow: Option<(DynamicImage, (i32, i32))>,
    pub offset: (i32, i32),
}

impl Sprite {
    pub fn new(
        image: Option<DynamicImage>,
        shadow: Option<(DynamicImage, (i32, i32))>,
        offset: (i32, i32),
    ) -> Self {
        Sprite {
            image,
            shadow,
            offset,
        }
    }
}

impl Component for Sprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_component_type(&self) -> ComponentType {
        ComponentType::Sprite
    }

    fn get_sprite_unchecked(&self) -> &Option<DynamicImage> {
        &self.image
    }

    fn get_shadow_unchecked(&self) -> &Option<(DynamicImage, (i32, i32))> {
        &self.shadow
    }
    fn get_sprite_offset_unchecked(&self) -> Option<(i32, i32)> {
        Some(self.offset)
    }
}
