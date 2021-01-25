mod builder;
mod component;
mod controller;
mod debug;
mod engine;
mod io;
mod model;
mod render;
mod utils;

pub use builder::*;
pub use component::*;
pub use controller::*;
pub use debug::*;
pub use engine::*;
pub use io::*;
pub use model::*;
pub use render::*;
pub use utils::*;

///
/// Start of the game. Should be move in it's own repo.
/// 

mod game;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

    game::main(canvas, gl)?;

    Ok(())
}
