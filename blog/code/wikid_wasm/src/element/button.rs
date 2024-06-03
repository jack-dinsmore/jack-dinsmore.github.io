use image::Rgba;

use crate::{style::TextAlign, Callback, Dim, Style};

use super::{Element, EventResponse, Mouse};

pub struct Button {
    top: u32,
    left: u32,
    bottom: u32,
    right: u32,
    text: String,
    
    hover: bool,
}

impl Button {
    pub fn new(pos: (Dim, Dim), text: String, window_width: u32, window_height: u32) -> Self {
        let width = Dim::Pixel(128).to_pixel(window_width, window_height, true);
        let height = Dim::Pixel(42).to_pixel(window_width, window_height, false);
        let left = pos.0.to_pixel(window_width, window_height, true) - width/2;
        let top = pos.1.to_pixel(window_width, window_height, false) - height/2;
        Self {
            top,
            left,
            bottom: top + height,
            right: left + width,
            text,

            hover: false,
        }
    }
}

impl Element for Button {
    fn draw(&self, pixels: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, style: &Style) {
        let light_color = Rgba([
            255 - (255 - style.highlight_color.0[0])/4,
            255 - (255 - style.highlight_color.0[1])/4,
            255 - (255 - style.highlight_color.0[2])/4,
            255,
        ]);
        if self.hover {
            for i in 0..self.right - self.left {
                for j in 0..self.bottom - self.top {
                    pixels[(self.left + i, self.top + j)] = light_color;
                }
            }
        }
        for i in 0..self.right - self.left {
            pixels[(self.left + i, self.top)] = style.highlight_color;
            pixels[(self.left + i, self.top+1)] = style.highlight_color;
            pixels[(self.left + i, self.bottom)] = style.highlight_color;
            pixels[(self.left + i, self.bottom-1)] = style.highlight_color;
        }
        for j in 0..self.bottom - self.top {
            pixels[(self.left, self.top + j)] = style.highlight_color;
            pixels[(self.left+1, self.top + j)] = style.highlight_color;
            pixels[(self.right, self.top + j)] = style.highlight_color;
            pixels[(self.right-1, self.top + j)] = style.highlight_color;
        }

        style.render_text(
            pixels, (self.left + self.right) / 2, (self.top + self.bottom) / 2,
            &self.text, Rgba([0,0,0,255]), TextAlign::Center, TextAlign::Center
        );
    }

    fn bbox(&self, mouse: Mouse) -> bool {
        mouse.x > self.left &&
        mouse.x < self.right &&
        mouse.y > self.top &&
        mouse.y < self.bottom
    }

    fn mouse_button_up(&mut self, mouse: Mouse) -> EventResponse {
        if !self.bbox(mouse) { return EventResponse::NoEvent }
        EventResponse::PlaceCallback(Callback::ButtonClicked(self as *const Self))
    }

    fn mouse_move(&mut self, mouse: Mouse) -> EventResponse {
        self.hover = false;
        if !self.bbox(mouse) { return EventResponse::NoEvent }
        self.hover = true;
        EventResponse::Responded
    }
}