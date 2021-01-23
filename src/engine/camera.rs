use mat4;
use web_sys::WebGl2RenderingContext;
use crate::controller::*;

pub struct Camera {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect: f32,
    controllers: Vec<Box<dyn Controller<Camera>>>
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: [0., 0., 0.],
            rotation: [0., 0., 0.],
            x: 0.,
            y: 0.,
            width: 1.,
            height: 1.,
            fov: 30.,
            near: 0.01,
            far: 1000.,
            aspect: 0.,
            controllers: vec![]
        }
    }
}

impl Camera {
    pub fn get_matrix(&self) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();

        let mut matrix_clone = matrix.clone();

        mat4::rotate(
            &mut matrix,
            &matrix_clone,
            &(180. - self.rotation[0]).to_radians(),
            &self.rotation[1].to_radians(),
            &self.rotation[2].to_radians(),
        );

        matrix_clone = matrix.clone();

        mat4::translate(
            &mut matrix,
            &matrix_clone,
            &[-self.position[0], -self.position[1], -self.position[2]],
        );

        return matrix;
    }

    pub fn attach_viewport(&self, gl: &WebGl2RenderingContext, max_width: u32, max_height: u32) {
        let x = if self.x > 0. && self.x <= 1. {
            max_width as f32 * self.x
        } else {
            self.x
        };

        let y = if self.y > 0. && self.y <= 1. {
            max_height as f32 * self.y
        } else {
            self.y
        };

        let width = if self.width <= 1. {
            max_width as f32 * self.width
        } else {
            self.width
        };

        let height = if self.height <= 1. {
            max_height as f32 * self.height
        } else {
            self.height
        };

        gl.viewport(x as i32, y as i32, width as i32, height as i32);
    }

    pub fn projection_matrix(&self) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();

        mat4::perspective(&mut matrix, &self.fov, &self.aspect, &self.near, &self.far);

        return matrix;
    }
}

impl Controllable for Camera {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>) {
        self.controllers.push(controller);
    }

    fn update_controllers(&mut self, input: &crate::io::Input) {
        let controllers = std::mem::replace(&mut self.controllers, vec![]);

        for controller in controllers.iter() {
            crate::utils::coerce(&controller).update(self, input);
        }

        self.controllers = controllers;
    }
}