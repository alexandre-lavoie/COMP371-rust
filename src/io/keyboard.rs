use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{EventTarget, KeyboardEvent};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

pub enum KeyboardKey {
    LEFT, FORWARD, BACKWARD, RIGHT, UP, DOWN
}

#[derive(Clone, Debug)]
pub struct Keyboard {
    left: Rc<RefCell<bool>>,
    right: Rc<RefCell<bool>>,
    up: Rc<RefCell<bool>>,
    down: Rc<RefCell<bool>>,
    forward: Rc<RefCell<bool>>,
    backward: Rc<RefCell<bool>>
}

impl Default for Keyboard {
    fn default() -> Self {
        let window_target: EventTarget = web_sys::window().unwrap().into();

        let left = Rc::new(RefCell::new(false));
        let right = Rc::new(RefCell::new(false));
        let up = Rc::new(RefCell::new(false));
        let down = Rc::new(RefCell::new(false));
        let forward = Rc::new(RefCell::new(false));
        let backward = Rc::new(RefCell::new(false));

        {
            let left = left.clone();
            let right = right.clone();
            let up = up.clone();
            let down = down.clone();
            let forward = forward.clone();
            let backward = backward.clone();

            let key_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                match event.key().as_str() {
                    "a" | "A" => *left.clone().borrow_mut() = true,
                    "s" | "S" => *backward.clone().borrow_mut() = true,
                    "d" | "D" => *right.clone().borrow_mut() = true,
                    "w" | "W" => *forward.clone().borrow_mut() = true,
                    " " => *up.clone().borrow_mut() = true,
                    "Shift" => *down.clone().borrow_mut() = true,
                    _ => {}
                };
            }) as Box<dyn FnMut(KeyboardEvent)>);

            window_target
                .add_event_listener_with_callback("keydown", key_cb.as_ref().unchecked_ref())
                .unwrap();

            key_cb.forget();
        }

        {
            let left = left.clone();
            let right = right.clone();
            let up = up.clone();
            let down = down.clone();
            let forward = forward.clone();
            let backward = backward.clone();

            let key_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                match event.key().as_str() {
                    "a" | "A" => *left.clone().borrow_mut() = false,
                    "s" | "S" => *backward.clone().borrow_mut() = false,
                    "d" | "D" => *right.clone().borrow_mut() = false,
                    "w" | "W" => *forward.clone().borrow_mut() = false,
                    " " => *up.clone().borrow_mut() = false,
                    "Shift" => *down.clone().borrow_mut() = false,
                    _ => {}
                };
            }) as Box<dyn FnMut(KeyboardEvent)>);

            window_target
                .add_event_listener_with_callback("keyup", key_cb.as_ref().unchecked_ref())
                .unwrap();

            key_cb.forget();
        }

        Keyboard {
            left,
            right,
            up,
            down,
            forward,
            backward
        }
    }
}

impl Keyboard {
    pub fn is_down(&self, key: KeyboardKey) -> bool {
        match key {
            KeyboardKey::UP => self.up.borrow().clone(),
            KeyboardKey::DOWN => self.down.borrow().clone(),
            KeyboardKey::RIGHT => self.right.borrow().clone(),
            KeyboardKey::LEFT => self.left.borrow().clone(),
            KeyboardKey::FORWARD => self.forward.borrow().clone(),
            KeyboardKey::BACKWARD => self.backward.borrow().clone()
        }
    }
}