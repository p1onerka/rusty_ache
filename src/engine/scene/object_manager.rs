use crate::engine::scene::Scene;
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

// mod game_object_factory_tests {
//     use super::*;
//     struct MockComponent;
//     impl Component for MockComponent {
        
//         fn as_any(&self) -> &dyn std::any::Any {
//             todo!()
//         }
        
//         fn get_component_type(&self) -> crate::engine::scene::game_object::components::ComponentType {
//             todo!()
//         }
        
//         fn get_sprite_unchecked(&self) -> &Option<image::DynamicImage> {
//             &None
//         }
//     }

//     #[test]
//     fn test_new_factory_init() {
//         let factory = GameObjectFactory::new(7);
//         assert_eq!(factory.max_objects, 7);
//         assert_eq!(factory.allocated_objects, 0);
//         assert!(factory.uids.is_empty())
//     }

//     #[test]
//     fn test_create_object() {
//         let mut factory = GameObjectFactory::new(5);
//         let components: Vec<Box<dyn Component>> = vec![Box::new(MockComponent)];
//         let position = Position { x: 10, y: 20, z: 15, is_relative: false };
        
//         let (uid, obj) = factory.create_object(components, position);
        
//         assert_eq!(uid, 1);
//         assert_eq!(factory.allocated_objects, 1);
//         assert!(factory.uids.is_empty());
//     }

//     #[test]
//     fn test_create_multiple_objects_within_limit() {
//         let mut factory = GameObjectFactory::new(3);
        
//         let (uid1, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 0.0, y: 0.0 }
//         );
//         let (uid2, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 1.0, y: 1.0 }
//         );
//         let (uid3, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 2.0, y: 2.0 }
//         );
        
//         assert_eq!(uid1, 1);
//         assert_eq!(uid2, 2);
//         assert_eq!(uid3, 3);
//         assert_eq!(factory.allocated_objects, 3);
//     }

//     #[test]
//     #[should_panic(expected = "Trying to create object above limit")]
//     fn test_create_object_exceeds_limit() {
//         let mut factory = GameObjectFactory::new(2);
        
//         // Создаем два объекта - достигаем лимита
//         factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 0.0, y: 0.0 }
//         );
//         factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 1.0, y: 1.0 }
//         );
        
//         // Третий объект должен вызвать панику
//         factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 2.0, y: 2.0 }
//         );
//     }

//     #[test]
//     fn test_reuse_uid_from_pool() {
//         let mut factory = GameObjectFactory::new(5);
        
//         // Создаем объект
//         let (uid1, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 0.0, y: 0.0 }
//         );
        
//         // Вручную добавляем UID обратно в пул (имитация удаления объекта)
//         factory.uids.insert(uid1);
        
//         // Создаем новый объект - должен использовать освобожденный UID
//         let (uid2, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 1.0, y: 1.0 }
//         );
        
//         assert_eq!(uid1, uid2);
//         assert!(factory.uids.is_empty());
//         // allocated_objects не должен увеличиться
//         assert_eq!(factory.allocated_objects, 1);
//     }

//     #[test]
//     fn test_reuse_multiple_uids() {
//         let mut factory = GameObjectFactory::new(10);
        
//         // Создаем несколько объектов
//         let (uid1, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 0.0, y: 0.0 }
//         );
//         let (uid2, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 1.0, y: 1.0 }
//         );
        
//         // Освобождаем оба UID
//         factory.uids.insert(uid1);
//         factory.uids.insert(uid2);
        
//         // Создаем два новых объекта
//         let (reused_uid1, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 2.0, y: 2.0 }
//         );
//         let (reused_uid2, _) = factory.create_object(
//             vec![Box::new(MockComponent)],
//             Position { x: 3.0, y: 3.0 }
//         );
        
//         // Проверяем, что использованы старые UID
//         assert!(reused_uid1 == uid1 || reused_uid1 == uid2);
//         assert!(reused_uid2 == uid1 || reused_uid2 == uid2);
//         assert_ne!(reused_uid1, reused_uid2);
//         assert!(factory.uids.is_empty());
//     }


// }

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
