use crate::object::Object;
use mat4;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub struct Cube {
    position: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
    program: WebGlProgram,
}

impl Cube {
    pub fn new(
        program: WebGlProgram,
        position: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Self {
        Cube {
            position: position,
            rotation: rotation,
            scale: scale,
            program: program,
        }
    }
}

impl Object for Cube {
    fn init(&self, gl: &WebGl2RenderingContext) -> Result<(), &'static str> {
        let vertex_vector = self.get_verticies();

        let index_vector = self.get_indicies();

        let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer")?;

        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let vertex_array = js_sys::Float32Array::view(&vertex_vector);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertex_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        };

        let index_buffer = gl.create_buffer().ok_or("failed to create buffer")?;

        gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );
        unsafe {
            let index_array = js_sys::Uint16Array::view(&index_vector);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &index_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        Ok(())
    }

    fn get_program(&self) -> &WebGlProgram {
        &self.program
    }

    fn get_verticies(&self) -> &[f32] {
        return &[
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
    }

    fn get_indicies(&self) -> &[u16] {
        return &[
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
    }

    fn get_normals(&self) -> &[f32] {
        return &[
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
    }

    fn get_matrix(&self) -> [f32; 16] {
        let mut matrix = mat4::new_identity::<f32>();

        let mut matrix_clone = matrix.clone();

        mat4::scale(&mut matrix, &matrix_clone, &self.scale);

        matrix_clone = matrix.clone();

        mat4::rotate(
            &mut matrix,
            &matrix_clone,
            &self.rotation[0].to_radians(),
            &self.rotation[1].to_radians(),
            &self.rotation[2].to_radians(),
        );

        matrix_clone = matrix.clone();

        mat4::translate(&mut matrix, &matrix_clone, &self.position);

        return matrix;
    }
}
