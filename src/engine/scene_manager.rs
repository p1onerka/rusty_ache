use crate::engine::scene::Scene;
use crate::engine::scene::game_object::GameObject;
use image::DynamicImage;

pub struct SceneManager {
    pub(crate) active_scene: Scene,
}

impl SceneManager {
    pub fn new(main_scene: Scene) -> Self {
        SceneManager {
            active_scene: main_scene,
        }
    }

    pub fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    pub fn init_active_scene(&self) -> Vec<(&GameObject, &DynamicImage)> {
        self.active_scene.init()
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::scene::game_object::{Object, Position, components::{Component, sprite::Sprite}};

    use super::*;
    
    fn create_test_position(x: i32, y: i32, z: i32, is_relative: bool) -> Position {
        Position { x, y, z, is_relative }
    }
    
    fn create_test_components() -> Vec<Box<dyn Component>> {
        vec![Box::new(Sprite::new(None))]
    }
    
    fn create_simple_scene() -> Scene {
        Scene::new(
            vec![],
            create_test_components(),
            create_test_position(0, 0, 0, false),
        )
    }

    fn create_scene_with_sprites(sprite_count: usize) -> Scene {
        let mut objects = vec![];
        
        for i in 0..sprite_count {
            let obj = GameObject::new(
                create_test_components(),
                create_test_position(i as i32, i as i32, i as i32, false),
            );
            objects.push(obj);
        }
        
        Scene::new(objects, vec![], create_test_position(0, 0, 0, false))
    }
    
    #[test]
    fn test_new_stores_provided_scene() {
        let scene = Scene::new(
            vec![],
            create_test_components(),
            create_test_position(10, 20, 30, false),
        );
        
        let manager = SceneManager::new(scene);
        
        assert_eq!(manager.active_scene.main_object.position.x, 10);
        assert_eq!(manager.active_scene.main_object.position.y, 20);
        assert_eq!(manager.active_scene.main_object.position.z, 30);
    }
    
    #[test]
    fn test_active_scene_returns_same_scene() {
        let scene = Scene::new(
            vec![],
            create_test_components(),
            create_test_position(15, 25, 35, false),
        );
        
        let manager = SceneManager::new(scene);
        let active = manager.active_scene();
        
        assert_eq!(active.main_object.position.x, 15);
        assert_eq!(active.main_object.position.y, 25);
        assert_eq!(active.main_object.position.z, 35);
    }
    
    #[test]
    fn test_init_active_scene_returns_empty_for_scene_without_sprites() {
        let scene = create_simple_scene();
        let manager = SceneManager::new(scene);
        
        let renderable = manager.init_active_scene();
        
        assert_eq!(renderable.len(), 0);
    }
    
    // #[test]
    // fn test_init_active_scene_returns_sprites() {
    //     let scene = create_scene_with_sprites(1);
    //     let manager = SceneManager::new(scene);
        
    //     let renderable = manager.init_active_scene();
        
    //     assert_eq!(renderable.len(), 1);
    // }
    
    #[test]
    fn test_scene_manager_with_empty_scene() {
        let scene = Scene::new(vec![], vec![], create_test_position(0, 0, 0, false));
        let manager = SceneManager::new(scene);
        
        let renderable = manager.init_active_scene();
        
        assert_eq!(renderable.len(), 0);
    }
    
    // #[test]
    // fn test_scene_manager_with_scene_containing_objects_without_sprites() {
    //     let obj1 = GameObject::new(
    //         create_test_components(),
    //         create_test_position(0, 0, 0, false),
    //     );
    //     let obj2 = GameObject::new(
    //         create_test_components(),
    //         create_test_position(10, 10, 10, false),
    //     );
        
    //     let scene = Scene::new(
    //         vec![obj1, obj2],
    //         vec![],
    //         create_test_position(0, 0, 0, false),
    //     );
    //     let manager = SceneManager::new(scene);
        
    //     let renderable = manager.init_active_scene();
        
    //     assert_eq!(renderable.len(), 0);
    // }
    
    #[test]
    fn test_active_scene_preserves_scene_structure() {
        let obj1 = GameObject::new(
            create_test_components(),
            create_test_position(1, 2, 3, false),
        );
        let obj2 = GameObject::new(
            create_test_components(),
            create_test_position(4, 5, 6, false),
        );
        
        let scene = Scene::new(
            vec![obj1, obj2],
            vec![],
            create_test_position(7, 8, 9, false),
        );
        
        let manager = SceneManager::new(scene);
        let active = manager.active_scene();
        assert_eq!(active.main_object.position.x, 7);
    }
    
    
    #[test]
    fn test_active_scene_is_immutable_reference() {
        let scene = create_simple_scene();
        let manager = SceneManager::new(scene);
        
        let _active = manager.active_scene();
        let _active2 = manager.active_scene();
    }
}