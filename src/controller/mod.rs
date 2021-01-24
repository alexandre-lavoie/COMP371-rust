use crate::io::Input;

pub trait Controllable {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>)
    where
        Self: Sized;

    fn update_controllers(&mut self, dt: f32, input: &Input);
}

pub trait Controller<T: Controllable> {
    fn update(&self, parent: &mut T, dt: f32, input: &Input);
}