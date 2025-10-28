use crate::engine::scene::game_object::components::script::Script;
use crate::engine::scene::game_object::components::{Component, ComponentError};
pub(crate) use crate::engine::scene::game_object::position::Position;

pub mod components;
pub mod position;

pub enum GameObjectError {
    ComponentError(ComponentError),
    UIDError(String),
    PositionError(String),
    UnknownError(String),
}

/// A trait describing the basic game object entity
pub trait Object {
    fn get_uid(&self) -> usize;
    fn add_component(&mut self, game_object: Box<dyn Component>) -> Result<(), GameObjectError>;

    fn remove_component(&mut self, game_object_id: usize) -> Result<(), GameObjectError>;

    fn get_position(&self) -> Result<Position, GameObjectError>;

    fn update_position(&mut self, position: Position) -> Result<(), GameObjectError>;
}

pub struct GameObject {
    pub components: Vec<Box<dyn Component + Send + Sync>>,
    pub script: Option<Box<dyn Script + Send + Sync>>,
    pub position: Position,
}

impl GameObject {
    pub fn new(
        components: Vec<Box<dyn Component + Send + Sync>>,
        script: Option<Box<dyn Script + Send + Sync>>,
        position: Position,
    ) -> Self {
        
        GameObject {
            components,
            script,
            position,
        }
    }

    pub fn add_position(&mut self, vec: (i32, i32)) {
        self.position.x += vec.0;
        self.position.y += vec.1;
    }

    pub fn run_action(&self) {}
}
