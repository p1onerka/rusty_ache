use crate::engine::scene::Scene;

pub struct SceneManager {
    active_scene: Box<dyn Scene>,
}

impl SceneManager {
    pub fn new(main_scene: Box<dyn Scene>) -> Self {
        SceneManager {
            active_scene: main_scene,
        }
    }
}
