mod slider;
mod dynamic_plot;
mod button;

use image::ImageBuffer;
pub use slider::*;
pub use dynamic_plot::*;
pub use button::*;
use super::{Callback, Style};

pub trait Element {
    fn draw(&self, pixels: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>, style: &Style);
    fn tick(&mut self) -> Option<Callback> { None }
    /// Returns true if the mouse is inside the element
    fn bbox(&self, mouse: Mouse) -> bool;
    fn mouse_button_down(&mut self, mouse: Mouse) -> EventResponse {
        if !self.bbox(mouse) { return EventResponse::NoEvent }
        EventResponse::Responded
    }
    fn mouse_button_up(&mut self, mouse: Mouse) -> EventResponse {
        if !self.bbox(mouse) { return EventResponse::NoEvent }
        EventResponse::Responded
    }
    fn mouse_move(&mut self, mouse: Mouse) -> EventResponse {
        if !self.bbox(mouse) { return EventResponse::NoEvent }
        EventResponse::Responded
    }
}

/// Enum used to determine whether a given element is affected by an event
pub enum EventResponse {
    /// This element is not affected by the event
    NoEvent,
    /// The element has responded to the event
    Responded,
    /// The element has responded to the event by generating a callback
    PlaceCallback(Callback),
}

#[derive(Clone, Copy, Debug)]
pub struct Mouse {
    pub x: u32,
    pub y: u32,
    pub down: bool
}