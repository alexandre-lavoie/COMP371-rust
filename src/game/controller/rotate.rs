use crate::*;
use crate::game::*;

#[derive (Default, Clone, Debug)]
pub struct Rotate {
    pub speed: f32
}

impl<T: HasControllers + HasComponent<Transform>> Controller<T> for Rotate {
    fn update(&mut self, parent: &mut T, dt: f32) {
        let transform: &mut Transform = parent.get_component_mut().unwrap();

        transform.delta_rotation([1. * self.speed, 2. * self.speed, 0.], dt);
    }
}