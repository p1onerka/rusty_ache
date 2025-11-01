use rusty_ache::engine::Engine;
use rusty_ache::engine::scene::game_object::GameObject;
use rusty_ache::engine::scene::game_object::components::script::Script;
use rusty_ache::engine::scene::game_object::position::Position;
use rusty_ache::interface::{create_obj_with_img, init_engine, init_scene};
use rusty_ache::screen::{HEIGHT, WIDTH};

fn main() {
    let tower_obj = create_obj_with_img("src/bin/resources/tower.png", 82, 37, true);
    let junk_house_obj = create_obj_with_img("src/bin/resources/junk_house.png", 150, -150, true);
    let pool_house_obj = create_obj_with_img("src/bin/resources/pool_house.png", 15, -25, true);
    let tall_house_obj = create_obj_with_img("src/bin/resources/tall_house.png", 210, -80, true);
    let skyscraper_obj = create_obj_with_img("src/bin/resources/skyscraper.png", 150, 55, true);
    let cabin_obj = create_obj_with_img("src/bin/resources/cabin.png", 280, -60, true);
    let main_ship_obj = create_obj_with_img("src/bin/resources/white_ship.png", 0, 0, true);

    let hermit_house_obj = create_obj_with_img("src/bin/resources/junk_house.png", 400, 240, true);

    let scene = init_scene(
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
    );
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

    #[test]
    fn test_actions_is_downed_true() {
        let mut script = MyScript::new(true);
        let position = Position {
            x: 15,
            y: 25,
            z: 35,
            is_relative: false,
        };
        let game_object = &mut GameObject::new(vec![], None, position);
        script.action(game_object);
        assert_eq!(game_object.position.x, 15);
        assert_eq!(game_object.position.y, 26);
        assert_eq!(game_object.position.z, 35);
    }
}
