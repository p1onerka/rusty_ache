//! A struct describing any entity that can be rendered

use std::collections::HashMap;

use image::{DynamicImage, GenericImageView};

use crate::Resolution;
use crate::engine::scene::game_object::components::sprite;
use crate::engine::scene::game_object::GameObject;
use crate::screen::{HEIGHT, WIDTH};

use self::sprite::Sprite;

pub trait Renderable {}

pub struct Rectangle {
    pub top_left: (u32, u32),
    pub bot_right: (u32, u32),
}

/// A struct describing entity for:
/// * Choosing which pixels to recolor based on info from Engine.
/// * Forming recolored frame and sending it to Screen.
pub struct Renderer {
    resolution: Resolution,
    background: image::Rgb<u8>,
}

impl Renderer {
    // TODO: add here first emition of image into Screen. it will contain only slice of background
    pub(crate) fn new(resolution: Resolution, background: (u8, u8, u8)) -> Self {
        Renderer {
            resolution,
            background: image::Rgb([background.0, background.1, background.2]),
        }
    }

    /// Find intersection of two rectangulars. Is used in render to find what part of object (if any)
    /// should be rendered with current camera position
    fn find_intersection(
        fst: Rectangle,
        snd: Rectangle,
    ) -> Option<Rectangle> {
        let left = fst.top_left.0.max(snd.top_left.0);
        let right = fst.bot_right.0.min(snd.bot_right.0);
        let top = fst.top_left.1.min(snd.top_left.1);
        let bot = fst.bot_right.1.max(snd.bot_right.1);
        if left < right && top < bot {
            Some(Rectangle {
                top_left: (left, top),
                bot_right: (right, bot),
            })
        } else {
            return None;
        }
    }

    /// Form new frame based on previous one and info from Engine
    fn render(camera: GameObject, objs: Vec<GameObject>) {
        // find cam rectangle
        let camera_rect = Rectangle {
            top_left: (camera.position.x, camera.position.y),
            bot_right: (camera.position.x + WIDTH, camera.position.y + HEIGHT),
        };

        // find all rectangles of sprites
        let mut objs_rects = HashMap::<usize, (Rectangle, DynamicImage)>::new();
        for obj in objs {
            let pos = &obj.position;
            for component in &obj.components {
                if let Some(sprite) = component.as_any().downcast_ref::<Sprite>() {
                    if let Some(image) = &sprite.image {
                        let im_size = image.dimensions();
                        let im_bot_right = (pos.x + im_size.0, pos.y + im_size.1);
                        let im_rect = Rectangle {
                            top_left: (obj.position.x, obj.position.y),
                            bot_right: im_bot_right,
                        };
                        let im = image.clone();
                        objs_rects.insert(obj.uid, (im_rect, im));
                    }
                }
            }
        }

        // find camera & objs intersections. basically, who will be rendered
        let mut objs_intersections = HashMap::<usize, Rectangle>::new();

        // sort objects by z coord in descending order. 
        // distant objects should be rendered first, so near ones can overlap them
        let mut objs_sorted_by_z = Vec::<usize>::new();
    }

    /// Emit new frame to Screen
    fn emit() {}
}
