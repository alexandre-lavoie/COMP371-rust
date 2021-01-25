use crate::*;
use web_sys::WebGlProgram;

pub trait ObjectModel: HasComponents + HasControllers + HasComponent<Transform> + HasComponent<Shader> {
    fn init_renderer(&mut self, gl: &web_sys::WebGl2RenderingContext);

    fn render_renderer(&mut self, gl: &web_sys::WebGl2RenderingContext, camera: &mut CameraRenderer);

    fn get_program(&self) -> &WebGlProgram;
    
    fn init(&mut self, gl: &web_sys::WebGl2RenderingContext) {
        self.init_renderer(gl);
    }

    fn render(&mut self, gl: &web_sys::WebGl2RenderingContext, camera: &mut CameraRenderer) {
        let projection_matrix = camera.get_projection_matrix();
        let transform: &Transform = self.get_component().unwrap();
        let world_matrix = transform.get_matrix();
        let view_matrix = camera.get_camera_matrix();
        let mut normal_matrix = mat4::new_identity::<f32>();
        mat4::inv(&mut normal_matrix, &world_matrix);
        let matrix_clone = normal_matrix.clone();
        mat4::transpose(&mut normal_matrix, &matrix_clone);

        let program = self.get_program();

        gl.use_program(Some(&program));
        
        let projection = gl.get_uniform_location(program, "u_projection");
        gl.uniform_matrix4fv_with_f32_array(projection.as_ref(), false, &projection_matrix);

        let view = gl.get_uniform_location(program, "u_view");
        gl.uniform_matrix4fv_with_f32_array(view.as_ref(), false, &view_matrix);

        let world = gl.get_uniform_location(program, "u_world");
        gl.uniform_matrix4fv_with_f32_array(world.as_ref(), false, &world_matrix);

        let normal = gl.get_uniform_location(program, "u_normal");
        gl.uniform_matrix4fv_with_f32_array(normal.as_ref(), false, &normal_matrix);

        self.render_renderer(gl, camera);
    }
}