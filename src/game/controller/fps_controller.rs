use crate::game::*;
use crate::*;

#[derive(Default, Clone, Debug)]
pub struct FPSController {}

impl<T: HasControllers + HasComponent<Input> + HasComponent<Transform>> Controller<T>
    for FPSController
{
    fn update(&mut self, parent: &mut T, dt: f32) {
        let input: &Input = parent.get_component().unwrap();
        let mouse = input.get_mouse();

        let dx = mouse.get_dx();
        let dy = mouse.get_dy();

        if mouse.is_down(MouseButton::PRIMARY) {
            let transform: &mut Transform = parent.get_component_mut().unwrap();

            transform.delta_rotation([dy as f32, dx as f32, 0.], dt / 2.0);
        }

        let input: &Input = parent.get_component().unwrap();
        let keyboard = input.get_keyboard();
        let transform: &Transform = parent.get_component().unwrap();

        let dx = if keyboard.is_down(KeyboardKey::FORWARD) {
            let dx = transform.get_forward();

            [dx[0], 0., dx[2]]
        } else if keyboard.is_down(KeyboardKey::BACKWARD) {
            let dx = transform.get_forward();
            
            [-dx[0], 0., -dx[2]]
        } else {
            [0f32; 3]
        };

        let dy = if keyboard.is_down(KeyboardKey::UP) {
            [0., 1., 0.]
        } else if keyboard.is_down(KeyboardKey::DOWN) {
            [0., -1., 0.]
        } else {
            [0.; 3]
        };

        let dz = if keyboard.is_down(KeyboardKey::LEFT) {
            let dx = transform.get_left();

            [dx[0], 0., dx[2]]
        } else if keyboard.is_down(KeyboardKey::RIGHT) {
            let dx = transform.get_left();
            
            [-dx[0], 0., -dx[2]]
        } else {
            [0f32; 3]
        };

        let mut dp = [
            dx[0] + dy[0] + dz[0],
            dx[1] + dy[1] + dz[1],
            dx[2] + dy[2] + dz[2],
        ];

        let dp_copy = dp.clone();

        vec3::norm(&mut dp, &dp_copy);

        let transform: &mut Transform = parent.get_component_mut().unwrap();

        transform.delta_position(
            [
                dx[0] + dy[0] + dz[0],
                dx[1] + dy[1] + dz[1],
                dx[2] + dy[2] + dz[2],
            ],
            dt,
        );
    }
}
