use std::collections::HashSet;
use crate::engine::*;
use crate::engine::scene::game_object::components::Component;
use crate::engine::scene::game_object::GameObject;
use crate::engine::scene::game_object::Position;

pub struct GameObjectFactory {
    uids: HashSet<usize>,
    max_objects: usize,
    allocated_objects: usize,
}

impl GameObjectFactory {
    pub fn new(max_objects: usize) -> Self {
        GameObjectFactory {
            uids: HashSet::new(),
            max_objects,
            allocated_objects: 0,
        }
    }

    pub fn create_object(&mut self, components: Vec<Box<dyn Component>>, position: Position) -> GameObject {
        if self.uids.is_empty() && self.max_objects == self.allocated_objects {
            panic!("Trying to create object above limit")
        } else if self.uids.is_empty() == false {
            let uid = self.uids.iter().next().unwrap().clone();
            self.uids.remove(&uid);
            return GameObject::new(components, position, uid)
        }
        self.allocated_objects += 1;
        GameObject::new(components, position, self.allocated_objects)
    }
}
