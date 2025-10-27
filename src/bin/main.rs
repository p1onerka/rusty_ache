use rusty_ache::Resolution;
use rusty_ache::engine::config::{Config, EngineConfig};
use rusty_ache::engine::scene::Scene;
use rusty_ache::engine::scene::game_object::position::Position;
use rusty_ache::engine::{Engine, GameEngine};

fn main() {
    let mut engine = GameEngine::new(
        Box::new(EngineConfig::new(Resolution::new(200, 200))),
        Scene::new(
            Vec::new(),
            Vec::new(),
            Position {
                x: 0,
                y: 0,
                z: 0,
                is_relative: false,
            },
        ),
    );
    engine.render().unwrap()
}
