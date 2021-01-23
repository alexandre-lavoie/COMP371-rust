use web_sys::{WebGlBuffer, WebGl2RenderingContext};

#[derive(Clone, Debug)]
pub struct Buffers {
    pub vertex: Option<WebGlBuffer>,
    pub index: Option<WebGlBuffer>,
    pub normal: Option<WebGlBuffer>
}

impl Buffers {
    pub fn new(gl: &WebGl2RenderingContext, verticies: &[f32], indicies: &[u16], normals: &[f32]) -> Result<Self, &'static str> {
        Ok(
            Buffers {
                vertex: Some(Buffers::init_vertex_buffer(&gl, verticies)?),
                index: Some(Buffers::init_index_buffer(&gl, indicies)?),
                normal: Some(Buffers::init_normal_buffer(&gl, normals)?)
            }
        )
    }

    fn init_buffer_f32(gl: &WebGl2RenderingContext, target: u32, vector: &[f32]) -> Result<WebGlBuffer, &'static str> {
        let buffer = gl.create_buffer().ok_or("failed to creat buffer")?;
    
        gl.bind_buffer(target, Some(&buffer));
    
        unsafe {
            let array = js_sys::Float32Array::view(&vector);
    
            gl.buffer_data_with_array_buffer_view(
                target,
                &array,
                WebGl2RenderingContext::STATIC_DRAW
            );
        }

        gl.bind_buffer(target, Some(&buffer));
    
        Ok(buffer)
    }
    
    fn init_buffer_u16(gl: &WebGl2RenderingContext, target: u32, vector: &[u16]) -> Result<WebGlBuffer, &'static str> {
        let buffer = gl.create_buffer().ok_or("failed to creat buffer")?;
    
        gl.bind_buffer(target, Some(&buffer));
    
        unsafe {
            let array = js_sys::Uint16Array::view(&vector);
    
            gl.buffer_data_with_array_buffer_view(
                target,
                &array,
                WebGl2RenderingContext::STATIC_DRAW
            );
        }

        gl.bind_buffer(target, Some(&buffer));
    
        Ok(buffer)
    }
    
    fn init_vertex_buffer(gl: &WebGl2RenderingContext, verticies: &[f32]) -> Result<WebGlBuffer, &'static str> {
        Buffers::init_buffer_f32(gl, WebGl2RenderingContext::ARRAY_BUFFER, verticies)
    }
    
    fn init_index_buffer(gl: &WebGl2RenderingContext, indicies: &[u16]) -> Result<WebGlBuffer, &'static str> {
        Buffers::init_buffer_u16(gl, WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, indicies)
    }

    fn init_normal_buffer(gl: &WebGl2RenderingContext, normals: &[f32]) -> Result<WebGlBuffer, &'static str> {
        Buffers::init_buffer_f32(gl, WebGl2RenderingContext::ARRAY_BUFFER, normals)
    }
}