mod camera;
mod scene;

use crate::controller::*;
use crate::io::Input;
use crate::objects::Object;
use crate::render::shader::*;
pub use camera::*;
pub use scene::*;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram};

pub struct Engine {
    canvas: Option<HtmlCanvasElement>,
    pub gl: Option<WebGl2RenderingContext>,
    input: Option<Input>,
    scene_index: usize,
    scenes: Vec<Scene>,
    controllers: Vec<Box<dyn Controller<Engine>>>,
}

impl Default for Engine {
    fn default() -> Engine {
        Engine {
            canvas: None,
            gl: None,
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

        self.gl = Some(gl);

        self.canvas = Some(canvas);

        let f = Rc::new(RefCell::new(None));

        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_: f32| {
            self.update().unwrap();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f32)>));

        request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(())
    }

    fn update(&mut self) -> Result<(), &'static str> {
        let input = self.input.as_ref().unwrap().clone();

        self.update_controllers(&input);

        let width = self.canvas.as_ref().unwrap().width() as f32;

        let height = self.canvas.as_ref().unwrap().height() as f32;

        let scene_mut = self.get_current_scene_mut();

        scene_mut.update_controllers(&input);

        scene_mut.set_canvas_dimensions(width, height);

        self.get_current_scene().render(self.canvas.as_ref().unwrap(), self.gl.as_ref().unwrap());

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

    pub fn link_program(&self, gl: &WebGl2RenderingContext, vertex_shader: &str, fragment_shader: &str) -> WebGlProgram {
        let v_result = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_shader);

        let vertex_shader = match v_result {
            Ok(v) => v,
            Err(s) => panic!("{:?}", s),
        };

        let f_result = compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            fragment_shader,
        );

        let fragment_shader = match f_result {
            Ok(f) => f,
            Err(s) => panic!("{:?}", s),
        };

        match link_program(&gl, &vertex_shader, &fragment_shader) {
            Ok(p) => p,
            Err(s) => panic!("{:?}", s),
        }
    }
}

impl Controllable for Engine {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>) {
        self.controllers.push(controller);
    }

    fn update_controllers(&mut self, input: &Input) {
        let controllers = std::mem::replace(&mut self.controllers, vec![]);

        for controller in controllers.iter() {
            crate::utils::coerce(&controller).update(self, input);
        }

        self.controllers = controllers;

        self.get_current_scene_mut().update_controllers(input);
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) {
    web_sys::window()
        .expect("No window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register animation");
}
