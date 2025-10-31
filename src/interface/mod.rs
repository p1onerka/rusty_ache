use image::ImageReader;

use crate::{engine::{config::{Config, EngineConfig}, scene::{
    game_object::{components::sprite::Sprite, GameObject, Object, Position}, Scene
}, Engine, GameEngine}, Resolution};

#[derive(Clone)]
pub struct ObjectWithImage<'a> {
    image_path: &'a str,
    x: i32,
    y: i32,
    has_shadow: bool,
}

/// A function for initializing vector of GameObjects with their ObjectWithImage interface.
/// Objects should be arranged in a vector from farthest from the viewer to closest
pub fn create_gameobj_vec(objs: &[ObjectWithImage]) -> Vec<GameObject> {
    let mut res = Vec::new();
    let mut z_coord = 1;
    for obj in objs {
        res.push(GameObject::new(
            vec![Box::new(Sprite::new(
                Some(ImageReader::open(obj.image_path).unwrap().decode().unwrap()),
                obj.has_shadow,
                (0, 0),
            ))],
            None,
            Position {
                x: obj.x,
                y: obj.y,
                z: z_coord,
                is_relative: false,
            },
        ));
        z_coord += 1;
    }
    return res;
}

/// A function for initializing ObjectWithImage which is a simplified interface of GameObject
pub fn create_obj_with_img(image_path: &str, x: i32, y: i32, has_shadow: bool) -> ObjectWithImage {
    return ObjectWithImage {
        image_path,
        x,
        y,
        has_shadow,
    };
}

/// A function for initializing the Scene object based on ObjectWithImage vector and main object
pub fn init_scene(objs: &[ObjectWithImage], main_obj: ObjectWithImage) -> Scene {
    let game_objs = create_gameobj_vec(objs);
    return Scene::new(
        game_objs,
        vec![Box::new(Sprite::new(
            Some(
                ImageReader::open(main_obj.image_path)
                    .unwrap()
                    .decode()
                    .unwrap(),
            ),
            true,
            (60, -60),
        ))],
        Position {
            x: main_obj.x,
            y: main_obj.y,
            z: 0,
            is_relative: false,
        },
    );
}

pub fn init_engine(scene: Scene, width: u32, height: u32) -> GameEngine {
    return GameEngine::new(
        Box::new(EngineConfig::new(Resolution::new(width, height))), scene)
}
