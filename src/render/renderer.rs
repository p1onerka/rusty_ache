//! A struct describing any entity that can be rendered

use std::collections::HashMap;

use image::{DynamicImage, GenericImageView};

use crate::Resolution;
use crate::engine::scene::game_object::GameObject;
use crate::engine::scene::game_object::components::sprite;
use crate::screen::{HEIGHT, WIDTH};

use self::sprite::Sprite;

use super::utils::make_init_frame;

pub const DEFAULT_BACKGROUND_COLOR: (u8, u8, u8, u8) = (245, 245, 220, 255);

pub struct Renderable {
    pub sprite: DynamicImage,
    pub visible_area: Rectangle,
    pub position: (u32, u32),
    pub remoteness: u32,
}

pub struct Rectangle {
    pub top_left: (u32, u32),
    pub bot_right: (u32, u32),
}

/// A struct describing entity for:
/// * Choosing which pixels to recolor based on info from Engine.
/// * Forming recolored frame and sending it to Screen.
pub struct Renderer {
    resolution: Resolution,
    background: Option<DynamicImage>,
    prev_frame: Vec<(u8, u8, u8, u8)>,
    frame_ready: bool,
}

impl Renderer {
    // TODO: add here first emition of image into Screen. it will contain only slice of background
    pub(crate) fn new(resolution: Resolution, background: Option<DynamicImage>) -> Self {
        let background_clone = background.clone();
        Renderer {
            resolution,
            background: background,
            prev_frame: make_init_frame(background_clone),
            frame_ready: false,
        }
    }

    /// Find intersection of two rectangulars. Is used in render to find what part of object (if any)
    /// should be rendered with current camera position
    fn find_intersection(fst: &Rectangle, snd: &Rectangle) -> Option<Rectangle> {
        let left = fst.top_left.0.max(snd.top_left.0);
        let right = fst.bot_right.0.min(snd.bot_right.0);
        let top = fst.top_left.1.min(snd.top_left.1);
        let bot = fst.bot_right.1.max(snd.bot_right.1);
        if left < right && top > bot {
            Some(Rectangle {
                top_left: (left, top),
                bot_right: (right, bot),
            })
        } else {
            return None;
        }
    }

    fn blit_sprite(
        frame: &mut Vec<(u8, u8, u8, u8)>,
        sprite: &DynamicImage,
        visible_area: &Rectangle,
        position: (u32, u32),
        camera_top: (u32, u32),
        frame_size: (u32, u32),
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

                if sprite_x >= sprite_w || sprite_y >= sprite_h {
                    continue;
                }

                let px = sprite.get_pixel(sprite_x, sprite_y);
                let src = px.0;

                // skip transparent pixels
                if src[3] == 0 {
                    continue;
                }

                // map world -> screen coordinates
                let sx_i = wx as i32 - camera_top.0 as i32;
                let sy_i = camera_top.1 as i32 - wy as i32;

                if sx_i < 0 || sy_i < 0 {
                    continue;
                }
                let sx = sx_i as u32;
                let sy = sy_i as u32;

                if sx >= frame_w || sy >= frame_h {
                    continue;
                }

                // fully opaque → just overwrite
                let idx = (sy * frame_w + sx) as usize;
                frame[idx] = (src[0], src[1], src[2], src[3]);
            }
        }
    }

    /// Form new frame based on previous one and info from Engine
    fn render(&mut self, camera: GameObject, objs: Vec<GameObject>) {
        // find cam rectangle
        let camera_rect = Rectangle {
            top_left: (camera.position.x, camera.position.y),
            bot_right: (camera.position.x + WIDTH, camera.position.y - HEIGHT),
        };
        let width = self.resolution.width;
        let height = self.resolution.height;

        // create an empty frame buffer
        let mut frame: Vec<(u8, u8, u8, u8)> = Vec::with_capacity((width * height) as usize);

        if let Some(bg) = &self.background {
            let (bg_w, bg_h) = bg.dimensions();

            // loop over every pixel on screen
            for sy in 0..height {
                for sx in 0..width {
                    // convert screen pixel to world coordinates
                    let world_x = camera.position.x + sx;
                    let world_y = (camera.position.y as i32 - sy as i32) as u32; // Y-up world coords

                    // sample the background if inside its bounds
                    if world_x < bg_w && world_y < bg_h {
                        let p = bg.get_pixel(world_x, world_y).0;
                        frame.push((p[0], p[1], p[2], p[3]));
                    } else {
                        frame.push(DEFAULT_BACKGROUND_COLOR);
                    }
                }
            }
        } else {
            // no background image: fill with default color
            frame.resize((width * height) as usize, DEFAULT_BACKGROUND_COLOR);
        }

        // find all oobjects with sprites that intersect with camera object
        let mut renderable_objs = HashMap::<usize, Renderable>::new();
        // TODO: what happens when two objects have the same z?
        let mut uids_by_z = HashMap::<u32, usize>::new();
        for obj in objs {
            let pos = &obj.position;
            for component in &obj.components {
                if let Some(sprite) = component.as_any().downcast_ref::<Sprite>() {
                    if let Some(image) = &sprite.image {
                        let im_size = image.dimensions();
                        let im_bot_right = (pos.x + im_size.0, pos.y - im_size.1);
                        let im_rect = Rectangle {
                            top_left: (obj.position.x, obj.position.y),
                            bot_right: im_bot_right,
                        };
                        if let Some(intersection) = Self::find_intersection(&camera_rect, &im_rect)
                        {
                            let im = image.clone();
                            let renderable = Renderable {
                                sprite: im,
                                visible_area: intersection,
                                position: (pos.x, pos.y),
                                remoteness: pos.z,
                            };
                            renderable_objs.insert(obj.uid, renderable);
                            uids_by_z.insert(pos.z, obj.uid);
                        }
                    }
                }
            }
        }

        // sort objects by z coord in descending order
        let mut z_sorted: Vec<u32> = uids_by_z.keys().cloned().collect();
        z_sorted.sort();
        let mut objs_sorted_by_z: Vec<usize> = z_sorted
            .iter()
            .map(|key| uids_by_z.get(key).cloned().unwrap())
            .collect();

        // distant objects should be rendered first, so near ones can overlap them
        for obj in objs_sorted_by_z {
            // TODO
            if let Some(renderable) = renderable_objs.get(&obj) {
                Self::blit_sprite(
                    &mut frame,
                    &renderable.sprite,
                    &renderable.visible_area,
                    renderable.position,
                    (camera.position.x, camera.position.y),
                    (self.resolution.width, self.resolution.height),
                );
            }
        }
        self.prev_frame = frame;
        self.frame_ready = true;
    }

    /// Emit new frame to Screen
    pub fn emit(&mut self) -> Option<Vec<(u8, u8, u8, u8)>> {
        if self.frame_ready {
            self.frame_ready = false;
            Some(self.prev_frame.clone())
        } else {
            None
        }
    }
}
