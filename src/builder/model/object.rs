use crate::*;

#[derive (Default)]
pub struct ObjectBuilder<T: ObjectModel> {
    inner: T
}

impl<T: ObjectModel> Builder<T> for ObjectBuilder<T> {
    fn get_inner(&self) -> &T  {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut T  {
        &mut self.inner
    }

    fn build(mut self) -> Result<T, &'static str> {
        Ok(self.inner)
    }
} 