use std::cell::RefCell;
use super::*;
use crate::engine::scene::game_object::{GameObject, Object};
use image::DynamicImage;

pub trait Script {
    fn action(&mut self, game_object: &mut GameObject);
    fn new(is_downed: bool) -> Self where Self: Sized;
}
