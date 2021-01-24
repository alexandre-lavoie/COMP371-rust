pub mod shader;
pub mod primitives;
mod object;
pub mod buffer;

pub use primitives::*;
pub use buffer::Buffers;
pub use object::ObjectRenderer;

use crate::engine::Camera;

use web_sys::{WebGl2RenderingContext};

pub trait Renderable {
    fn init(&mut self, gl: &WebGl2RenderingContext);

    fn render(&mut self, gl: &WebGl2RenderingContext, camera: &mut Camera);
}