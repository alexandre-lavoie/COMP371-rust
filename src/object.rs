use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub trait Object {
    fn init(&self, gl: &WebGl2RenderingContext) -> Result<(), &'static str>;
    fn get_verticies(&self) -> &[f32];
    fn get_indicies(&self) -> &[u16];
    fn get_normals(&self) -> &[f32];
    fn get_program(&self) -> &WebGlProgram;
    fn get_matrix(&self) -> [f32; 16];
}