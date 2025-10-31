use image::ImageReader;

use rusty_ache::Resolution;
use rusty_ache::engine::config::{Config, EngineConfig};
use rusty_ache::engine::scene::Scene;
use rusty_ache::engine::scene::game_object::components::script::Script;
use rusty_ache::engine::scene::game_object::components::sprite::Sprite;
use rusty_ache::engine::scene::game_object::position::Position;
use rusty_ache::engine::scene::game_object::{GameObject, Object};
use rusty_ache::engine::{Engine, GameEngine};
use rusty_ache::screen::{HEIGHT, WIDTH};

fn main() {
    let mut engine = GameEngine::new(
        Box::new(EngineConfig::new(Resolution::new(WIDTH, HEIGHT))),
        Scene::new(
            vec![
                GameObject::new(
                    vec![Box::new(Sprite::new(
                        Some(
                            ImageReader::open("src/bin/resources/junk_house.png")
                                .unwrap()
                                .decode()
                                .unwrap(),
                        ),
                        None,
                        (0, 0),
                    ))],
                    None,
                    Position {
                        x: 0,
                        y: 0,
                        z: 1,
                        is_relative: false,
                    },
                ),
                GameObject::new(
                    vec![Box::new(Sprite::new(
                        Some(
                            ImageReader::open("src/bin/resources/tall_house.png")
                                .unwrap()
                                .decode()
                                .unwrap(),
                        ),
                        Some((
                            ImageReader::open("src/bin/resources/cc_shadow.png")
                                .unwrap()
                                .decode()
                                .unwrap(),
                            (-7, 4),
                        )),
                        (0, 0),
                    ))],
                    None,
                    Position {
                        x: 130,
                        y: -100,
                        z: 2,
                        is_relative: false,
                    },
                ),
                GameObject::new(
                    vec![Box::new(Sprite::new(
                        Some(
                            ImageReader::open("src/bin/resources/tall_house.png")
                                .unwrap()
                                .decode()
                                .unwrap(),
                        ),
                        Some((
                            ImageReader::open("src/bin/resources/cc_shadow.png")
                                .unwrap()
                                .decode()
                                .unwrap(),
                            (-7, 4),
                        )),
                        (0, 0),
                    ))],
                    None,
                    Position {
                        x: 15,
                        y: -25,
                        z: 3,
                        is_relative: false,
                    },
                ),
            ],
            vec![Box::new(Sprite::new(
                Some(
                    ImageReader::open("src/bin/resources/white_ship.png")
                        .unwrap()
                        .decode()
                        .unwrap(),
                ),
                Some((
                    ImageReader::open("src/bin/resources/bc_shadow.png")
                        .unwrap()
                        .decode()
                        .unwrap(),
                    (0, -10),
                )),
                (60, -60),
            ))],
            Position {
                x: -10,
                y: 10,
                z: 0,
                is_relative: false,
            },
        ),
    );
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
