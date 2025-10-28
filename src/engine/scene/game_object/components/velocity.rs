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

#[cfg(test)]
mod tests {
    use super::*;
    
    mod creation {
        use super::*;
        
        #[test]
        fn test_new_velocity_values() {
            let velocity = Velocity::new();
            assert_eq!(velocity.x, 0);
            assert_eq!(velocity.y, 0);
        }

        #[test]
        fn test_velocity_component_type() {
            let velocity = Velocity::new();
            assert_eq!(velocity.component_type, ComponentType::Velocity);
        }
    }

    mod component_trait {
        use super::*;

        #[test]
        fn test_update_changes_values() {
            let mut velocity = Velocity::new();
            velocity.update(100, 200);
            
            assert_eq!(velocity.x, 100);
            assert_eq!(velocity.y, 200);
        }
        
        #[test]
        fn test_get_component_type() {
            let velocity = Velocity::new();
            assert_eq!(velocity.get_component_type(), ComponentType::Velocity);
        }
        
        #[test]
        fn test_as_any_correct_type() {
            let velocity = Velocity::new();
            let any = velocity.as_any();
            
            assert!(any.is::<Velocity>());
            assert!(any.downcast_ref::<Velocity>().is_some());
        }

        #[test]
        fn test_as_any_downcasting() {
            let velocity = Velocity::new();
            let any = velocity.as_any();
            
            let downcasted = any.downcast_ref::<Velocity>();
            assert!(downcasted.is_some());
            
            if let Some(v) = downcasted {
                assert_eq!(v.x, 0);
                assert_eq!(v.y, 0);
            }
        }
        
        #[test]
        fn test_velocity_does_not_have_sprite() {
            let velocity = Velocity::new();
            let sprite = velocity.get_sprite_unchecked();
            assert!(sprite.is_none());
        }
    }
}