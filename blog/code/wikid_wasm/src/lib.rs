#![allow(dead_code)]
//! Compile with `wasm-pack build --target web``
//!```
//! use wasm_bindgen::prelude::*;
//!
//!mod wikid_wasm;
//!use wikid_wasm::Applet;
//!
//!#[wasm_bindgen]
//!pub struct MyProgram {
//!    applet: Applet,
//!}
//!
//!/// Public methods, exported to JavaScript.
//!#[wasm_bindgen]
//!impl MyProgram {
//!    pub fn new(canvas: String) -> Self {
//!        let applet = Applet::new(256, 256, canvas);
//!        
//!        Self {
//!            applet,
//!        }
//!    }
//!
//!    pub fn render(&self) {
//!        self.applet.render();
//!    }
//!
//!    pub fn tick(&mut self) {
//!        self.applet.tick();
//!    }
//!}
//! ```
mod applet;
mod style;
pub mod element;
mod util;

#[cfg(test)]
mod tests;

pub use applet::{Applet, Callback};
pub use style::{Style, TextAlign};
pub use util::*;

/// Macro to log results to console
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn debug_panic() {
    console_error_panic_hook::set_once();
}