use std::any::Any;

use crate::engine::scene::game_object::components::{Component, ComponentType};
use image::DynamicImage;

pub struct Sprite {
    pub image: Option<DynamicImage>,
    pub shadow: Option<(DynamicImage, (i32, i32))>,
    pub offset: (i32, i32),
}

impl Sprite {
    pub fn new(
        image: Option<DynamicImage>,
        shadow: Option<(DynamicImage, (i32, i32))>,
        offset: (i32, i32),
    ) -> Self {
        Sprite {
            image,
            shadow,
            offset,
        }
    }
}

impl Component for Sprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_component_type(&self) -> ComponentType {
        ComponentType::Sprite
    }

    fn get_sprite_unchecked(&self) -> &Option<DynamicImage> {
        &self.image
    }

    fn get_shadow_unchecked(&self) -> &Option<(DynamicImage, (i32, i32))> {
        &self.shadow
    }
    fn get_sprite_offset_unchecked(&self) -> Option<(i32, i32)> {
        Some(self.offset)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbImage};

    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let img = RgbImage::new(width, height);
        DynamicImage::ImageRgb8(img)
    }

    mod creation {
        use super::*;

        #[test]
        fn test_sprite_without_image() {
            let sprite = Sprite::new(None);
            assert!(sprite.image.is_none());
        }

        #[test]
        fn test_new_sprite_with_image() {
            let image = create_test_image(100, 100);
            let sprite = Sprite::new(Some(image));

            assert!(sprite.image.is_some());
        }

        #[test]
        fn test_sprite_correct_dimensions() {
            let image = create_test_image(200, 150);
            let sprite = Sprite::new(Some(image));

            assert!(sprite.image.is_some());
            if let Some(ref img) = sprite.image {
                assert_eq!(img.width(), 200);
                assert_eq!(img.height(), 150);
            } else {
                panic!("No image presented");
            }
        }
    }

    mod component_trait {
        use super::*;

        #[test]
        fn test_get_component_type_returns_sprite() {
            let sprite = Sprite::new(None);
            assert_eq!(sprite.get_component_type(), ComponentType::Sprite);
        }

        #[test]
        fn test_as_any_returns_correct_type() {
            let sprite = Sprite::new(None);
            let any = sprite.as_any();

            assert!(any.is::<Sprite>());
            assert!(any.downcast_ref::<Sprite>().is_some());
        }

        #[test]
        fn test_as_any_downcasting() {
            let sprite = Sprite::new(None);
            let any = sprite.as_any();

            let downcasted = any.downcast_ref::<Sprite>();
            assert!(downcasted.is_some());

            if let Some(s) = downcasted {
                assert_eq!(s.get_component_type(), ComponentType::Sprite);
            }
        }

        #[test]
        fn test_get_sprite_unchecked_returns_image() {
            let image = create_test_image(50, 50);
            let sprite = Sprite::new(Some(image));

            let result = sprite.get_sprite_unchecked();
            assert!(result.is_some());
        }

        #[test]
        fn test_get_sprite_unchecked_without_image() {
            let sprite = Sprite::new(None);
            let result = sprite.get_sprite_unchecked();
            assert!(result.is_none());
        }
    }
}
