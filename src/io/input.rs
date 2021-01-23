use crate::io::*;
use web_sys::HtmlCanvasElement;

#[derive(Clone)]
pub struct Input {
    mouse: Mouse,
    keyboard: Keyboard,
    gamepads: Vec<Gamepad>
}

impl Input {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        Input {
            mouse: Mouse::new(canvas),
            keyboard: Keyboard::new(canvas),
            gamepads: vec![]
        }
    }

    pub fn get_mouse(&self) -> &Mouse {
        &self.mouse
    }
}