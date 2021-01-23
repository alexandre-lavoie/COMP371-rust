use crate::engine::Camera;
use crate::render::{Buffers, ObjectRenderer, Renderable};
use web_sys::WebGlProgram;

pub struct CubeRenderer {
    program: WebGlProgram,
    buffers: Option<Buffers>
}

impl CubeRenderer {
    pub fn new(program: WebGlProgram) -> Self {
        CubeRenderer {
            program: program.clone(),
            buffers: None
        }
    }
}

impl ObjectRenderer for CubeRenderer {
    fn get_buffers(&self) -> &Option<Buffers> {
        &self.buffers
    }

    fn get_program(&self) -> &WebGlProgram {
        &self.program
    }

    fn get_index_count(&self) -> usize {
        INDICIES.len()
    }
}

impl Renderable for CubeRenderer {
    fn init(&mut self, gl: &web_sys::WebGl2RenderingContext) {
        self.buffers = Some(Buffers::new(gl, &VERTICIES, &INDICIES, &NORMALS).unwrap());
    }

    fn render(&self, gl: &web_sys::WebGl2RenderingContext, _camera: &Camera) {
        ObjectRenderer::__render(self, gl);
    }
}

static VERTICIES: [f32; 72] = [
    // Front face
    -1.0, -1.0, 1.0, //
    1.0, -1.0, 1.0, //
    1.0, 1.0, 1.0, //
    -1.0, 1.0, 1.0, //
    // Back face
    -1.0, -1.0, -1.0, //
    -1.0, 1.0, -1.0, //
    1.0, 1.0, -1.0, //
    1.0, -1.0, -1.0, //
    // Top face
    -1.0, 1.0, -1.0, //
    -1.0, 1.0, 1.0, //
    1.0, 1.0, 1.0, //
    1.0, 1.0, -1.0, //
    // Bottom face
    -1.0, -1.0, -1.0, //
    1.0, -1.0, -1.0, //
    1.0, -1.0, 1.0, //
    -1.0, -1.0, 1.0, //
    // Right face
    1.0, -1.0, -1.0, //
    1.0, 1.0, -1.0, //
    1.0, 1.0, 1.0, //
    1.0, -1.0, 1.0, //
    // Left face
    -1.0, -1.0, -1.0, //
    -1.0, -1.0, 1.0, //
    -1.0, 1.0, 1.0, //
    -1.0, 1.0, -1.0, //
];

static INDICIES: [u16; 36] = [
    // Front face
    0, 1, 2, 0, 2, 3, //
    // Back face
    4, 5, 6, 4, 6, 7, //
    // Top face
    8, 9, 10, 8, 10, 11, //
    // Bottom face
    12, 13, 14, 12, 14, 15, //
    // Right face
    16, 17, 18, 16, 18, 19, //
    // Left face
    20, 21, 22, 20, 22, 23, //
];

static NORMALS: [f32; 72] = [
    // Front face
    0.0, 0.0, 1.0, //
    0.0, 0.0, 1.0, //
    0.0, 0.0, 1.0, //
    0.0, 0.0, 1.0, //
    // Back face
    0.0, 0.0, -1.0, //
    0.0, 0.0, -1.0, //
    0.0, 0.0, -1.0, //
    0.0, 0.0, -1.0, //
    // Top face
    0.0, 1.0, 0.0, //
    0.0, 1.0, 0.0, //
    0.0, 1.0, 0.0, //
    0.0, 1.0, 0.0, //
    // Bottom face
    0.0, -1.0, 0.0, //
    0.0, -1.0, 0.0, //
    0.0, -1.0, 0.0, //
    0.0, -1.0, 0.0, //
    // Right face
    1.0, 0.0, 0.0, //
    1.0, 0.0, 0.0, //
    1.0, 0.0, 0.0, //
    1.0, 0.0, 0.0, //
    // Left face
    -1.0, 0.0, 0.0, //
    -1.0, 0.0, 0.0, //
    -1.0, 0.0, 0.0, //
    -1.0, 0.0, 0.0, //
];
