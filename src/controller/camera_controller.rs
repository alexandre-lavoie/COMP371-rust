use crate::controller::*;
use crate::engine::*;
use crate::io::*;

#[derive(Default)]
pub struct CameraController {}

impl Controller<Camera> for CameraController {
    fn update(&self, camera: &mut Camera, input: &Input) {
        let mouse = input.get_mouse();

        if mouse.is_down(MouseButton::PRIMARY) {
            camera.rotation[1] -= mouse.get_dx() as f32 / 50.0;
            camera.rotation[0] -= mouse.get_dy() as f32 / 50.0;
        }
    }
}