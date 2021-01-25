pub mod shader;
pub mod primitives;
mod camera;
mod object;
pub mod buffer;

pub use camera::*;
pub use primitives::*;
pub use buffer::Buffers;
pub use object::ObjectRenderer;
pub use shader::*;

use web_sys::{WebGl2RenderingContext};

pub trait Renderable {
    fn init(&mut self, gl: &WebGl2RenderingContext);

    fn render(&mut self, gl: &WebGl2RenderingContext, camera: &CameraRenderer);
}

pub trait RenderableClone {
    fn clone_box(&self) -> Box<dyn Renderable>;
}

impl<T: 'static + Renderable + Clone> RenderableClone for T {
    fn clone_box(&self) -> Box<dyn Renderable> {
        Box::new(self.clone())
    }
}