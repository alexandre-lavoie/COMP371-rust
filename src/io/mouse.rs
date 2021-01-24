use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlCanvasElement, MouseEvent};

pub enum MouseButton {
    PRIMARY,
    SECONDARY,
    AUXILIARY,
    FORTH,
    FIFTH,
}

#[derive(Clone)]
pub struct Mouse {
    buttons: Rc<RefCell<u16>>,
    dx: Rc<RefCell<i32>>,
    dy: Rc<RefCell<i32>>,
}

impl Mouse {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let canvas_target: EventTarget = canvas.clone().into();

        let buttons = Rc::new(RefCell::new(0u16));
        let dx = Rc::new(RefCell::new(0i32));
        let dy = Rc::new(RefCell::new(0i32));

        {
            let buttons = buttons.clone();

            let mouse_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
                *buttons.borrow_mut() = event.buttons();
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas_target
                .add_event_listener_with_callback("mousedown", mouse_cb.as_ref().unchecked_ref())
                .unwrap();

            mouse_cb.forget();
        }

        {
            let buttons = buttons.clone();

            let mouse_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
                *buttons.borrow_mut() = event.buttons();
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas_target
                .add_event_listener_with_callback("mouseup", mouse_cb.as_ref().unchecked_ref())
                .unwrap();

            mouse_cb.forget();
        }

        {
            let dx = dx.clone();
            let dy = dy.clone();

            let mouse_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
                *dx.borrow_mut() = event.movement_x();
                *dy.borrow_mut() = event.movement_y();
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas_target
                .add_event_listener_with_callback("mousemove", mouse_cb.as_ref().unchecked_ref())
                .unwrap();

            mouse_cb.forget();
        }

        {
            let buttons = buttons.clone();
            let dx = dx.clone();
            let dy = dy.clone();

            let mouse_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *buttons.borrow_mut() = 0;
                *dx.borrow_mut() = 0;
                *dy.borrow_mut() = 0;
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas_target
                .add_event_listener_with_callback("mouseleave", mouse_cb.as_ref().unchecked_ref())
                .unwrap();

            mouse_cb.forget();
        }

        Mouse { buttons, dx, dy }
    }

    pub fn is_down(&self, mouse_button: MouseButton) -> bool {
        let bv = self.buttons.borrow().clone();

        match mouse_button {
            MouseButton::PRIMARY => bv & 0b0000_0001 == 1,
            MouseButton::SECONDARY => bv & 0b0000_0010 == 1,
            MouseButton::AUXILIARY => bv & 0b0000_0100 == 1,
            MouseButton::FORTH => bv & 0b0000_1000 == 1,
            MouseButton::FIFTH => bv & 0b0001_0000 == 1,
        }
    }

    pub fn is_up(&self, mouse_button: MouseButton) -> bool {
        !self.is_down(mouse_button)
    }

    pub fn get_dx(&self) -> i32 {
        self.dx.borrow().clone()
    }

    pub fn get_dy(&self) -> i32 {
        self.dy.borrow().clone()
    }
}
