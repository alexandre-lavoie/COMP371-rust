use crate::component::{Children, HasComponent, HasComponents};
use crate::controller::{HasControllers};
use crate::engine::Scene;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

#[derive(Default)]
pub struct Engine {
    scene_index: usize,
    scenes: Children<Scene>,
}

impl Engine {
    fn get_scene_index(&self) -> usize {
        self.scene_index
    }

    pub fn init(
        mut self,
        canvas: HtmlCanvasElement,
        gl: WebGl2RenderingContext,
    ) -> Result<(), &'static str> {
        let scene_index = self.get_scene_index();

        let scenes: &mut Children<Scene> = self.get_component_mut()?;

        let current_scene = scenes.get_mut(scene_index);

        current_scene.init(&gl);

        let f = Rc::new(RefCell::new(None));

        let gl_cell = Rc::new(RefCell::new(gl));

        let canvas_cell = Rc::new(RefCell::new(canvas));

        let last = Rc::new(RefCell::new(0.));

        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |now: f32| {
            let dt = now - *last.borrow();

            *last.borrow_mut() = now;

            self.update(dt, &*canvas_cell.borrow(), &*gl_cell.borrow())
                .unwrap();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f32)>));

        request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(())
    }

    fn update(
        &mut self,
        dt: f32,
        canvas: &HtmlCanvasElement,
        gl: &WebGl2RenderingContext,
    ) -> Result<(), &'static str> {
        self.update_components(dt);

        self.update_controllers(dt);

        let scene_index = self.get_scene_index();

        let scenes: &mut Children<Scene> = self.get_component_mut()?;

        let current_scene = scenes.get_mut(scene_index);

        current_scene.update_components(dt);

        current_scene.update_controllers(dt);

        let width = canvas.width() as f32;

        let height = canvas.height() as f32;

        current_scene.set_canvas_dimensions(width, height);

        current_scene.render(canvas, gl);

        Ok(())
    }
}

impl HasComponents for Engine {
    fn update_components(&mut self, dt: f32) {}
}

impl HasControllers for Engine {
    fn update_controllers(&mut self, dt: f32) {}
}

impl HasComponent<Children<Scene>> for Engine {
    fn attach_component(&mut self, component: Children<Scene>) {
        self.scenes = component;
    }

    fn get_component(&self) -> Result<&Children<Scene>, &'static str> {
        Ok(&self.scenes)
    }

    fn get_component_mut(&mut self) -> Result<&mut Children<Scene>, &'static str> {
        Ok(&mut self.scenes)
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) {
    web_sys::window()
        .expect("No window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register animation");
}
