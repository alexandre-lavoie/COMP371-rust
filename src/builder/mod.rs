mod camera;
mod engine;
mod object;
mod scene;

pub use camera::*;
pub use engine::*;
pub use object::*;
pub use scene::*;

use crate::controller::{Controller, Controllable};

pub trait Builder<T: Default>: Default {
    fn get_inner(&mut self) -> &mut T;

    fn build(self) -> Result<T, &'static str>;

    fn boxed(self) -> Result<Box<T>, &'static str> {
        Ok(Box::new(self.build()?))
    }
}

pub trait AttachBuilder<U: Default, B: Builder<U> + Sized> {
    fn attach_builder(self, builder: B) -> Self;
}

pub trait DimensionBuilder<T: Default>: Builder<T> {
    fn set_position(self, position: [f32; 3]) -> Self;

    fn set_rotation(self, rotation: [f32; 3]) -> Self;

    fn set_scale(self, scale: [f32; 3]) -> Self;
}

pub trait ControlledBuilder<T: Default + Controllable>: Builder<T> {
    fn attach_controller(mut self, controller: Box<dyn Controller<T>>) -> Self {
        self.get_inner().attach_controller(controller);

        self
    }
}
