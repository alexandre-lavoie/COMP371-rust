use crate::*;
use crate::game::*;
use web_sys::{WebGlProgram, WebGl2RenderingContext};

#[derive (Default, Clone, Debug)]
pub struct Cube {
    transform: Transform,
    renderer: CubeRenderer,
    rotate: Rotate
}

impl ObjectModel for Cube {
    fn init_renderer(&mut self, gl: &WebGl2RenderingContext) {
        self.renderer.init(gl);
    }

    fn render_renderer(&mut self, gl: &web_sys::WebGl2RenderingContext, camera: &mut CameraRenderer) {
        self.renderer.render(gl, camera);
    }

    fn get_program(&self) -> &WebGlProgram {
        let shader: &Shader = self.get_component().unwrap();

        shader.get_program()
    }
}

impl HasComponents for Cube {
    fn update_components(&mut self, dt: f32) {
        self.transform.update(dt);
        self.renderer.update_components(dt);
    }
}

impl HasControllers for Cube {
    fn update_controllers(&mut self, dt: f32) {
        let mut clone = self.rotate.clone();

        clone.update(self, dt);

        self.rotate = clone;
    }
}

impl HasController<Cube, Rotate> for Cube {
    fn attach_controller(&mut self, controller: Rotate) {
        self.rotate = controller;
    }

    fn get_controller(&self) -> &Rotate {
        &self.rotate
    }
}

impl HasComponent<Shader> for Cube {
    fn attach_component(&mut self, shader: Shader) {
        self.renderer.attach_component(shader);
    }

    fn get_component(&self) -> Result<&Shader, &'static str> {
        self.renderer.get_component()
    }

    fn get_component_mut(&mut self) -> Result<&mut Shader, &'static str> {
        self.renderer.get_component_mut()
    }
}

impl HasComponent<Transform> for Cube {
    fn attach_component(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn get_component(&self) -> Result<&Transform, &'static str> {
        Ok(&self.transform)
    }

    fn get_component_mut(&mut self) -> Result<&mut Transform, &'static str> {
        Ok(&mut self.transform)
    }
}