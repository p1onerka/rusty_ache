//! A struct describing any entity that can be rendered

use image::{DynamicImage, GenericImageView};
use std::collections::HashMap;

use crate::Resolution;
use crate::engine::scene::game_object::Position;
use crate::engine::scene_manager::SceneManager;
use crate::screen::{HEIGHT, WIDTH};

use super::utils::make_init_frame;

pub const DEFAULT_BACKGROUND_COLOR: (u8, u8, u8, u8) = (98, 96, 96, 255);
pub const OFFSET: (i32, i32) = (10, -10);
pub const SHADOW_OPAQUENESS: u8 = 80;

pub struct Renderable {
    pub uid: u32,
    pub sprite: DynamicImage,
    pub visible_area: Rectangle,
    pub position: Position,
}

pub struct Rectangle {
    pub top_left: (i32, i32),
    pub bot_right: (i32, i32),
}

/// A struct describing entity for:
/// * Choosing which pixels to recolor based on info from Engine.
/// * Forming recolored frame and sending it to Screen.
pub struct Renderer {
    resolution: Resolution,
    background: Option<DynamicImage>,
    prev_frame: Vec<(u8, u8, u8, u8)>,
    pub scene_manager: SceneManager,
}

impl Renderer {
    pub(crate) fn new(
        resolution: Resolution,
        background: Option<DynamicImage>,
        scene_manager: SceneManager,
    ) -> Self {
        let background_clone = background.clone();
        let init_frame = make_init_frame(background_clone);
        Renderer {
            resolution,
            background,
            prev_frame: init_frame.clone(),
            scene_manager,
        }
    }

    /// Find intersection of two rectangular. Is used in render to find what part of object (if any)
    /// should be rendered with current camera position
    fn _find_intersection(fst: &Rectangle, snd: &Rectangle) -> Option<Rectangle> {
        let left = fst.top_left.0.max(snd.top_left.0);
        let right = fst.bot_right.0.min(snd.bot_right.0);
        let top = fst.top_left.1.min(snd.top_left.1);
        let bot = fst.bot_right.1.max(snd.bot_right.1);
        if left < right && top > bot {
            return Some(Rectangle {
                top_left: (left, top),
                bot_right: (right, bot),
            });
        };

        None
    }

    fn blit_sprite(
        frame: &mut [(u8, u8, u8, u8)],
        sprite: &DynamicImage,
        visible_area: &Rectangle,
        position: (i32, i32),
        camera_top: (i32, i32),
        frame_size: (i32, i32),
        has_shadow: bool,
    ) {
        let (frame_w, frame_h) = frame_size;

        // loop over world coordinates of visible area
        let (sprite_w, sprite_h) = sprite.dimensions();

        for wy in visible_area.bot_right.1..visible_area.top_left.1 {
            for wx in visible_area.top_left.0..visible_area.bot_right.0 {
                // map world -> sprite coordinates
                if wx < position.0 || wy > position.1 {
                    continue;
                }
                let sprite_x = wx - position.0;
                let sprite_y = position.1 - wy;

                if sprite_x >= sprite_w as i32 || sprite_y >= sprite_h as i32 {
                    continue;
                }

                let px = sprite.get_pixel(sprite_x.try_into().unwrap(), sprite_y as u32);
                let src = px.0;

                // skip transparent pixels
                if src[3] == 0 {
                    continue;
                }

                if has_shadow {
                    let sx_i_shadow = wx + OFFSET.0 - camera_top.0;
                    let sy_i_shadow = camera_top.1 - wy + OFFSET.1;
                    if sx_i_shadow < 0 || sy_i_shadow < 0 {
                        continue;
                    }
                    let sx_shadow = sx_i_shadow as u32;
                    let sy_shadow = sy_i_shadow as u32;
                    if sx_shadow >= frame_w as u32 || sy_shadow >= frame_h as u32 {
                        continue;
                    }
                    let idx = (sy_shadow * frame_w as u32 + sx_shadow) as usize;
                    let existing = frame[idx];
                    let alpha = SHADOW_OPAQUENESS as f32 / 255.0;
                    let blended = (
                        (existing.0 as f32 * (1.0 - alpha)) as u8,
                        (existing.1 as f32 * (1.0 - alpha)) as u8,
                        (existing.2 as f32 * (1.0 - alpha)) as u8,
                        255,
                    );
                    frame[idx] = blended;
                }

                // map world -> screen coordinates
                let sx_i = wx - camera_top.0;
                let sy_i = camera_top.1 - wy;

                if sx_i < 0 || sy_i < 0 {
                    continue;
                }
                let sx = sx_i as u32;
                let sy = sy_i as u32;

                if sx >= frame_w as u32 || sy >= frame_h as u32 {
                    continue;
                }

                // fully opaque => just overwrite
                let idx = (sy * frame_w as u32 + sx) as usize;
                let mut shadowed = src;
                if src[0] == 0 && src[1] == 0 && src[2] == 0 && src[3] != 255 {
                    shadowed[0] = frame[idx].0.saturating_sub(src[3]);
                    shadowed[1] = frame[idx].1.saturating_sub(src[3]);
                    shadowed[2] = frame[idx].2.saturating_sub(src[3]);
                }

                frame[idx] = (shadowed[0], shadowed[1], shadowed[2], shadowed[3]);
            }
        }
    }

    /// Form new frame based on previous one and info from Engine
    pub(crate) fn render(&mut self) {
        // find cam rectangle
        let main_object = &self.scene_manager.active_scene.main_object;
        let mut frame: Vec<(u8, u8, u8, u8)> = make_init_frame(self.background.clone());
        //println!("Main object collected");
        let renderable = self.scene_manager.init_active_scene();

        let _camera_rect = Rectangle {
            top_left: (main_object.position.x, main_object.position.y),
            bot_right: (
                main_object.position.x + WIDTH as i32,
                main_object.position.y - HEIGHT as i32,
            ),
        };

        let _uids_by_z = HashMap::<u32, usize>::new();
        for (obj, img, offset, has_shadow) in renderable {
            let pos = Position {
                x: obj.position.x + offset.0,
                y: obj.position.y + offset.1,
                z: obj.position.z,
                is_relative: obj.position.is_relative,
            };

            let im_size = img.dimensions();
            let im_bot_right = (pos.x + im_size.0 as i32, pos.y - im_size.1 as i32);
            let im_rect = Rectangle {
                top_left: (pos.x, pos.y),
                bot_right: im_bot_right,
            };

            Self::blit_sprite(
                &mut frame,
                img,
                &im_rect,
                (pos.x, pos.y),
                (main_object.position.x, main_object.position.y),
                (self.resolution.width as i32, self.resolution.height as i32),
                has_shadow,
            );
            self.prev_frame = frame.clone();
        }
    }

    /// Emit new frame to Screen
    pub fn emit(&mut self) -> Option<Vec<(u8, u8, u8, u8)>> {
        Some(self.prev_frame.clone())
    }
}

#[cfg(test)]
mod tests {
    use image::{Rgba, RgbaImage};

    use crate::interface::{create_obj_with_img, init_scene};

    use super::*;

    const DEFAULT_BACKGROUND: (u8, u8, u8, u8) = (
                DEFAULT_BACKGROUND_COLOR.0,
                DEFAULT_BACKGROUND_COLOR.1,
                DEFAULT_BACKGROUND_COLOR.2,
                DEFAULT_BACKGROUND_COLOR.3,
            );

    fn test_init_renderer() -> Renderer {
        let resolution = Resolution::new(200, 200);
        let background = None;
        let objs = [create_obj_with_img("image_path", 200, 200, false)];
        let main_obj = create_obj_with_img("image", 300, 300, true);
        let main_scene = init_scene(&objs, main_obj);
        let scene_manager = SceneManager::new(main_scene);
        let renderer = Renderer::new(resolution, background, scene_manager);
        return renderer
    }

    fn create_sprite_with_color(width: u32, height: u32, color: [u8;4]) -> DynamicImage {
        let mut img = RgbaImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, Rgba(color));
            }
        }
        DynamicImage::ImageRgba8(img)
    }

    // #[test]
    // fn test_renderer() {
    //     let renderer = test_init_renderer();
    //     assert_eq!(renderer.resolution.height, 200);
    //     assert_eq!(renderer.resolution.height, 200);
    //     assert_eq!(renderer.background, None);
    //     let mut vector = renderer.prev_frame;
    //     for _ in 0..HEIGHT {
    //     for _ in 0..WIDTH {
    //         assert_eq!(vector.pop(), Some(DEFAULT_BACKGROUND));
    //     }}
    // }

    #[test]
    fn test_find_intersection_symmetric_rectangles() {
        let fst = Rectangle{ top_left: (0, 200), bot_right: (200, 0)};
        let snd = Rectangle{ top_left: (0, 200), bot_right: (200, 0)};
        let result = Renderer::_find_intersection(&fst, &snd);
        match result {
            None => assert!(false),
            Some(res) => {
                assert_eq!(res.top_left, (0, 200));
                assert_eq!(res.bot_right, (200, 0));
            }
        }
    }

    #[test]
    fn test_find_intersection_simple_case() {
        let fst = Rectangle{ top_left: (0, 200), bot_right: (200, 0)};
        let snd = Rectangle{ top_left: (0, 200), bot_right: (150, 150)};
        let result = Renderer::_find_intersection(&fst, &snd);
        match result {
            None => assert!(false),
            Some(res) => {
                assert_eq!(res.top_left, (0, 200));
                assert_eq!(res.bot_right, (150, 150));
            }
        }
    }

    #[test]
    fn test_find_intersection_zero_intersection() {
        let fst = Rectangle{ top_left: (0, 200), bot_right: (200, 0)};
        let snd = Rectangle{ top_left: (-200, 200), bot_right: (0, 0)};
        let result = Renderer::_find_intersection(&fst, &snd);
        match result {
            None => assert!(true),
            Some(_) => assert!(false)
        }
    }

    #[test]
    fn test_fully_opaque_no_shadow() {

        let mut frame = vec![(0u8, 0u8, 0u8, 0u8); 10*10];
        let sprite = create_sprite_with_color(3, 3, [255, 0, 0, 255]); // red opaque
        let visible_area = Rectangle { top_left: (0, 5), bot_right: (5, 0) };
        Renderer::blit_sprite(
            &mut frame,
            &sprite,
            &visible_area,
            (1, 3),
            (0, 5),
            (10, 10),
            false,
        );
        let idx = (2 * 10 + 1) as usize;
        assert_eq!(frame[idx].0, 255);
        assert_eq!(frame[idx].3, 255);
    }
    

    #[test]
    fn test_transparent_pixels_skipped() {
        let mut frame = vec![(100, 100, 100, 100); 10*10];
        let mut sprite_img = RgbaImage::new(2, 2);
        sprite_img.put_pixel(0, 0, Rgba([0, 0, 0, 0]));       // transparent
        sprite_img.put_pixel(1, 0, Rgba([50, 50, 50, 255]));  // opaque
        sprite_img.put_pixel(0, 1, Rgba([10, 10, 10, 0]));    // transparent
        sprite_img.put_pixel(1, 1, Rgba([20, 20, 20, 255]));  // opaque
        let sprite = DynamicImage::ImageRgba8(sprite_img);
        let visible_area = Rectangle { top_left: (1, 2), bot_right: (3, 0) };
        Renderer::blit_sprite(
            &mut frame,
            &sprite,
            &visible_area,
            (1, 1),
            (0, 2),
            (10, 10),
            false,
        );

        assert_eq!(frame[1 * 10 + 0], (100, 100, 100, 100));
    }

    #[test]
    fn test_shadow_pixels_with_partial_alpha_subtract() {
        let mut frame = vec![(100, 100, 100, 255); 10*10];
        let mut sprite_img = RgbaImage::new(1, 1);
        sprite_img.put_pixel(0, 0, Rgba([0, 0, 0, 10])); // partially transparent black pixel
        let sprite = DynamicImage::ImageRgba8(sprite_img);
        let visible_area = Rectangle { top_left: (0, 1), bot_right: (1, 0) };
        Renderer::blit_sprite(
            &mut frame,
            &sprite,
            &visible_area,
            (0, 0),
            (0, 1),
            (10, 10),
            false,
        );

        let idx = (1 * 10 + 0) as usize;
        assert_eq!(frame[idx].0, 90);
        assert_eq!(frame[idx].1, 90);
        assert_eq!(frame[idx].2, 90);
    }

    #[test]
    fn test_pixels_outside_visible_area_not_drawn() {
        let mut frame = vec![(50, 50, 50, 255); 10*10];
        let sprite = create_sprite_with_color(2, 2, [255, 255, 255, 255]);
        let visible_area = Rectangle { top_left: (0, 2), bot_right: (2, 1) };
        Renderer::blit_sprite(
            &mut frame,
            &sprite,
            &visible_area,
            (3, 3),
            (0, 2),
            (10, 10),
            false,
        );

        for color in frame.iter() {
            assert_eq!(*color, (50, 50, 50, 255));
        }
    }

    // #[test]
    // fn test_emit() {
    //     let mut renderer = test_init_renderer();
    //     let result = renderer.emit();
    //     match result {
    //         None => assert!(false),
    //         Some(mut res) => for _ in 0..HEIGHT {
    //     for _ in 0..WIDTH {
    //         assert_eq!(res.pop(), Some(DEFAULT_BACKGROUND));
    //     }}
    //     }
    // }
}