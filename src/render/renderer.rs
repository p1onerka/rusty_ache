//! A struct describing any entity that can be rendered

use image::{DynamicImage, GenericImageView};
use std::collections::HashMap;
use std::ptr::read_unaligned;

use crate::Resolution;
use crate::engine::scene::game_object::components::sprite::Sprite;
use crate::engine::scene::game_object::{GameObject, Position};
use crate::engine::scene_manager::SceneManager;
use crate::screen::{HEIGHT, WIDTH};

use super::utils::make_init_frame;

pub const DEFAULT_BACKGROUND_COLOR: (u8, u8, u8, u8) = (245, 245, 220, 255);

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
    renderable: Vec<Renderable>,
    pub(crate) scene_manager: SceneManager,
}

impl Renderer {
    // TODO: add here first edition of image into Screen. it will contain only slice of background
    pub(crate) fn new(
        resolution: Resolution,
        background: Option<DynamicImage>,
        scene_manager: SceneManager,
    ) -> Self {
        let background_clone = background.clone();
        Renderer {
            resolution,
            background,
            prev_frame: make_init_frame(background_clone),
            renderable: Vec::new(),
            scene_manager,
        }
    }

    /// Find intersection of two rectangular. Is used in render to find what part of object (if any)
    /// should be rendered with current camera position
    fn find_intersection(fst: &Rectangle, snd: &Rectangle) -> Option<Rectangle> {
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

    fn recolor_frame() {}

    /// Form new frame based on previous one and info from Engine
    pub(crate) fn render(&self) {
        println!("Starting render");
        // find cam rectangle
        let main_object = &self.scene_manager.active_scene.main_object;

        println!("Main object collected");
        let renderable = self.scene_manager.init_active_scene();

        println!("Renderable collected");
        let camera_rect = Rectangle {
            top_left: (main_object.position.x, main_object.position.y),
            bot_right: (
                main_object.position.x + WIDTH as i32,
                main_object.position.y - HEIGHT as i32,
            ),
        };
        println!("{} {}", main_object.position.y, HEIGHT);

        // TODO: what happens when two objects have the same z?
        let uids_by_z = HashMap::<u32, usize>::new();
        for (obj, img) in renderable {
            let pos = &obj.position;

            let im_size = img.dimensions();
            let im_bot_right = (pos.x + im_size.0 as i32, pos.y - im_size.1 as i32);
            let im_rect = Rectangle {
                top_left: (obj.position.x, obj.position.y),
                bot_right: im_bot_right,
            };
            println!("{} {} {} {}", im_rect.bot_right.0, im_rect.bot_right.1, im_rect.top_left.0, im_rect.top_left.1);
        }

        // sort objects by z coord in descending order
        let mut z_sorted: Vec<u32> = uids_by_z.keys().cloned().collect();
        z_sorted.sort();
        let objs_sorted_by_z: Vec<usize> = z_sorted
            .iter()
            .map(|key| uids_by_z.get(key).cloned().unwrap())
            .collect();

        // distant objects should be rendered first, so near ones can overlap them
        for obj in objs_sorted_by_z {
            // TODO
        }
    }

    /// Emit new frame to Screen
    fn emit() {}
}
