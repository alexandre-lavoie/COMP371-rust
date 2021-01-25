use crate::*;

#[derive (Default)]
pub struct TransformBuilder {
    inner: Transform
}

impl TransformBuilder {
    pub fn set_position(mut self, position: [f32; 3]) -> Result<Self, &'static str> {
        self.get_inner_mut().set_position(position);

        Ok(self)
    }

    pub fn set_rotation(mut self, rotation: [f32; 3]) -> Result<Self, &'static str> {
        self.get_inner_mut().set_rotation(rotation);

        Ok(self)
    }

    pub fn set_scale(mut self, scale: [f32; 3]) -> Result<Self, &'static str> {
        self.get_inner_mut().set_scale(scale);

        Ok(self)
    }
}

impl Builder<Transform> for TransformBuilder {
    fn get_inner(&self) -> &Transform {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut Transform {
        &mut self.inner
    }

    fn build(self) -> Result<Transform, &'static str> {
        Ok(self.inner)
    }
}