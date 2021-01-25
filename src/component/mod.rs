mod children;
mod input;
mod shader;
mod transform;

pub use children::*;
pub use input::*;
pub use shader::*;
pub use transform::*;

pub trait HasComponents {
    fn update_components(&mut self, dt: f32);
}

impl<U: HasComponents> HasComponents for Box<U> {
    fn update_components(&mut self, dt: f32) {
        self.as_mut().update_components(dt);
    }
}

pub trait HasComponent<T: Component>: HasComponents {
    fn attach_component(&mut self, controller: T);

    fn get_component(&self) -> Result<&T, &'static str>;

    fn get_component_mut(&mut self) -> Result<&mut T, &'static str>;
}

pub trait Component {
    fn update(&mut self, dt: f32);
}
