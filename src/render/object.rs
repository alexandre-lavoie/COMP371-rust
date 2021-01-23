use crate::render::{Buffers, Renderable};
use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub trait ObjectRenderer: Renderable {
    fn get_buffers(&self) -> &Option<Buffers>;
    fn get_program(&self) -> &WebGlProgram;
    fn get_index_count(&self) -> usize;

    fn __render(&self, gl: &WebGl2RenderingContext) {
        let program = self.get_program();

        let buffers = self.get_buffers().as_ref().unwrap();

        if buffers.vertex.is_some() {
            gl.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                buffers.vertex.as_ref(),
            );

            // let a_position = gl.get_attrib_location(program, "a_position");

            //if a_position >= 0 {
                // crate::log!("Position");

                gl.vertex_attrib_pointer_with_i32(
                    0, //a_position as u32,
                    3,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
    
                gl.enable_vertex_attrib_array(0);
            //}

        }

        if buffers.normal.is_some() {
            gl.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                buffers.normal.as_ref(),
            );

            // let a_normal = gl.get_attrib_location(program, "a_normal");

            // if a_normal >= 0 {
                // crate::log!("Normal");

                gl.vertex_attrib_pointer_with_i32(
                    1, // a_normal as u32
                    3,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
    
                gl.enable_vertex_attrib_array(1);
            // }
        }

        // crate::log!("Index");
        

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
