use rusty_ache::engine::scene::game_object::components::script::Script;
use rusty_ache::engine::scene::game_object::position::Position;
use rusty_ache::engine::scene::game_object::GameObject;
use rusty_ache::engine::Engine;
use rusty_ache::screen::{HEIGHT, WIDTH};
use rusty_ache::interface::{create_obj_with_img, init_engine, init_scene};

fn main() {
    let tower_obj = create_obj_with_img("src/bin/resources/tower.png", 90, 40, true);
    let junk_house_obj = create_obj_with_img("src/bin/resources/junk_house.png", 130, -100, true);
    let pool_house_obj = create_obj_with_img("src/bin/resources/pool_house.png", 15, -25, true);
    let main_ship_obj = create_obj_with_img("src/bin/resources/white_ship.png", 0, 0, true);

    let scene = init_scene(&[tower_obj, junk_house_obj, pool_house_obj], main_ship_obj);
    let mut engine = init_engine(scene, WIDTH, HEIGHT);
    engine.render().unwrap();
    engine.run().unwrap()
}

pub struct MyScript {
    is_downed: bool,
}

impl Script for MyScript {
    fn new(is_downed: bool) -> MyScript {
        MyScript { is_downed }
    }

    fn action(&mut self, game_object: &mut GameObject) {
        if !self.is_downed {
            game_object.position = Position {
                x: game_object.position.x,
                y: game_object.position.y - 1,
                z: game_object.position.z,
                is_relative: game_object.position.is_relative,
            };
            self.is_downed = true;
        } else {
            game_object.position = Position {
                x: game_object.position.x,
                y: game_object.position.y + 1,
                z: game_object.position.z,
                is_relative: game_object.position.is_relative,
            };
            self.is_downed = false;
        }
    }
}
