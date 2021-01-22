use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

mod camera;
mod cube;
mod engine;
mod object;
mod shader;

use camera::Camera;
use cube::Cube;
use engine::Engine;
use shader::{compile_shader, link_program};

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("./shaders/vert.glsl"),
    )?;

    let frag_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        include_str!("./shaders/frag.glsl"),
    )?;

    let program = link_program(&gl, &vert_shader, &frag_shader)?;

    let mut engine = Engine::new(canvas, gl);

    engine.attach_camera(Camera::new([0., 5., 0.], [0., 0., 0.], [0., 0., 1., 1.]));

    engine.attach_camera(Camera::new([10., 5., 0.], [0., 0., 0.], [0., 0.75, 0.25, 0.25]));

    engine.attach_object(Box::new(Cube::new(
        program.clone(),
        [0., 0., 0.],
        [0., 0., 0.],
        [5., 1., 5.],
    )));

    engine.attach_object(Box::new(Cube::new(
        program.clone(),
        [2., 1., 2.],
        [0., 0., 0.],
        [3., 1., 3.],
    )));

    engine.init()?;

    let f = Rc::new(RefCell::new(None));

    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f32| {
        engine.update(time);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f32)>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

pub fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) {
    web_sys::window()
        .expect("No window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register animation");
}
