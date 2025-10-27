use crate::engine::scene::Scene;
use crate::engine::scene::game_object::GameObject;
use image::DynamicImage;

pub struct SceneManager {
    pub(crate) active_scene: Scene,
}

impl SceneManager {
    pub fn new(main_scene: Scene) -> Self {
        SceneManager {
            active_scene: main_scene,
        }
    }

    pub fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    pub fn init_active_scene(&self) -> Vec<(&GameObject, &DynamicImage)> {
        self.active_scene.init()
    }
}
