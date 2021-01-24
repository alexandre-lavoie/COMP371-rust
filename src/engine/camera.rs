use mat4;
use web_sys::WebGl2RenderingContext;
use crate::controller::*;
use crate::engine::{normalize_rotation, Transform};

pub struct Camera {
    position: [f32; 3],
    rotation: [f32; 3],
    viewport: [f32; 4],
    fov: f32,
    near: f32,
    far: f32,
    aspect: f32,
    controllers: Vec<Box<dyn Controller<Camera>>>,
    camera_matrix: [f32; 16],
    projection: [f32; 16],
    refresh_camera_matrix: bool,
    refresh_projection: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: [0., 0., 0.],
            rotation: [0., 0., 0.],
            viewport: [0., 0., 1., 1.],
            fov: 30.,
            near: 0.01,
            far: 1000.,
            aspect: 0.,
            controllers: vec![],
            camera_matrix: [0.; 16],
            projection: [0.; 16],
            refresh_camera_matrix: true,
            refresh_projection: true
        }
    }
}

impl Camera {
    pub fn attach_viewport(&self, gl: &WebGl2RenderingContext, max_width: u32, max_height: u32) {
        let v = self.viewport;

        let x = if v[0] > 0. && v[0] <= 1. {
            max_width as f32 * v[0]
        } else {
            v[0]
        };

        let y = if v[1] > 0. && v[1] <= 1. {
            max_height as f32 * v[1]
        } else {
            v[1]
        };

        let width = if v[2] <= 1. {
            max_width as f32 * v[2]
        } else {
            v[2]
        };

        let height = if v[3] <= 1. {
            max_height as f32 * v[3]
        } else {
            v[3]
        };

        gl.viewport(x as i32, y as i32, width as i32, height as i32);
    }

    fn calculate_camera_matrix(&self) -> [f32; 16] {
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

        matrix
    }

    fn calculate_projection_matrix(&self) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();

        mat4::perspective(&mut matrix, &self.fov, &self.aspect, &self.near, &self.far);

        matrix
    }

    pub fn get_camera_matrix(&mut self) -> [f32; 16] {
        if self.refresh_camera_matrix {
            self.refresh_camera_matrix = false;

            self.camera_matrix = self.calculate_camera_matrix();
        }
            
        self.camera_matrix
    }

    pub fn get_projection_matrix(&mut self) -> [f32; 16] {
        if self.refresh_projection {
            self.refresh_projection = false;

            self.projection = self.calculate_projection_matrix()
        }

        self.projection
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.refresh_projection = true;
        self.fov = fov;
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.refresh_projection = true;
        self.aspect = aspect;
    }

    pub fn set_near(&mut self, near: f32) {
        self.refresh_projection = true;
        self.near = near;
    }

    pub fn set_far(&mut self, far: f32) {
        self.refresh_projection = true;
        self.far = far;
    }

    pub fn set_viewport(&mut self, viewport: [f32; 4]) {
        self.refresh_projection = true;
        self.viewport = viewport;
    }
}

impl Transform for Camera {
    fn set_position(&mut self, position: [f32; 3]) {
        self.refresh_camera_matrix = true;
        self.position = position;
    }

    fn get_position(&self) -> &[f32; 3] {
        &self.position
    }

    fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.refresh_camera_matrix = true;
        self.rotation = normalize_rotation(&rotation);
    }

    fn get_rotation(&self) -> &[f32; 3] {
        &self.rotation
    }

    fn set_scale(&mut self, _scale: [f32; 3]) {}

    fn get_scale(&self) -> &[f32; 3] {
        &[1., 1., 1.]
    }

    fn get_matrix(&mut self) -> [f32; 16] {
        self.calculate_matrix()
    }
}

impl Controllable for Camera {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>) {
        self.controllers.push(controller);
    }

    fn update_controllers(&mut self, dt: f32, input: &crate::io::Input) {
        let controllers = std::mem::replace(&mut self.controllers, vec![]);

        for controller in controllers.iter() {
            crate::utils::coerce(&controller).update(self, dt, input);
        }

        self.controllers = controllers;
    }
}