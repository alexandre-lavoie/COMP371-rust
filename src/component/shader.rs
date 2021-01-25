use crate::component::Component;
use web_sys::WebGlProgram;

#[derive(Default, Clone, Debug)]
pub struct Shader {
    program: Option<WebGlProgram>,
}

impl Shader {
    pub fn get_program(&self) -> &WebGlProgram {
        self.program.as_ref().expect("No program set for shader. Did you attach_component(shader)?")
    }
}

impl From<WebGlProgram> for Shader {
    fn from(program: WebGlProgram) -> Self {
        Shader {
            program: Some(program)
        }
    }
}

impl Component for Shader {
    fn update(&mut self, dt: f32) {}
}
