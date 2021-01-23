use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use wasm_bindgen::JsCast;

mod builder;
mod controller;
mod debug;
mod engine;
mod io;
mod objects;
mod render;
mod utils;

use builder::*;
use controller::*;
use objects::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let gl = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    let engine = EngineBuilder::default();

    let program = engine.link_program(
        &gl,
        include_str!("../data/shaders/vert.glsl"),
        include_str!("../data/shaders/frag.glsl"),
    );

    EngineBuilder::default()
        .attach_builder(
            SceneBuilder::default()
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                        .set_program(&program)
                        .set_scale([2., 2., 2.]),
                )
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                    .set_program(&program)
                    .set_position([4., 0., 0.])
                    .set_scale([2., 2., 2.]),
                )
                .attach_builder(
                    CameraBuilder::default()
                        .set_position([0., 5., 0.])
                        .attach_controller(Box::new(CameraController::default())),
                ),
        )
        .build()?
        .init(canvas, gl)?;

    Ok(())
}
