mod controller;
mod object;

pub use controller::*;
pub use object::*;

pub use crate::*;

use wasm_bindgen::prelude::*;

pub fn main(
    canvas: web_sys::HtmlCanvasElement,
    gl: web_sys::WebGl2RenderingContext,
) -> Result<(), JsValue> {
    let program = link_program_str(
        &gl,
        include_str!("../../resources/shaders/vert.glsl"),
        include_str!("../../resources/shaders/frag.glsl"),
    );

    let input = Input::new(&canvas);

    let shader = Shader::from(program);

    EngineBuilder::default()
        .attach_builder(
            SceneBuilder::default()
                .attach_builder(
                    CameraBuilder::<Camera>::default()
                        .attach_component(input.clone())?
                        .attach_component_builder(
                            TransformBuilder::default().set_position([0., 5., 0.])?,
                        )?,
                )?
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                        .attach_component(shader.clone())?
                        .attach_component_builder(
                            TransformBuilder::default().set_scale([5., 1., 5.])?,
                        )?,
                )?
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                        .attach_component(shader.clone())?
                        .attach_controller(Rotate { speed: 2. })?
                        .attach_component_builder(
                            TransformBuilder::default()
                                .set_position([-15., 0., 0.])?
                                .set_scale([4., 4., 4.])?,
                        )?,
                )?,
        )?
        .build()?
        .init(canvas, gl)?;

    Ok(())
}
