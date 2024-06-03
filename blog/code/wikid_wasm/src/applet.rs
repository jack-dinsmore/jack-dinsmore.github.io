
use std::vec::Drain;

use image::ImageBuffer;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

use super::{element::{Element, EventResponse, Mouse}, style::{BLACK, WHITE}, Style};

pub enum Callback {
    ButtonClicked(*const super::element::Button),
}

pub struct Applet {
    width: u32,
    height: u32,
    name: String,
    pub style: Style,
    
    callbacks: Vec<Callback>,
    mouse_down: bool,
    buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl Applet {
    pub fn new(width: u32, height: u32, name: String, style: Style) -> Self {
        let window = web_sys::window().expect("No global `window` exists");
        let document = window.document().expect("Should have a document on window");
        let canvas = document.get_element_by_id(&name).expect("Could not find the canvas");
        canvas.set_attribute("width", &width.to_string()).expect("The provided HTML object was not a canvas");
        canvas.set_attribute("height", &height.to_string()).expect("The provided HTML object was not a canvas");

        let buffer = ImageBuffer::new(width, height);
        let callbacks = Vec::new();

        Self {
            width,
            height,
            name,
            style,

            buffer,
            callbacks,
            mouse_down: false,
        }
    }

    pub fn tick(&mut self, elements: Vec<*mut  dyn Element>) -> Drain<Callback> {
        for element in elements {
            let element = unsafe { &mut *element };
            if let Some(callback) = element.tick() {
                self.callbacks.push(callback);
            }
        }
        self.callbacks.drain(..)
    }

    pub fn render(&mut self, elements: &[*const dyn Element]) {
        // Clear
        for (_, _, pixel) in self.buffer.enumerate_pixels_mut() {
            *pixel = WHITE;
        }

        // Draw
        for element in elements {
            let element = unsafe { &**element };
            element.draw(&mut self.buffer, &self.style);
        }

        // Rim
        for i in 0..self.width {
            self.buffer[(i, 0)] = BLACK;
            self.buffer[(i, self.height-1)] = BLACK;
        }
        for j in 0..self.height {
            self.buffer[(0, j)] = BLACK;
            self.buffer[(self.width-1, j)] = BLACK;
        }

        // Commit image buffer to the canvas
        let window = web_sys::window().unwrap();
        let document = window.document().expect("Could not get document");
        let canvas = document
            .get_element_by_id(&self.name)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("2d").unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
        let clamped_buf: Clamped<&[u8]> = Clamped(self.buffer.as_raw());
        let image_data_temp = 
            ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, self.width, self.height).unwrap();
        context.put_image_data(&image_data_temp, 0.0, 0.0).unwrap();
    }

    pub fn mouse_button_down(&mut self, x: u32, y: u32, elements: Vec<*mut dyn Element>) {
        self.mouse_down = true;
        let mouse = Mouse {
            x,
            y,
            down: self.mouse_down
        };
        for element in elements {
            let element = unsafe { &mut *element };
            match element.mouse_button_down(mouse) {
                EventResponse::NoEvent => (),
                EventResponse::Responded => return,
                EventResponse::PlaceCallback(c) => {
                    self.callbacks.push(c);
                    return;
                },
            }
        }
    }

    pub fn mouse_button_up(&mut self, x: u32, y: u32, elements: Vec<*mut dyn Element>) {
        self.mouse_down = false;
        let mouse = Mouse {
            x,
            y,
            down: self.mouse_down
        };
        for element in elements {
            let element = unsafe { &mut *element };
            match element.mouse_button_up(mouse) {
                EventResponse::NoEvent => (),
                EventResponse::Responded => return,
                EventResponse::PlaceCallback(c) => {
                    self.callbacks.push(c);
                    return;
                },
            }
        }
    }

    pub fn mouse_move(&mut self, x: u32, y: u32, elements: Vec<*mut dyn Element>) {
        let mouse = Mouse {
            x,
            y,
            down: self.mouse_down
        };
        for element in elements {
            let element = unsafe { &mut *element };
            match element.mouse_move(mouse) {
                EventResponse::NoEvent => (),
                EventResponse::Responded => return,
                EventResponse::PlaceCallback(c) => {
                    self.callbacks.push(c);
                    return;
                },
            }
        }
    }
}