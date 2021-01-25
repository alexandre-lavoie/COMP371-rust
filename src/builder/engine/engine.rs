use crate::*;

#[derive (Default)]
pub struct EngineBuilder {
    inner: Engine
}

impl Builder<Engine> for EngineBuilder {
    fn get_inner(&self) -> &Engine {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut Engine {
        &mut self.inner
    }

    fn build(self) -> Result<Engine, &'static str> {
        Ok(self.inner)
    }
}