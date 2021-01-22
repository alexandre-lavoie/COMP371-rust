use crate::camera::Camera;
use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    EventTarget, HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext, WebGlBuffer,
};

pub struct Input {
    pub m_dx: Rc<RefCell<f32>>,
    pub m_dy: Rc<RefCell<f32>>,
    pub p_dx: Rc<RefCell<f32>>,
    pub p_dy: Rc<RefCell<f32>>,
    pub m_down: Rc<RefCell<bool>>,
}

pub struct Engine {
    vertex_buffer: Option<WebGlBuffer>,
    normal_buffer: Option<WebGlBuffer>,
    index_buffer: Option<WebGlBuffer>,
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    objects: Vec<Box<dyn Object>>,
    cameras: Vec<Camera>,
    indicies_sizes: Vec<u16>,
    previous_time: f32,
    input: Input,
}

pub fn coerce<S: ?Sized>(r: &Box<S>) -> &S {
    r
}

impl Engine {
    pub fn new(canvas: HtmlCanvasElement, gl: WebGl2RenderingContext) -> Self {
        Engine {
            canvas: canvas,
            gl: gl,
            objects: vec![],
            cameras: vec![],
            indicies_sizes: vec![],
            previous_time: 0.,
            input: Input {
                m_dx: Rc::new(RefCell::new(0.0)),
                m_dy: Rc::new(RefCell::new(0.0)),
                p_dx: Rc::new(RefCell::new(0.0)),
                p_dy: Rc::new(RefCell::new(0.0)),
                m_down: Rc::new(RefCell::new(false)),
            },
            vertex_buffer: None,
            index_buffer: None,
            normal_buffer: None,
        }
    }

    pub fn attach_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn attach_camera(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.attach_input();

        self.init_objects()?;

        Ok(())
    }

    fn init_objects(&mut self) -> Result<(), &'static str> {
        let mut vertex_vector: Vec<f32> = vec![];
        let mut index_vector: Vec<u16> = vec![];
        let mut normal_vector: Vec<f32> = vec![];

        for box_object in self.objects.iter() {
            let obj = coerce(box_object);

            let indicies = obj.get_indicies();

            self.indicies_sizes.push(indicies.len() as u16);

            index_vector.append(&mut Vec::from(indicies));

            vertex_vector.append(&mut Vec::from(obj.get_verticies()));

            normal_vector.append(&mut Vec::from(obj.get_normals()));
        }

        let vertex_buffer = self.gl.create_buffer().ok_or("failed to create buffer")?;

        self.gl
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        unsafe {
            let vertex_array = js_sys::Float32Array::view(&vertex_vector);
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertex_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        };

        let normal_buffer = self.gl.create_buffer().ok_or("failed to craete buffer")?;

        self.gl
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&normal_buffer));

        unsafe {
            let normal_array = js_sys::Float32Array::view(&normal_vector);
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &normal_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        };

        let index_buffer = self.gl.create_buffer().ok_or("failed to create buffer")?;

        self.gl.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );

        unsafe {
            let index_array = js_sys::Uint16Array::view(&index_vector.as_slice());
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &index_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.normal_buffer = Some(normal_buffer);

        Ok(())
    }

    pub fn update(&mut self, time: f32) {
        let dt = time - self.previous_time;

        self.previous_time = time;

        self.update_input(dt);
        self.render(dt);
    }

    fn attach_input(&mut self) {
        let canvas_target: EventTarget = self.canvas.clone().into();
        let window_target: EventTarget = web_sys::window().expect("No window").into();

        let m_down = Rc::new(RefCell::new(false));
        let m_dx = Rc::new(RefCell::new(0.0));
        let m_dy = Rc::new(RefCell::new(0.0));
        let p_dx = Rc::new(RefCell::new(0.0));
        let p_dy = Rc::new(RefCell::new(0.0));

        {
            let m_down = m_down.clone();

            let mousedown_cb = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                *m_down.borrow_mut() = true;
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas_target
                .add_event_listener_with_callback(
                    "mousedown",
                    mousedown_cb.as_ref().unchecked_ref(),
                )
                .unwrap();

            mousedown_cb.forget();
        }

        {
            let m_down = m_down.clone();

            let mouseup_cb = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                *m_down.borrow_mut() = false;
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas_target
                .add_event_listener_with_callback("mouseup", mouseup_cb.as_ref().unchecked_ref())
                .unwrap();

            mouseup_cb.forget();
        }

        {
            let dx = m_dx.clone();
            let dy = m_dy.clone();

            let mousemove_cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                *dx.borrow_mut() = event.movement_x() as f32;
                *dy.borrow_mut() = event.movement_y() as f32;
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas_target
                .add_event_listener_with_callback(
                    "mousemove",
                    mousemove_cb.as_ref().unchecked_ref(),
                )
                .unwrap();

            mousemove_cb.forget();
        }

        {
            let dx = p_dx.clone();
            let dy = p_dy.clone();

            let keydown_cb = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let (x, y) = match &*event.key() {
                    "a" => (-1., *dy.borrow_mut()),
                    "d" => (1., *dy.borrow_mut()),
                    "w" => (*dx.borrow_mut(), 1.),
                    "s" => (*dx.borrow_mut(), -1.),
                    _ => (0., 0.),
                };

                *dx.borrow_mut() = x;
                *dy.borrow_mut() = y;
            }) as Box<dyn FnMut(KeyboardEvent)>);

            window_target
                .add_event_listener_with_callback("keydown", keydown_cb.as_ref().unchecked_ref())
                .unwrap();
            keydown_cb.forget();
        }

        {
            let dx = p_dx.clone();
            let dy = p_dy.clone();

            let keydown_cb = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let (x, y) = match &*event.key() {
                    "a" => (0., *dy.borrow_mut()),
                    "d" => (0., *dy.borrow_mut()),
                    "w" => (*dx.borrow_mut(), 0.),
                    "s" => (*dx.borrow_mut(), 0.),
                    _ => (0., 0.),
                };

                *dx.borrow_mut() = x;
                *dy.borrow_mut() = y;
            }) as Box<dyn FnMut(KeyboardEvent)>);

            window_target
                .add_event_listener_with_callback("keyup", keydown_cb.as_ref().unchecked_ref())
                .unwrap();
            keydown_cb.forget();
        }

        self.input = Input {
            m_dx: m_dx,
            m_dy: m_dy,
            p_dx: p_dx,
            p_dy: p_dy,
            m_down,
        };
    }

    fn update_input(&mut self, _dt: f32) {
        for camera in self.cameras.iter_mut() {
            if self.input.m_down.borrow().clone() {
                camera.rotation[1] += self.input.m_dx.borrow().clone() / 10.;
                camera.rotation[0] -= self.input.m_dy.borrow().clone() / 10.;
            }

            camera.position[0] += self.input.p_dx.borrow().clone() / 10.;
            camera.position[2] += self.input.p_dy.borrow().clone() / 10.;
        }
    }

    fn render(&mut self, _dt: f32) {
        self.gl.enable(WebGl2RenderingContext::DEPTH_TEST);

        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.gl.clear_color(0., 0., 0., 1.);

        self.gl.clear_depth(1.);

        self.gl.enable(WebGl2RenderingContext::CULL_FACE);

        self.gl.cull_face(WebGl2RenderingContext::BACK);

        let width = self.canvas.width();

        let height = self.canvas.height();

        for camera in self.cameras.iter() {
            camera.attach_viewport(&self.gl, width, height);

            let projection_matrix = camera.projection_matrix(width as f32 / height as f32);
            let mut offset = 0u16;
            for (box_object, index_size) in self.objects.iter().zip(self.indicies_sizes.iter()) {
                let obj = coerce(box_object);
                let program = obj.get_program();
                let projection = self.gl.get_uniform_location(program, "u_projection");
                self.gl.uniform_matrix4fv_with_f32_array(
                    projection.as_ref(),
                    false,
                    &projection_matrix,
                );
                let view_matrix = camera.get_matrix();
                let view = self.gl.get_uniform_location(program, "u_view");
                self.gl
                    .uniform_matrix4fv_with_f32_array(view.as_ref(), false, &view_matrix);
                let mut normal_matrix = mat4::new_identity::<f32>();
                mat4::inv(&mut normal_matrix, &view_matrix);
                let matrix_clone = normal_matrix.clone();
                mat4::transpose(&mut normal_matrix, &matrix_clone);
                let normal = self.gl.get_uniform_location(program, "u_normal");
                self.gl
                    .uniform_matrix4fv_with_f32_array(normal.as_ref(), false, &normal_matrix);
                let world_matrix = obj.get_matrix();
                let world = self.gl.get_uniform_location(program, "u_world");
                self.gl
                    .uniform_matrix4fv_with_f32_array(world.as_ref(), false, &world_matrix);
                self.gl.use_program(Some(&program));

                self.gl.bind_buffer(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    self.vertex_buffer.as_ref(),
                );
                self.gl.vertex_attrib_pointer_with_i32(
                    0,
                    3,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
                self.gl.enable_vertex_attrib_array(0);

                self.gl.bind_buffer(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    self.normal_buffer.as_ref(),
                );
                self.gl.vertex_attrib_pointer_with_i32(
                    1,
                    3,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
                self.gl.enable_vertex_attrib_array(1);

                self.gl.bind_buffer(
                    WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                    self.index_buffer.as_ref(),
                );
                self.gl.draw_elements_with_i32(
                    WebGl2RenderingContext::TRIANGLES,
                    index_size.clone() as i32,
                    WebGl2RenderingContext::UNSIGNED_SHORT,
                    offset as i32,
                );
                offset += index_size;
            }
        }
    }
}
