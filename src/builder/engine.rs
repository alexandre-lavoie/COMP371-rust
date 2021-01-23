use crate::builder::{Builder, AttachBuilder, SceneBuilder, ControlledBuilder};
use crate::engine::{Engine, Scene};
use web_sys::{WebGlProgram, WebGl2RenderingContext};

#[derive(Default)]
pub struct EngineBuilder {
    engine: Engine
}

impl EngineBuilder {
    pub fn get_gl(&self) -> &WebGl2RenderingContext {
        self.engine.gl.as_ref().unwrap()
    }

    pub fn link_program(&self, gl: &WebGl2RenderingContext, vertex_shader: &str, fragment_shader: &str) -> WebGlProgram {
        self.engine.link_program(gl, vertex_shader, fragment_shader)
    }
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