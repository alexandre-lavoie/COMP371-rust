use crate::objects::Object;
use crate::render::primitives::CubeRenderer;
use crate::render::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram};
use crate::controller::*;
use crate::io::Input;
use crate::engine::{Transform, Camera};

#[derive(Default)]
pub struct Cube {
    position: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
    renderer: Option<CubeRenderer>,
    controllers: Vec<Box<dyn Controller<Cube>>>,
    matrix: [f32; 16],
    refresh_matrix: bool
}

impl Object for Cube {
    fn set_program(&mut self, program: WebGlProgram) {
        self.renderer = Some(CubeRenderer::new(program))
    }

    fn get_program(&self) -> &WebGlProgram {
        self.renderer.as_ref().unwrap().get_program()
    }
}

impl Transform for Cube {
    fn set_position(&mut self, position: [f32; 3]) {
        self.refresh_matrix = true;
        self.position = position;
    }

    fn get_position(&self) -> &[f32; 3] {
        &self.position
    }

    fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.refresh_matrix = true;
        self.rotation = rotation;
    }

    fn get_rotation(&self) -> &[f32; 3] {
        &self.rotation
    }

    fn set_scale(&mut self, scale: [f32; 3]) {
        self.refresh_matrix = true;
        self.scale = scale;
    }

    fn get_scale(&self) -> &[f32; 3] {
        &self.scale
    }

    fn get_matrix(&mut self) -> [f32; 16] {
        if self.refresh_matrix {
            self.refresh_matrix = false;

            self.matrix = self.calculate_matrix();
        }

        self.matrix
    }
}

impl Controllable for Cube {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>) {
        self.controllers.push(controller);
    }

    fn update_controllers(&mut self, dt: f32, input: &Input) {
        let controllers = std::mem::replace(&mut self.controllers, vec![]);

        for controller in controllers.iter() {
            crate::utils::coerce(&controller).update(self, dt, input);
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

    fn render(&mut self, gl: &WebGl2RenderingContext, camera: &mut Camera) {
        if self.renderer.is_some() {
            Object::__render(self, gl, camera);

            let renderer = self.renderer.as_mut().unwrap();

            renderer.render(gl, camera);
        }
    }
}
