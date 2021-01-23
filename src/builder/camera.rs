use crate::builder::{DimensionBuilder, Builder, ControlledBuilder};
use crate::engine::Camera;

#[derive(Default)]
pub struct CameraBuilder {
    camera: Camera
}

impl CameraBuilder {
    pub fn set_fov(mut self, fov: f32) -> Self {
        self.camera.fov = fov;

        self
    }

    pub fn set_near(mut self, near: f32) -> Self {
        self.camera.near = near;

        self
    }

    pub fn set_far(mut self, far: f32) -> Self {
        self.camera.far = far;

        self
    }

    pub fn set_viewport(mut self, viewport: [f32; 4]) -> Self {
        self.camera.x = viewport[0];
        self.camera.y = viewport[1];
        self.camera.width = viewport[2];
        self.camera.height = viewport[3];

        self
    }
}

impl DimensionBuilder<Camera> for CameraBuilder {
    fn set_position(mut self, position: [f32; 3]) -> Self {
        self.camera.position = position;

        self
    }

    fn set_rotation(mut self, rotation: [f32; 3]) -> Self {
        self.camera.rotation = rotation;

        self
    }

    fn set_scale(self, _scale: [f32; 3]) -> Self {
        self
    }
}

impl Builder<Camera> for CameraBuilder {
    fn build(self) -> Result<Camera, &'static str> {
        Ok(self.camera)
    }

    fn get_inner(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

impl ControlledBuilder<Camera> for CameraBuilder {}