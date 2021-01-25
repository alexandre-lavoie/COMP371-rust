use crate::*;

#[derive (Default)]
pub struct CameraBuilder<T: CameraModel> {
    inner: T
}

impl<T: CameraModel> Builder<T> for CameraBuilder<T> {
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