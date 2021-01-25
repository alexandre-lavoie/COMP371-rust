use crate::component::Component;
use crate::io::*;
use web_sys::HtmlCanvasElement;

#[derive(Default, Clone, Debug)]
pub struct Input {
    mouse: Mouse,
    keyboard: Keyboard,
    gamepads: Vec<Gamepad>,
}

impl Input {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        Input {
            mouse: Mouse::new(canvas),
            keyboard: Keyboard::default(),
            gamepads: vec![],
        }
    }

    pub fn get_mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub fn get_keyboard(&self) -> &Keyboard {
        &self.keyboard
    }
}

impl Component for Input {
    fn update(&mut self, dt: f32) {}
}
