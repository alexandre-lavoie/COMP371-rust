pub trait HasControllers {
    fn update_controllers(&mut self, dt: f32);
}

pub trait HasController<S: HasControllers, T: Controller<S>>: HasControllers {
    fn attach_controller(&mut self, controller: T);

    fn get_controller(&self) -> &T;
}

pub trait Controller<P: HasControllers> {
    fn update(&mut self, parent: &mut P, dt: f32);
}