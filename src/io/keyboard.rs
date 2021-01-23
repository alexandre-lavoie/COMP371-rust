use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{EventTarget, HtmlCanvasElement, MouseEvent};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct Keyboard {

}

impl Keyboard {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        Keyboard {

        }
    }
}