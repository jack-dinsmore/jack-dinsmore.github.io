use image::Rgba;
use crate::{blend_color, style::{BLACK, WHITE}, Dim, Style};
use super::{Element, EventResponse, Mouse};

const SLIDER_RADIUS: i32 = 6;
const THICKNESS: i32 = 1;
const TEXT_BUFFER: i32 = 8;

pub enum SliderType {
    Float(u32),
    Int,
}

pub struct Slider {
    x: u32,
    y: u32,
    width: i32,
    text: String,
    slider_type: SliderType,

    pos: f32,
    start: f32,
    end: f32,
    selected: bool,
}

impl Slider {
    pub fn new(x: Dim, y: Dim, text: String, slider_type: SliderType, data: [f32;3], window_width: u32, window_height: u32) -> Self {
        let width = Dim::Pixel(128);
        Self {
            x: x.to_pixel(window_width, window_height, true),
            y: y.to_pixel(window_width, window_height, false),
            width: width.to_pixel(window_width, window_height, true) as i32,
            text,
            slider_type,

            pos: (data[2] - data[0]) / (data[1] - data[0]),
            start: data[0],
            end: data[1],
            selected: false,
        }
    }

    pub fn get_value(&self) -> f32 {
        let f_out = self.pos * (self.end - self.start) + self.start;
        match self.slider_type {
            SliderType::Float(d) => {
                let base = 10f32.powi(d as i32);
                (f_out * base).round() / base
            },
            SliderType::Int => f_out.round(),
        }.clamp(self.start, self.end)
    }
}

impl Element for Slider {
    fn draw(&self, pixels: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, style: &Style) {
        for i in -self.width/2..=self.width/2 {
            pixels[((self.x as i32 + i) as u32, self.y)] = Rgba([0,0,0,255]);
        }
        let dx = (self.pos * self.width as f32) as i32 - self.width / 2;
        for i in -(SLIDER_RADIUS+THICKNESS/2)..=SLIDER_RADIUS+THICKNESS/2 {
            for j in -(SLIDER_RADIUS+THICKNESS/2)..=SLIDER_RADIUS+THICKNESS/2 {
                let dist = ((i * i + j * j) as f32).sqrt();
                let v = ((SLIDER_RADIUS as f32 - dist)/THICKNESS as f32).abs().min(1.);
                if v >= 0. {
                    if self.x as i32 + i + dx < 0 {continue;}
                    if self.x as i32 + i + dx >= pixels.width() as i32 {continue;}
                    if self.y as i32 + j < 0 {continue;}
                    if self.y as i32 + j >= pixels.height() as i32 {continue;}
                    pixels[((self.x as i32 + i + dx) as u32, (self.y as i32 + j) as u32)] = blend_color(WHITE, style.highlight_color, v);
                }
            }
        }
        style.render_text(
            pixels, (self.x as i32 - self.width/2 - TEXT_BUFFER) as u32, self.y,
            &self.text, BLACK,
            crate::style::TextAlign::Center,
            crate::style::TextAlign::LowerRight
        );
        style.render_text(
            pixels, (self.x as i32 + self.width/2 + TEXT_BUFFER) as u32, self.y,
            &format!("{}", self.get_value()), BLACK,
            crate::style::TextAlign::Center,
            crate::style::TextAlign::UpperLeft
        );
    }

    fn bbox(&self, mouse: Mouse) -> bool {
        (mouse.x as i32) > self.x as i32 - self.width/2-SLIDER_RADIUS &&
        (mouse.x as i32) < self.x as i32 + self.width/2+SLIDER_RADIUS &&
        (mouse.y as i32) > self.y as i32 - SLIDER_RADIUS &&
        (mouse.y as i32) < self.y as i32 + SLIDER_RADIUS
    }

    fn mouse_button_down(&mut self, mouse: Mouse) -> super::EventResponse {
        if !self.bbox(mouse) { return EventResponse::NoEvent }
        self.selected = true;
        self.pos = (mouse.x as i32 - (self.x as i32 - self.width/2)) as f32 / self.width as f32;
        self.pos = self.pos.clamp(0., 1.);
        EventResponse::Responded
    }

    fn mouse_button_up(&mut self, mouse: Mouse) -> super::EventResponse {
        self.selected = false;
        if !self.bbox(mouse) { return EventResponse::NoEvent }
        EventResponse::Responded
    }

    fn mouse_move(&mut self, mouse: Mouse) -> super::EventResponse {
        if mouse.down && self.selected {
            self.pos = (mouse.x as i32 - (self.x as i32 - self.width/2)) as f32 / self.width as f32;
            self.pos = self.pos.clamp(0., 1.);
        }
        if !self.bbox(mouse) { return EventResponse::NoEvent }
        EventResponse::Responded
    }
}