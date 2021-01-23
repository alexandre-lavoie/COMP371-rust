mod primitives;

use crate::controller::Controllable;
use crate::engine::Camera;
use crate::log;
use crate::render::Renderable;
use mat4;
pub use primitives::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub trait Object: Renderable + Controllable {
    fn set_position(&mut self, position: [f32; 3]);

    fn get_position(&self) -> &[f32; 3];

    fn set_rotation(&mut self, rotation: [f32; 3]);

    fn get_rotation(&self) -> &[f32; 3];

    fn set_scale(&mut self, scale: [f32; 3]);

    fn get_scale(&self) -> &[f32; 3];

    fn set_program(&mut self, program: WebGlProgram);

    fn get_matrix(&self) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();

        let mut matrix_clone = matrix.clone();

        mat4::scale(&mut matrix, &matrix_clone, &self.get_scale());

        matrix_clone = matrix.clone();

        let rotation = self.get_rotation();

        mat4::rotate(
            &mut matrix,
            &matrix_clone,
            &rotation[0].to_radians(),
            &rotation[1].to_radians(),
            &rotation[2].to_radians(),
        );

        matrix_clone = matrix.clone();

        mat4::translate(&mut matrix, &matrix_clone, &self.get_position());

        return matrix;
    }

    fn __render(&self, gl: &WebGl2RenderingContext, program: &WebGlProgram, camera: &Camera) {
        gl.use_program(Some(&program));

        let projection_matrix = camera.projection_matrix();
        // crate::log!("Projection {:?}", projection_matrix);
        let projection = gl.get_uniform_location(program, "u_projection");
        gl.uniform_matrix4fv_with_f32_array(projection.as_ref(), false, &projection_matrix);

        let view_matrix = camera.get_matrix();
        // crate::log!("View {:?}", view_matrix);
        let view = gl.get_uniform_location(program, "u_view");
        gl.uniform_matrix4fv_with_f32_array(view.as_ref(), false, &view_matrix);

        let mut normal_matrix = mat4::new_identity::<f32>();
        // crate::log!("Normal {:?}", normal_matrix);
        mat4::inv(&mut normal_matrix, &view_matrix);
        let matrix_clone = normal_matrix.clone();
        mat4::transpose(&mut normal_matrix, &matrix_clone);
        let normal = gl.get_uniform_location(program, "u_normal");
        gl.uniform_matrix4fv_with_f32_array(normal.as_ref(), false, &normal_matrix);

        let world_matrix = self.get_matrix();
        // crate::log!("Projection {:?}", world_matrix);
        let world = gl.get_uniform_location(program, "u_world");
        gl.uniform_matrix4fv_with_f32_array(world.as_ref(), false, &world_matrix);
    }
}
