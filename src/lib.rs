pub mod engine;
pub mod render;
pub mod screen;

/// Struct representing screen resolution.
#[derive(Copy, Clone)]
pub struct Resolution {
    width: u32,
    height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_creates_resolution_with_correct_values() {
        let resolution = Resolution::new(1920, 1080);
        
        assert_eq!(resolution.width, 1920);
        assert_eq!(resolution.height, 1080);
    }
    
    #[test]
    fn test_new_with_zero_values() {
        let resolution = Resolution::new(0, 0);
        
        assert_eq!(resolution.width, 0);
        assert_eq!(resolution.height, 0);
    }
    
    #[test]
    fn test_new_with_maximum_values() {
        let resolution = Resolution::new(u32::MAX, u32::MAX);
        
        assert_eq!(resolution.width, u32::MAX);
        assert_eq!(resolution.height, u32::MAX);
    }
}