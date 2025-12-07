use image::ImageReader;

use rusty_ache::engine::Engine;
use rusty_ache::engine::scene::game_object::GameObject;
use rusty_ache::engine::scene::game_object::components::ComponentType::Action;
use rusty_ache::engine::scene::game_object::components::script::Script;
use rusty_ache::engine::scene::game_object::components::sprite::Sprite;
use rusty_ache::engine::scene::game_object::components::{Component, ComponentType};
use rusty_ache::engine::scene::game_object::position::Position;
use rusty_ache::interface::{create_obj_with_img, init_end_scene, init_engine, init_scene};

use rusty_ache::screen::{HEIGHT, WIDTH};
use std::any::Any;

use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct MyScript {
    is_downed: bool,
    movement: u64,
}

impl Script for MyScript {
    fn action(&mut self, game_object: &mut GameObject) {
        self.movement += 1;
        println!("movement: {}", self.movement);
        if self.movement < 30 {
            game_object.position.y -= 1;
        } else if self.movement < 60 {
            game_object.position.y += 1;
        } else {
            self.movement = 0;
        }
    }

    fn new(is_downed: bool) -> MyScript {
        MyScript {
            is_downed,
            movement: 1,
        }
    }

    fn clone_box(&self) -> Box<dyn Script + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Component for MyScript {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_component_type(&self) -> ComponentType {
        Action
    }

    fn clone_box(&self) -> Box<dyn Component + Send + Sync> {
        Box::new(self.clone())
    }
}

fn main() {
    let tower_obj = create_obj_with_img("src/bin/resources/tower.png", 82, 37, true);
    let junk_house_obj = create_obj_with_img("src/bin/resources/junk_house.png", 150, -150, true);
    let pool_house_obj = create_obj_with_img("src/bin/resources/pool_house.png", 15, -25, true);
    let tall_house_obj = create_obj_with_img("src/bin/resources/tall_house.png", 210, -80, true);
    let skyscraper_obj = create_obj_with_img("src/bin/resources/skyscraper.png", 150, 55, true);
    let cabin_obj = create_obj_with_img("src/bin/resources/cabin.png", 280, -60, true);
    let main_ship_obj = create_obj_with_img("src/bin/resources/white_ship.png", 0, 0, true);

    let hermit_house_obj = create_obj_with_img("src/bin/resources/junk_house.png", 400, 240, true);
    let mut uids: Vec<usize> = vec![];
    let mut scene = init_scene(
        &[
            cabin_obj,
            skyscraper_obj,
            hermit_house_obj,
            tower_obj,
            tall_house_obj,
            junk_house_obj,
            pool_house_obj,
        ],
        main_ship_obj,
        &mut uids,
    );

    uids.push(scene.manager.add_game_object(
        vec![Box::new(Sprite::new(
            Some(ImageReader::open("src/bin/resources/white_ship.png").unwrap().decode().unwrap()),
            true,
            (0, 0),
        ))],
        Position {
            x: 60,
            y: -10,
            z: 40,
            is_relative: false,
        },
    ));

    let script = MyScript::new(false);
    let end_scene = init_end_scene("src/bin/resources/game_over.jpg", None);
    let mut engine = init_engine(scene, end_scene, WIDTH, HEIGHT);

    let main_pos_arc = engine.main_pos.clone();
    let end_scene_flag = engine.is_end_scene_active.clone();
    let is_removed = false;

    thread::spawn(move || {
        loop {
            let (x, y) = *main_pos_arc.read().unwrap();
            if x > 150 {
                end_scene_flag.store(true, std::sync::atomic::Ordering::SeqCst);
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

    engine.render().unwrap();
    engine.run(vec![(8, Box::new(script))]).unwrap()
}

#[cfg(test)]
mod tests {
    use rusty_ache::engine::scene::game_object::{GameObject, Object, position::Position};

    use crate::{MyScript, Script};

    #[test]
    fn test_new_script() {
        let script = MyScript::new(false);
        assert!(!script.is_downed)
    }

    #[test]
    fn test_actions_is_downed_false() {
        let mut script = MyScript::new(false);
        let position = Position {
            x: 15,
            y: 25,
            z: 35,
            is_relative: false,
        };
        let game_object = &mut GameObject::new(vec![], None, position);
        script.action(game_object);
        assert_eq!(game_object.position.x, 15);
        assert_eq!(game_object.position.y, 24);
        assert_eq!(game_object.position.z, 35);
    }
}
