use super::*;
use crate::engine::scene::game_object::Object;
use image::DynamicImage;

pub struct Velocity {
    x: usize,
    y: usize,
    component_type: ComponentType,
}

impl Velocity {
    pub fn new() -> Self {
        Velocity {
            x: 0,
            y: 0,
            component_type: ComponentType::Velocity,
        }
    }
    pub fn update(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

impl Component for Velocity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_component_type(&self) -> ComponentType {
        ComponentType::Velocity
    }
}
