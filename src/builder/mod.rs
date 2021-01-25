mod engine;
mod model;
mod component;

pub use model::*;
pub use engine::*;
pub use component::*;

use crate::component::*;
use crate::controller::*;

pub trait Builder<T> {
    fn get_inner(&self) -> &T;

    fn get_inner_mut(&mut self) -> &mut T;

    fn modify_inner(mut self, func: Box<dyn Fn(&mut T)>) -> Result<Self, &'static str> where Self: Sized {
        func(self.get_inner_mut());

        Ok(self)
    }

    fn build(self) -> Result<T, &'static str>;

    fn boxed(self) -> Result<Box<T>, &'static str> where Self: Sized {
        Ok(Box::new(self.build()?))
    }
}

pub trait AttachComponent<T: Component, U: HasComponent<T>>: Builder<U> where Self: Sized {
    fn attach_component(self, component: T) -> Result<Self, &'static str>;
}

impl<T: Component, U: HasComponent<T> + Default, B: Builder<U>> AttachComponent<T, U> for B {
    fn attach_component(mut self, component: T) -> Result<Self, &'static str> {
        self.get_inner_mut().attach_component(component);

        Ok(self)
    }
}

pub trait AttachComponentBuilder<T: Component, B: Builder<T>, U: HasComponent<T>>: Builder<U> where Self: Sized {
    fn attach_component_builder(self, builder: B) -> Result<Self, &'static str>;
}

impl<T: Component, B: Builder<T>, U: HasComponent<T> + Default, B2: Builder<U>> AttachComponentBuilder<T, B, U> for B2 {
    fn attach_component_builder(mut self, builder: B) -> Result<Self, &'static str> {
        self.get_inner_mut().attach_component(builder.build()?);

        Ok(self)
    }
}

pub trait AttachController<T: Controller<U>, U: HasController<U, T>>: Builder<U> where Self: Sized {
    fn attach_controller(self, component: T) -> Result<Self, &'static str>;
}

impl<T: Controller<U>, U: HasController<U, T> + Default, B: Builder<U>> AttachController<T, U> for B {
    fn attach_controller(mut self, component: T) -> Result<Self, &'static str> {
        self.get_inner_mut().attach_controller(component);

        Ok(self)
    }
}

pub trait AttachBuilder<T: HasComponents, B: Builder<T>, U>: Builder<U> where Self: Sized {
    fn attach_builder(self, builder: B) -> Result<Self, &'static str>;
}

impl<T: HasComponents, U: HasComponent<Children<T>> + Default, B: Builder<T>, B2: Builder<U>> AttachBuilder<T, B, U> for B2 {
    fn attach_builder(mut self, builder: B) -> Result<Self, &'static str> {
        let inner = self.get_inner_mut();

        if let Ok(children) = inner.get_component_mut() {
            children.push(builder.build()?);
        } else {
            inner.attach_component(Children::from(builder.build()?));
        }

        Ok(self)
    }
}