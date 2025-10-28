use crate::engine::scene::game_object::GameObject;
use crate::engine::scene::game_object::Position;
use crate::engine::scene::game_object::components::Component;
use crate::engine::scene::game_object::{GameObject, Object};
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
        components: Vec<Box<dyn Component + Send + Sync>>,
        position: Position,
    ) -> (usize, GameObject) {
        if self.uids.is_empty() && self.max_objects == self.allocated_objects {
            panic!("Trying to create object above limit")
        } else if !self.uids.is_empty() {
            let uid = *self.uids.iter().next().unwrap();
            self.uids.remove(&uid);
            return (uid, GameObject::new(components, None, position));
        }
        self.allocated_objects += 1;
        (
            self.allocated_objects,
            GameObject::new(components, None, position),
        )
    }
}

#[cfg(test)]
mod factory_tests {
    use super::*;
    use crate::engine::scene::game_object::components::sprite::Sprite;

    fn create_test_position(x: i32, y: i32, z: i32, is_relative: bool) -> Position {
        Position {
            x,
            y,
            z,
            is_relative,
        }
    }

    fn create_test_components() -> Vec<Box<dyn Component>> {
        vec![Box::new(Sprite::new(None))]
    }

    #[test]
    fn test_new_creates_factory() {
        let factory = GameObjectFactory::new(100);

        assert_eq!(factory.max_objects, 100);
        assert_eq!(factory.allocated_objects, 0);
        assert!(factory.uids.is_empty());
    }

    #[test]
    fn test_new_with_zero_max_objects() {
        let factory = GameObjectFactory::new(0);

        assert_eq!(factory.max_objects, 0);
        assert_eq!(factory.allocated_objects, 0);
        assert!(factory.uids.is_empty());
    }

    #[test]
    fn test_create_object_returns_unique_uid() {
        let mut factory = GameObjectFactory::new(10);
        let components = create_test_components();
        let position = create_test_position(0, 0, 0, false);

        let (uid, _obj) = factory.create_object(components, position);

        assert_eq!(uid, 1);
        assert_eq!(factory.allocated_objects, 1);
    }

    #[test]
    fn test_create_object_returns_game_object_with_correct_position() {
        let mut factory = GameObjectFactory::new(10);
        let position = create_test_position(10, 20, 30, false);

        let (_uid, obj) = factory.create_object(create_test_components(), position);

        assert_eq!(obj.position.x, 10);
        assert_eq!(obj.position.y, 20);
        assert_eq!(obj.position.z, 30);
    }

    #[test]
    fn test_create_object_returns_game_object_with_components() {
        let mut factory = GameObjectFactory::new(10);
        let components = vec![
            Box::new(Sprite::new(None)) as Box<dyn Component>,
            Box::new(Sprite::new(None)) as Box<dyn Component>,
        ];

        let (_uid, obj) = factory.create_object(components, create_test_position(0, 0, 0, false));

        assert_eq!(obj.components.len(), 2);
    }

    #[test]
    #[should_panic(expected = "Trying to create object above limit")]
    fn test_create_object_panics_when_exceeding_limit() {
        let mut factory = GameObjectFactory::new(2);

        factory.create_object(
            create_test_components(),
            create_test_position(0, 0, 0, false),
        );
        factory.create_object(
            create_test_components(),
            create_test_position(1, 1, 1, false),
        );

        factory.create_object(
            create_test_components(),
            create_test_position(2, 2, 2, false),
        );
    }

    #[test]
    fn test_create_object_reuses_freed_uid() {
        let mut factory = GameObjectFactory::new(10);

        factory.create_object(
            create_test_components(),
            create_test_position(0, 0, 0, false),
        );

        factory.uids.insert(1);

        let (uid, _) = factory.create_object(
            create_test_components(),
            create_test_position(1, 1, 1, false),
        );

        assert_eq!(uid, 1);
        assert!(factory.uids.is_empty());
        assert_eq!(factory.allocated_objects, 1);
    }

    #[test]
    fn test_create_object_reuses_multiple_freed_uids() {
        let mut factory = GameObjectFactory::new(10);

        factory.create_object(
            create_test_components(),
            create_test_position(0, 0, 0, false),
        );
        factory.create_object(
            create_test_components(),
            create_test_position(1, 1, 1, false),
        );
        factory.create_object(
            create_test_components(),
            create_test_position(2, 2, 2, false),
        );

        factory.uids.insert(1);
        factory.uids.insert(2);

        let (uid1, _) = factory.create_object(
            create_test_components(),
            create_test_position(3, 3, 3, false),
        );
        let (uid2, _) = factory.create_object(
            create_test_components(),
            create_test_position(4, 4, 4, false),
        );

        assert!(uid1 == 1 || uid1 == 2);
        assert!(uid2 == 1 || uid2 == 2);
        assert_ne!(uid1, uid2);
        assert!(factory.uids.is_empty());
    }

    #[test]
    fn test_create_object_with_freed_uids_at_limit() {
        let mut factory = GameObjectFactory::new(2);

        factory.create_object(
            create_test_components(),
            create_test_position(0, 0, 0, false),
        );
        factory.create_object(
            create_test_components(),
            create_test_position(1, 1, 1, false),
        );

        factory.uids.insert(1);

        let (uid, _) = factory.create_object(
            create_test_components(),
            create_test_position(2, 2, 2, false),
        );

        assert_eq!(uid, 1);
        assert_eq!(factory.allocated_objects, 2);
    }

    #[test]
    fn test_create_object_with_empty_components() {
        let mut factory = GameObjectFactory::new(10);

        let (_uid, obj) = factory.create_object(vec![], create_test_position(0, 0, 0, false));

        assert_eq!(obj.components.len(), 0);
    }

    #[test]
    fn test_uid_reuse_priority() {
        let mut factory = GameObjectFactory::new(10);

        factory.create_object(
            create_test_components(),
            create_test_position(0, 0, 0, false),
        );
        factory.create_object(
            create_test_components(),
            create_test_position(1, 1, 1, false),
        );

        factory.uids.insert(5);

        let (uid, _) = factory.create_object(
            create_test_components(),
            create_test_position(2, 2, 2, false),
        );

        assert_eq!(uid, 5);
        assert!(factory.uids.is_empty());
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

    pub fn add_game_object(
        &mut self,
        components: Vec<Box<dyn Component + Send + Sync>>,
        position: Position,
    ) {
        let (uid, object) = self.factory.create_object(components, position);
        self.game_objects.insert(uid, object);
    }
}

#[cfg(test)]
mod manager_tests {
    use crate::engine::scene::game_object::components::sprite::Sprite;

    use super::*;

    fn create_test_position(x: i32, y: i32, z: i32, is_relative: bool) -> Position {
        Position {
            x,
            y,
            z,
            is_relative,
        }
    }

    fn create_test_components() -> Vec<Box<dyn Component>> {
        vec![Box::new(Sprite::new(None))]
    }

    #[test]
    fn test_new_with_different_capacities() {
        let manager1 = GameObjectManager::new(0);
        let manager2 = GameObjectManager::new(100);
        let manager3 = GameObjectManager::new(1000);

        assert_eq!(manager1.game_objects.len(), 0);
        assert_eq!(manager2.game_objects.len(), 0);
        assert_eq!(manager3.game_objects.len(), 0);
    }

    #[test]
    fn test_add_game_object_adds_to_hashmap() {
        let mut manager = GameObjectManager::new(10);

        manager.add_game_object(
            create_test_components(),
            create_test_position(0, 0, 0, false),
        );

        assert_eq!(manager.game_objects.len(), 1);
        assert!(manager.game_objects.contains_key(&1));
    }

    #[test]
    fn test_add_game_object_stores_correct_position() {
        let mut manager = GameObjectManager::new(10);
        let position = create_test_position(15, 25, 35, false);

        manager.add_game_object(create_test_components(), position);

        let obj = manager.game_objects.get(&1).unwrap();
        assert_eq!(obj.position.x, 15);
        assert_eq!(obj.position.y, 25);
        assert_eq!(obj.position.z, 35);
    }

    #[test]
    fn test_add_game_object_with_empty_components() {
        let mut manager = GameObjectManager::new(10);

        manager.add_game_object(vec![], create_test_position(0, 0, 0, false));

        assert_eq!(manager.game_objects.len(), 1);
        let obj = manager.game_objects.get(&1).unwrap();
        assert_eq!(obj.components.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Trying to create object above limit")]
    fn test_add_game_object_with_zero_limit() {
        let mut manager = GameObjectManager::new(0);

        manager.add_game_object(
            create_test_components(),
            create_test_position(0, 0, 0, false),
        );
    }

    #[test]
    fn test_add_game_object_with_negative_positions() {
        let mut manager = GameObjectManager::new(10);

        manager.add_game_object(
            create_test_components(),
            create_test_position(-10, -20, -30, false),
        );

        let obj = manager.game_objects.get(&1).unwrap();
        assert_eq!(obj.position.x, -10);
        assert_eq!(obj.position.y, -20);
        assert_eq!(obj.position.z, -30);
    }

    #[test]
    fn test_manager_can_retrieve_objects_by_uid() {
        let mut manager = GameObjectManager::new(10);

        manager.add_game_object(
            create_test_components(),
            create_test_position(100, 200, 300, false),
        );

        let retrieved = manager.game_objects.get(&1);
        assert!(retrieved.is_some());

        let obj = retrieved.unwrap();
        assert_eq!(obj.position.x, 100);
        assert_eq!(obj.position.y, 200);
        assert_eq!(obj.position.z, 300);
    }

    #[test]
    fn test_manager_returns_none_for_nonexistent_uid() {
        let manager = GameObjectManager::new(10);

        let retrieved = manager.game_objects.get(&999);
        assert!(retrieved.is_none());
    }
}
