mod camera;
mod scene;
mod transform;

use crate::controller::*;
use crate::io::Input;
use crate::objects::Object;
pub use camera::*;
pub use scene::*;
pub use transform::*;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub struct Engine {
    input: Option<Input>,
    scene_index: usize,
    scenes: Vec<Scene>,
    controllers: Vec<Box<dyn Controller<Engine>>>,
}

impl Default for Engine {
    fn default() -> Engine {
        Engine {
            input: None,
            scene_index: 0,
            scenes: vec![],
            controllers: vec![],
        }
    }
}

impl Engine {
    pub fn init(mut self, canvas: HtmlCanvasElement, gl: WebGl2RenderingContext) -> Result<(), &'static str> {
        self.input = Some(Input::new(&canvas));

        self.get_current_scene_mut().init(&gl);

        let f = Rc::new(RefCell::new(None));

        let gl_cell = Rc::new(RefCell::new(gl));

        let canvas_cell = Rc::new(RefCell::new(canvas));

        let last = Rc::new(RefCell::new(0.));

        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |now: f32| {
            let dt = now - *last.borrow();

            *last.borrow_mut() = now;

            self.update(dt, &*canvas_cell.borrow(), &*gl_cell.borrow()).unwrap();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f32)>));

        request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(())
    }

    fn update(&mut self, dt: f32, canvas: &HtmlCanvasElement, gl: &WebGl2RenderingContext) -> Result<(), &'static str> {
        let input = self.input.as_ref().unwrap().clone();

        self.update_controllers(dt, &input);

        let current_scene = self.get_current_scene_mut();

        current_scene.update_controllers(dt, &input);

        let width = canvas.width() as f32;

        let height = canvas.height() as f32;

        current_scene.set_canvas_dimensions(width, height);

        current_scene.render(canvas, gl);

        Ok(())
    }

    pub fn attach_scene(&mut self, scene: Scene) {
        self.scenes.push(scene)
    }

    fn get_current_scene(&self) -> &Scene {
        &self.scenes[self.scene_index]
    }

    fn get_current_scene_mut(&mut self) -> &mut Scene {
        &mut self.scenes[self.scene_index]
    }

    fn get_objects(&self) -> &Vec<Box<dyn Object>> {
        let scene = self.get_current_scene();

        return scene.get_objects();
    }

    fn get_cameras(&self) -> &Vec<Camera> {
        let scene = self.get_current_scene();

        return scene.get_cameras();
    }
}

impl Controllable for Engine {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>) {
        self.controllers.push(controller);
    }

    fn update_controllers(&mut self, dt: f32, input: &Input) {
        let controllers = std::mem::replace(&mut self.controllers, vec![]);

        for controller in controllers.iter() {
            crate::utils::coerce(&controller).update(self, dt, input);
        }

        self.controllers = controllers;

        self.get_current_scene_mut().update_controllers(dt, input);
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) {
    web_sys::window()
        .expect("No window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register animation");
}
