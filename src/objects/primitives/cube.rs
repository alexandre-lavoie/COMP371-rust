use crate::objects::Object;
use crate::render::primitives::CubeRenderer;
use crate::render::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram};
use crate::controller::*;
use crate::io::Input;
use crate::engine::Camera;
use crate::utils::*;

#[derive(Default)]
pub struct Cube {
    position: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
    renderer: Option<CubeRenderer>,
    controllers: Vec<Box<dyn Controller<Cube>>>
}

impl Object for Cube {
    fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    fn get_position(&self) -> &[f32; 3] {
        &self.position
    }

    fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.rotation = rotation;
    }

    fn get_rotation(&self) -> &[f32; 3] {
        &self.rotation
    }

    fn set_scale(&mut self, scale: [f32; 3]) {
        self.scale = scale;
    }

    fn get_scale(&self) -> &[f32; 3] {
        &self.scale
    }

    fn set_program(&mut self, program: WebGlProgram) {
        self.renderer = Some(CubeRenderer::new(program))
    }
}

impl Controllable for Cube {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>) {
        self.controllers.push(controller);
    }

    fn update_controllers(&mut self, input: &Input) {
        let controllers = std::mem::replace(&mut self.controllers, vec![]);

        for controller in controllers.iter() {
            crate::utils::coerce(&controller).update(self, input);
        }

        self.controllers = controllers;
    }
}

impl Renderable for Cube {
    fn init(&mut self, gl: &WebGl2RenderingContext) {
        if self.renderer.is_some() {
            self.renderer.as_mut().unwrap().init(gl);
        }
    }

    fn render(&self, gl: &WebGl2RenderingContext, camera: &Camera) {
        if self.renderer.is_some() {
            let renderer = self.renderer.as_ref().unwrap();

            let program = renderer.get_program();

            Object::__render(self, gl, program, camera);

            renderer.render(gl, camera);
        }
    }
}
