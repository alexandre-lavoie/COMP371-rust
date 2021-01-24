use crate::builder::{Builder, AttachBuilder, SceneBuilder, ControlledBuilder};
use crate::engine::{Engine, Scene};

#[derive(Default)]
pub struct EngineBuilder {
    engine: Engine
}

impl EngineBuilder {
}

impl AttachBuilder<Scene, SceneBuilder> for EngineBuilder {
    fn attach_builder(mut self, builder: SceneBuilder) -> Self {
        self.engine.attach_scene(builder.build().unwrap());

        self
    }
}

impl Builder<Engine> for EngineBuilder {
    fn build(self) -> Result<Engine, &'static str> {
        Ok(self.engine)
    }

    fn get_inner(&mut self) -> &mut Engine {
        &mut self.engine
    }
}

impl ControlledBuilder<Engine> for EngineBuilder {}