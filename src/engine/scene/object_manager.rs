use crate::engine::scene::Scene;
use crate::engine::scene::game_object::{Object, GameObject};
use crate::engine::scene::game_object::Position;
use crate::engine::scene::game_object::components::Component;
use std::collections::{HashMap, HashSet};

struct GameObjectFactory {
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

    pub fn create_object(
        &mut self,
        components: Vec<Box<dyn Component>>,
        position: Position,
    ) -> (usize, GameObject) {
        if self.uids.is_empty() && self.max_objects == self.allocated_objects {
            panic!("Trying to create object above limit")
        } else if self.uids.is_empty() == false {
            let uid = self.uids.iter().next().unwrap().clone();
            self.uids.remove(&uid);
            return (uid, GameObject::new(components, position));
        }
        self.allocated_objects += 1;
        (
            self.allocated_objects,
            GameObject::new(components, position),
        )
    }
}

pub struct GameObjectManager {
    pub game_objects: HashMap<usize, GameObject>,
    factory: GameObjectFactory,
}

impl GameObjectManager {
    pub fn new(max_objects: usize) -> Self {
        GameObjectManager {
            game_objects: HashMap::new(),
            factory: GameObjectFactory::new(max_objects),
        }
    }

    pub fn add_game_object(&mut self, components: Vec<Box<dyn Component>>, position: Position) {
        let (uid, object) = self.factory.create_object(components, position);
        self.game_objects.insert(uid, object);
    }
}
