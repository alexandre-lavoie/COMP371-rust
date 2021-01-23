use crate::builder::{Builder, DimensionBuilder, ControlledBuilder};
use crate::objects::{Object};
use crate::controller::{Controllable};
use web_sys::{WebGlProgram};

#[derive(Default)]
pub struct ObjectBuilder<T: Object + Default> {
    obj: T
}

impl<T: Object + Default> ObjectBuilder<T> {
    pub fn set_program(mut self, program: &WebGlProgram) -> Self {
        self.obj.set_program(program.clone());

        self
    }
}

impl<T: Object + Default> Builder<T> for ObjectBuilder<T> {
    fn build(self) -> Result<T, &'static str> {
        Ok(self.obj)
    }

    fn get_inner(&mut self) -> &mut T {
        &mut self.obj
    }
}

impl<T: Object + Default> DimensionBuilder<T> for ObjectBuilder<T> {
    fn set_position(mut self, position: [f32; 3]) -> Self {
        self.obj.set_position(position);

        self
    }

    fn set_rotation(mut self, rotation: [f32; 3]) -> Self {
        self.obj.set_rotation(rotation);

        self
    }

    fn set_scale(mut self, scale: [f32; 3]) -> Self {
        self.obj.set_scale(scale);
        
        self
    }
}

impl<T: Object + Default + Controllable> ControlledBuilder<T> for ObjectBuilder<T> {}