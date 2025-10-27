use crate::engine::scene::game_object::{GameObject, Object};

pub mod game_object;
mod object_manager;

/// A trait describing entity for accumulating objects and handling their interactions
pub trait Scene {
    fn add_game_object(&mut self, game_object: dyn Object);
    fn remove_game_object(&mut self, game_object_id: usize);
}
