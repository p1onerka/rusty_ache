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
