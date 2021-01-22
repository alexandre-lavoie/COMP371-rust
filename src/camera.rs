use mat4;
use web_sys::WebGl2RenderingContext;

#[derive(Copy, Clone)]
pub struct Camera {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn new(position: [f32; 3], rotation: [f32; 3], viewport: [f32; 4]) -> Self {
        Camera {
            position: position,
            rotation: rotation,
            x: viewport[0],
            y: viewport[1],
            width: viewport[2],
            height: viewport[3],
        }
    }

    pub fn forward(&self) -> [f32; 3] {
        let mut matrix = mat4::new_identity::<f32>();

        let matrix_clone = matrix.clone();

        mat4::rotate(
            &mut matrix,
            &matrix_clone,
            &(180. - self.rotation[0]).to_radians(),
            &self.rotation[1].to_radians(),
            &self.rotation[2].to_radians(),
        );

        return [matrix[0], matrix[5], matrix[10]];
    }

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

    pub fn projection_matrix(&self, aspect: f32) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();
        let fov = 30.0;
        let near = 0.01;
        let far = 1000.0;

        mat4::perspective(&mut matrix, &fov, &aspect, &near, &far);

        return matrix;
    }
}
