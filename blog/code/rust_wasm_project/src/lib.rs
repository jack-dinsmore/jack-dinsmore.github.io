// /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --allow-file-access-from-files test.html

use image::{GenericImageView, ImageBuffer};
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, JsCast, Clamped};
use wasm_bindgen_futures::JsFuture;
use web_sys::ImageData;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub async fn unred(canvas: String) -> Result<(), JsValue> {
    let mut rgba_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::new(1024, 1024);

    // I suppose this is what you tried to do in your original loop
    // judging by the function name:
    for (x, _, pixel) in rgba_image.enumerate_pixels_mut() {
        if x > 200 && x < 300 {
            *pixel = image::Rgba([255, 0, 0, 255]);
        }
    }

    let window = web_sys::window().unwrap();
    let document = window.document().expect("Could not get document");
    let canvas = document
        .get_element_by_id(&canvas)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let clamped_buf: Clamped<&[u8]> = Clamped(rgba_image.as_raw());
    let image_data_temp = 
        ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, 1024, 1024)?;
    context.put_image_data(&image_data_temp, 0.0, 0.0)?;
    Ok(())
}