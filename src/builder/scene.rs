use crate::builder::{Builder, AttachBuilder, ObjectBuilder, CameraBuilder, ControlledBuilder};
use crate::engine::{Scene, Camera};
use crate::objects::{Object};

#[derive(Default)]
pub struct SceneBuilder {
    scene: Scene
}

impl SceneBuilder {

}

impl AttachBuilder<Camera, CameraBuilder> for SceneBuilder {
    fn attach_builder(mut self, builder: CameraBuilder) -> Self {
        self.scene.attach_camera(builder.build().unwrap());

        self
    }
}

impl<T: Object + Default + 'static> AttachBuilder<T, ObjectBuilder<T>> for SceneBuilder {
    fn attach_builder(mut self, builder: ObjectBuilder<T>) -> Self {
        self.scene.attach_object(builder.boxed().unwrap());

        self
    }
}

impl Builder<Scene> for SceneBuilder {
    fn build(self) -> Result<Scene, &'static str> {
        Ok(self.scene)
    }

    fn get_inner(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

impl ControlledBuilder<Scene> for SceneBuilder {}

