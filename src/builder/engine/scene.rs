use crate::*;

#[derive (Default)]
pub struct SceneBuilder {
    inner: Scene
}

impl Builder<Scene> for SceneBuilder {
    fn get_inner(&self) -> &Scene {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut Scene {
        &mut self.inner
    }

    fn build(self) -> Result<Scene, &'static str> {
        Ok(self.inner)
    }
}

impl<T: HasComponents + ObjectModel + 'static> AttachBuilder<T, ObjectBuilder<T>, Scene> for SceneBuilder {
    fn attach_builder(mut self, builder: ObjectBuilder<T>) -> Result<Self, &'static str> {
        self.get_inner_mut().push_object(builder.boxed()?);

        Ok(self)
    }
}

impl<T: CameraModel + HasComponents + 'static> AttachBuilder<T, CameraBuilder<T>, Scene> for SceneBuilder {
    fn attach_builder(mut self, builder: CameraBuilder<T>) -> Result<Self, &'static str> {
        self.get_inner_mut().push_camera(builder.boxed()?);

        Ok(self)
    }
}