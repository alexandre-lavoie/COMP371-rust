use crate::render::Renderable;
use web_sys::WebGl2RenderingContext;

#[derive (Clone, Debug)]
pub struct CameraRenderer {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    viewport: [f32; 4],
    camera_matrix: [f32; 16],
    projection_matrix: [f32; 16],
    max_width: u32,
    max_height: u32,
    projection_update: bool,
}

impl Default for CameraRenderer {
    fn default() -> CameraRenderer {
        CameraRenderer {
            fov: 30.,
            aspect: 0.,
            near: 0.01,
            far: 1000.,
            viewport: [0., 0., 1., 1.],
            camera_matrix: [0f32; 16],
            projection_matrix: [0f32; 16],
            max_width: 1920u32,
            max_height: 1080u32,
            projection_update: true
        }
    }
}

impl CameraRenderer {
    pub fn set_fov(&mut self, fov: f32) {
        if self.fov != fov {
            self.projection_update = true;
        }
        
        self.fov = fov;
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        if self.aspect != aspect {
            self.projection_update = true;
        }
        
        self.aspect = aspect;
    }

    pub fn get_aspect(&self) -> f32 {
        self.aspect
    }

    pub fn set_near(&mut self, near: f32) {
        if self.near != near {
            self.projection_update = true;
        }
        
        self.near = near;
    }

    pub fn get_near(&self) -> f32 {
        self.near
    }

    pub fn set_far(&mut self, far: f32) {
        if self.far != far {
            self.projection_update = true;
        }
        
        self.far = far
    }

    pub fn get_far(&self) -> f32 {
        self.far
    }

    pub fn set_viewport(&mut self, viewport: [f32; 4]) {
        self.viewport = viewport;
    }

    pub fn set_canvas_max(&mut self, width: u32, height: u32) {
        if self.max_width != width || self.max_height != height {
            self.projection_update = true;
        }

        self.max_width = width;
        self.max_height = height;
    }

    pub fn get_viewport(&self) -> [f32; 4] {
        self.viewport
    }

    pub fn set_camera_matrix(&mut self, camera_matrix: [f32; 16]) {
        self.camera_matrix = camera_matrix;
    }

    pub fn get_camera_matrix(&self) -> [f32; 16] {
        self.camera_matrix
    }

    pub fn get_projection_matrix(&mut self) -> [f32; 16] {
        if self.projection_update {
            self.projection_update = false;

            self.update_projection_matrix();
        }

        self.projection_matrix
    }

    pub fn update_projection_matrix(&mut self) {
        self.projection_matrix = self.calculate_projection_matrix();
    }

    pub fn attach_viewport(&self, gl: &web_sys::WebGl2RenderingContext) {
        let v = self.get_viewport();

        let max_width = self.max_width;

        let max_height = self.max_height;

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

    fn calculate_projection_matrix(&self) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();

        let fov = self.get_fov();

        let aspect = self.get_aspect();

        let near = self.get_near();

        let far = self.get_far();

        mat4::perspective(&mut matrix, &fov, &aspect, &near, &far);

        matrix
    }
}

impl Renderable for CameraRenderer {
    fn init(&mut self, gl: &WebGl2RenderingContext) {
        self.projection_matrix = self.calculate_projection_matrix();
    }

    fn render(&mut self, gl: &WebGl2RenderingContext, camera: &mut CameraRenderer) {
        panic!("Should not render camera.");
    }
}