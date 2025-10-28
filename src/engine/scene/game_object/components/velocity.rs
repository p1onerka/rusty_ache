use super::*;

pub struct Velocity {
    _x: usize,
    _y: usize,
    _component_type: ComponentType,
}

impl Velocity {
    pub fn _new() -> Self {
        Velocity {
            _x: 0,
            _y: 0,
            _component_type: ComponentType::Velocity,
        }
    }
    pub fn _update(&mut self, x: usize, y: usize) {
        self._x = x;
        self._y = y;
    }
}

impl Component for Velocity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_component_type(&self) -> ComponentType {
        ComponentType::Velocity
    }
}
