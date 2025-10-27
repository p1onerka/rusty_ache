use image::{ImageFormat, ImageReader};
use rusty_ache::Resolution;
use rusty_ache::engine::config::{Config, EngineConfig};
use rusty_ache::engine::scene::Scene;
use rusty_ache::engine::scene::game_object::GameObject;
use rusty_ache::engine::scene::game_object::components::sprite::Sprite;
use rusty_ache::engine::scene::game_object::position::Position;
use rusty_ache::engine::{Engine, GameEngine};
use rusty_ache::screen::{example, example_keys};

fn main() {
    let mut engine = GameEngine::new(
        Box::new(EngineConfig::new(Resolution::new(200, 200))),
        Scene::new(
            vec![
                GameObject::new(
                    vec![Box::new(Sprite::new(Some(
                        ImageReader::open("src/bin/resources/command_center.png")
                            .unwrap()
                            .decode()
                            .unwrap(),
                    )))],
                    Position {
                        x: 0,
                        y: 0,
                        z: 1,
                        is_relative: false,
                    },
                ),
                GameObject::new(
                    vec![Box::new(Sprite::new(Some(
                        ImageReader::open("src/bin/resources/command_center.png")
                            .unwrap()
                            .decode()
                            .unwrap(),
                    )))],
                    Position {
                        x: 130,
                        y: -100,
                        z: 2,
                        is_relative: false,
                    },
                ),
                GameObject::new(
                    vec![Box::new(Sprite::new(Some(
                        ImageReader::open("src/bin/resources/command_center.png")
                            .unwrap()
                            .decode()
                            .unwrap(),
                    )))],
                    Position {
                        x: 15,
                        y: -25,
                        z: 3,
                        is_relative: false,
                    },
                ),
            ],
            vec![Box::new(Sprite::new(Some(
                ImageReader::open("src/bin/resources/battlecruiser_main.png")
                    .unwrap()
                    .decode()
                    .unwrap(),
            )))],
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
