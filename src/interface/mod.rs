use image::ImageReader;

use crate::{
    Resolution,
    engine::{
        Engine, GameEngine,
        config::{Config, EngineConfig},
        scene::{
            Scene,
            game_object::{GameObject, Object, Position, components::sprite::Sprite},
        },
    },
};

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
    res
}

/// A function for initializing ObjectWithImage which is a simplified interface of GameObject
pub fn create_obj_with_img(image_path: &str, x: i32, y: i32, has_shadow: bool) -> ObjectWithImage {
    ObjectWithImage {
        image_path,
        x,
        y,
        has_shadow,
    }
}

/// A function for initializing the Scene object based on ObjectWithImage vector and main object
pub fn init_scene(objs: &[ObjectWithImage], main_obj: ObjectWithImage) -> Scene {
    let game_objs = create_gameobj_vec(objs);
    Scene::new(
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
    )
}

pub fn init_engine(scene: Scene, width: u32, height: u32) -> GameEngine {
    GameEngine::new(
        Box::new(EngineConfig::new(Resolution::new(width, height))),
        scene,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_obj_with_img() {
        let obj = create_obj_with_img("./image", 100, 100, true);
        assert_eq!(obj.image_path, "./image");
        assert_eq!(obj.x, 100);
        assert_eq!(obj.y, 100);
        assert!(obj.has_shadow);
    }

    #[test]
    fn test_create_gameobj_vec_empty() {
        let objs = [];
        let owi = create_gameobj_vec(&objs);
        assert_eq!(owi.len(), 0)
    }

    #[test]
    fn test_create_gameobj_vec() {
        let objs = [create_obj_with_img("image_path", 200, 200, false)];
        let owi = create_gameobj_vec(&objs);
        assert_eq!(owi.len(), objs.len());
        assert_eq!(objs[0].x, owi[0].position.x);
        assert_eq!(objs[0].y, owi[0].position.y);
    }

    #[test]
    fn test_init_scene() {
        let objs = [create_obj_with_img("image_path", 200, 200, false)];
        let main_obj = create_obj_with_img("image", 300, 300, true);
        let main_obj_x = main_obj.x;
        let main_obj_y = main_obj.y;
        let scene = init_scene(&objs, main_obj);
        assert_eq!(scene.main_object.position.x, main_obj_x);
        assert_eq!(scene.main_object.position.y, main_obj_y);
    }
}
