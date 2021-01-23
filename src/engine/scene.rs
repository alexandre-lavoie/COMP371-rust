use crate::controller::*;
use crate::engine::Camera;
use crate::io::Input;
use crate::objects::Object;
use crate::utils::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

#[derive(Default)]
pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    cameras: Vec<Camera>,
    controllers: Vec<Box<dyn Controller<Scene>>>,
}

impl Scene {
    pub fn get_objects(&self) -> &Vec<Box<dyn Object>> {
        &self.objects
    }

    pub fn get_cameras(&self) -> &Vec<Camera> {
        &self.cameras
    }

    pub fn attach_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn attach_camera(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }

    pub fn init(&mut self, gl: &WebGl2RenderingContext) {
        for object in self.objects.iter_mut() {
            object.init(gl);
        }
    }

    pub fn set_canvas_dimensions(&mut self, width: f32, height: f32) {
        for camera in self.cameras.iter_mut() {
            camera.aspect = width / height;
        }
    }

    pub fn render(&self, canvas: &HtmlCanvasElement, gl: &WebGl2RenderingContext) {
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);

        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        gl.clear_color(0., 0., 0., 1.);

        gl.clear_depth(1.);

        gl.enable(WebGl2RenderingContext::CULL_FACE);

        gl.cull_face(WebGl2RenderingContext::BACK);

        let width = canvas.width();

        let height = canvas.height();

        for camera in self.cameras.iter() {
            camera.attach_viewport(&gl, width, height);

            for box_object in self.objects.iter() {
                let object = coerce(box_object);

                object.render(gl, camera);
            }
        }
    }
}

impl Controllable for Scene {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>) {
        self.controllers.push(controller);
    }

    fn update_controllers(&mut self, input: &Input) {
        let controllers = std::mem::replace(&mut self.controllers, vec![]);

        for controller in controllers.iter() {
            crate::utils::coerce(&controller).update(self, input);
        }

        self.controllers = controllers;

        for object in self.objects.iter_mut() {
            object.update_controllers(input);
        }

        for camera in self.cameras.iter_mut() {
            camera.update_controllers(input);
        }
    }
}
