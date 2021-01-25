use crate::component::{HasComponent, Shader};
use crate::render::{Buffers, Renderable};
use web_sys::WebGl2RenderingContext;

pub trait ObjectRenderer: Renderable + HasComponent<Shader> {
    fn get_buffers(&self) -> &Option<Buffers>;

    fn get_index_count(&self) -> usize;

    fn __render(&self, gl: &WebGl2RenderingContext) {
        let shader: &Shader = self.get_component().unwrap();

        let program = shader.get_program();

        let buffers = self.get_buffers().as_ref().unwrap();

        gl.use_program(Some(&program));

        if buffers.vertex.is_some() {
            gl.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                buffers.vertex.as_ref(),
            );

            let a_position = gl.get_attrib_location(program, "a_position");

            if a_position >= 0 {
                gl.vertex_attrib_pointer_with_i32(
                    a_position as u32,
                    3,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
                gl.enable_vertex_attrib_array(a_position as u32);
            }
        }

        if buffers.normal.is_some() {
            gl.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                buffers.normal.as_ref(),
            );

            let a_normal = gl.get_attrib_location(program, "a_normal");

            if a_normal >= 0 {
                gl.vertex_attrib_pointer_with_i32(
                    a_normal as u32,
                    3,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
                gl.enable_vertex_attrib_array(a_normal as u32);
            }
        }

        gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            buffers.index.as_ref(),
        );

        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.get_index_count() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        )
    }
}
