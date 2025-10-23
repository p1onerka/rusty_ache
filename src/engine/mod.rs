pub mod config;
pub mod input;
pub mod scene;

use crate::engine::config::Config;
use crate::engine::scene::Scene;
use std::io::Error;

/// A trait for describing entity for main engine logic
pub trait EngineTrait {
    fn set_active_scene(&mut self, new_scene: Box<dyn Scene>) -> Result<(), Error>;
    fn render(&mut self) -> Result<(), Error>;

    fn new(config: Box<dyn Config>, scene: Box<dyn Scene>) -> Self
    where
        Self: Sized;
}

pub struct Engine {
    config: Box<dyn Config>,
    scene: Box<dyn Scene>,
}

impl EngineTrait for Engine {
    fn set_active_scene(&mut self, new_scene: Box<dyn Scene>) -> Result<(), Error> {
        self.scene = new_scene;
        // Can return Err if scene isn't found. Not implemented.
        Ok(())
    }

    fn render(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn new(config: Box<dyn Config>, scene: Box<dyn Scene>) -> Self
    where
        Self: Sized,
    {
        Engine { config, scene }
    }
}
