use crate::builder::*;
use crate::controller::*;
use crate::objects::*;
use crate::render::shader::*;
use crate::io::*;
use crate::engine::*;

use wasm_bindgen::prelude::*;

pub fn main(
    canvas: web_sys::HtmlCanvasElement,
    gl: web_sys::WebGl2RenderingContext,
) -> Result<(), JsValue> {
    let program = link_program_str(
        &gl,
        include_str!("../resources/shaders/vert.glsl"),
        include_str!("../resources/shaders/frag.glsl"),
    );

    EngineBuilder::default()
        .attach_builder(
            SceneBuilder::default()
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                        .set_program(&program)
                        .set_scale([5., 1., 5.])
                )
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                        .set_program(&program)
                        .set_position([-15., 0., 0.])
                        .set_scale([4., 4., 4.])
                        .attach_controller(Box::new(Rotate { speed: 1.5 }))
                )
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                        .set_program(&program)
                        .set_position([0., 0., -15.])
                        .set_scale([4., 4., 4.])
                        .attach_controller(Box::new(Rotate { speed: 2. }))
                )
                .attach_builder(
                    CameraBuilder::default()
                        .set_position([0., 5., 0.])
                        .set_rotation([10., 0., 0.])
                        .attach_controller(Box::new(MouseController::default()))
                )
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                        .set_program(&program)
                        .set_scale([2., 1., 2.])
                        .set_position([15., 0., 0.])
                        .attach_controller(Box::new(Translate::default()))
                )
                .attach_builder(
                    ObjectBuilder::<Cube>::default()
                        .set_program(&program)
                        .set_position([0., 0., 15.])
                        .set_scale([2., 2., 2.])
                        .attach_controller(Box::new(Rotate { speed: 1. }))
                )
        )
        .build()?
        .init(canvas, gl)?;

    Ok(())
}

#[derive (Default)]
struct Rotate {
    speed: f32
}

impl<T: Transform + Controllable> Controller<T> for Rotate {
    fn update(&self, obj: &mut T, dt: f32, input: &Input) {
        obj.delta_rotation([2. * self.speed, 4. * self.speed, 0. * self.speed], dt);
    }
}

#[derive (Default)]
struct Translate {}

impl<T: Transform + Controllable> Controller<T> for Translate {
    fn update(&self, obj: &mut T, dt: f32, input: &Input) {
        let position = obj.get_position().clone();

        obj.delta_position([0., 2., 0.], dt);
    }
}

#[derive(Default)]
pub struct MouseController {}

impl<T: Transform + Controllable> Controller<T> for MouseController {
    fn update(&self, obj: &mut T, dt: f32, input: &Input) {
        let mouse = input.get_mouse();

        if mouse.is_down(MouseButton::PRIMARY) {
            obj.delta_rotation([-mouse.get_dy() as f32 / 5., -mouse.get_dx() as f32 / 5., 0.], dt);
        }
    }
}