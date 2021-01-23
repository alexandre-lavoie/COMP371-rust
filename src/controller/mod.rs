use crate::io::Input;

mod camera_controller;

pub use camera_controller::*;

pub trait Controllable {
    fn attach_controller(&mut self, controller: Box<dyn Controller<Self>>)
    where
        Self: Sized;

    fn update_controllers(&mut self, input: &Input);
}

pub trait Controller<T: Controllable> {
    fn update(&self, parent: &mut T, input: &Input);
}