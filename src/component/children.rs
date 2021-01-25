use crate::component::{Component, HasComponents};
use std::slice::{Iter, IterMut};

#[derive(Clone)]
pub struct Children<C> {
    vector: Vec<C>,
}

impl<C> Children<C> {
    pub fn get_all(&self) -> &Vec<C> {
        &self.vector
    }

    pub fn push(&mut self, object: C) {
        self.vector.push(object);
    }

    pub fn get(&self, index: usize) -> &C {
        &self.vector[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut C {
        self.vector.get_mut(index).unwrap()
    }

    pub fn iter(&self) -> Iter<'_, C> {
        self.vector.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, C> {
        self.vector.iter_mut()
    }
}

impl<C> From<Vec<C>> for Children<C> {
    fn from(vector: Vec<C>) -> Children<C> {
        Children {
            vector
        }
    }
}

impl<C> From<C> for Children<C> {
    fn from(element: C) -> Children<C> {
        Children {
            vector: vec![element]
        }
    }
}

impl<C> Default for Children<C> {
    fn default() -> Children<C> {
        Children {
            vector: vec![]
        }
    }
}

impl<C: HasComponents> Component for Children<C> {
    fn update(&mut self, dt: f32) {
        for child in self.vector.iter_mut() {
            child.update_components(dt);
        }
    }
}

impl<C: HasComponents> HasComponents for Children<Box<C>> {
    fn update_components(&mut self, dt: f32) {
        for child in self.vector.iter_mut() {
            child.as_mut().update_components(dt);
        }
    }
}