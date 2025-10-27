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
    fn new(components: Vec<Box<dyn Component>>, position: Position) -> Self;
    fn add_component(&mut self, component: Box<dyn Component>) -> Result<(), GameObjectError>;

    fn remove_component(&mut self, component_id: usize) -> Result<(), GameObjectError>;

    fn get_position(&self) -> Result<&Position, GameObjectError>;

    fn update_position(&mut self, position: Position) -> Result<(), GameObjectError>;
}

pub struct GameObject {
    pub components: Vec<Box<dyn Component>>,
    pub position: Position,
}

impl Object for GameObject {
    fn new(components: Vec<Box<dyn Component>>, position: Position) -> Self {
        GameObject {
            components,
            position,
        }
    }
    fn add_component(&mut self, component: Box<dyn Component>) -> Result<(), GameObjectError> {
        self.components.push(component);
        Ok(())
    }
    fn get_position(&self) -> Result<&Position, GameObjectError> {
        Ok(&self.position)
    }
    fn remove_component(&mut self, component_id: usize) -> Result<(), GameObjectError> {
        if component_id >= self.components.len() {
            return Err(GameObjectError::ComponentError(
                ComponentError::InvalidIndex(format!(
                    "Component ID {} is out of bounds (length: {})",
                    component_id,
                    self.components.len()
                )),
            ));
        }

        self.components.remove(component_id);
        Ok(())
    }
    fn update_position(&mut self, position: Position) -> Result<(), GameObjectError> {
        self.position = position;
        Ok(())
    }
}
