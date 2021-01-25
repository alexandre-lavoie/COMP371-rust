use crate::component::{HasComponents, Children, HasComponent};
use crate::controller::HasControllers;
use crate::model::{CameraModel, ObjectModel};
use crate::render::Renderable;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

#[derive(Default)]
pub struct Scene {
    objects: Children<Box<dyn ObjectModel>>,
    cameras: Children<Box<dyn CameraModel>>,
}

impl Scene {
    pub fn push_object(&mut self, object: Box<dyn ObjectModel>) {
        self.objects.push(object);
    }
    
    pub fn push_camera(&mut self, camera: Box<dyn CameraModel>) {
        self.cameras.push(camera);
    }

    pub fn init(&mut self, gl: &WebGl2RenderingContext) {
        for box_object in self.objects.iter_mut() {
            let object = box_object.as_mut();

            object.init(gl);
        }
    }

    pub fn set_canvas_dimensions(&mut self, width: f32, height: f32) {
        let aspect = width / height;

        for camera in self.cameras.iter_mut() {
            camera.as_mut().get_renderer_mut().set_aspect(aspect);
        }
    }

    pub fn render(&mut self, canvas: &HtmlCanvasElement, gl: &WebGl2RenderingContext) {
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);

        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        gl.clear_color(0., 0., 0., 1.);

        gl.clear_depth(1.);

        gl.enable(WebGl2RenderingContext::CULL_FACE);

        gl.cull_face(WebGl2RenderingContext::BACK);

        let width = canvas.width();

        let height = canvas.height();

        for camera in self.cameras.iter_mut() {
            camera.as_mut().get_renderer_mut().set_canvas_max(width, height);

            camera.update_matrix();
        }

        for camera in self.cameras.iter_mut() {
            let camera_renderer = camera.as_mut().get_renderer_mut();

            camera_renderer.attach_viewport(gl);

            for box_object in self.objects.iter_mut() {
                let object = box_object.as_mut();
    
                object.render(gl, camera_renderer);
            }
        }
    }
}

impl HasComponents for Scene {
    fn update_components(&mut self, dt: f32) {
        for camera in self.cameras.iter_mut() {
            camera.as_mut().update_components(dt);
        }

        for object in self.objects.iter_mut() {
            object.as_mut().update_components(dt);
        }
    }
}

impl HasControllers for Scene {
    fn update_controllers(&mut self, dt: f32) {
        for camera in self.cameras.iter_mut() {
            camera.as_mut().update_controllers(dt);
        }

        for object in self.objects.iter_mut() {
            object.as_mut().update_controllers(dt);
        }
    }
}