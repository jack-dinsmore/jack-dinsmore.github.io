use image::GenericImageView;
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
pub async fn fetch_url_binary(url: String) -> Result<Uint8Array, JsValue> {
    let window = web_sys::window().unwrap(); // Browser window
    let promise = JsFuture::from(window.fetch_with_str(&url)); // File fetch promise
    let result = promise.await?; // Await fulfillment of fetch
    let response: web_sys::Response = result.dyn_into().unwrap(); // Type casting
    let image_data = JsFuture::from(response.array_buffer()?).await?; // Get text
    Ok(Uint8Array::new(&image_data))
}

#[wasm_bindgen]
pub async fn unred(url: String, canvas: String) -> Result<(), JsValue> {
    log!("A");
    let binary = fetch_url_binary(url).await?;
    log!("A1");
    let altbuf = binary.to_vec();
    log!("A2");

    // Convert the png encoded bytes to an rgba pixel buffer (given the PNG is actually in 8byte RGBA format).
    let image = image::load_from_memory_with_format(&altbuf, image::ImageFormat::Png).unwrap();
    let mut rgba_image = image.to_rgba8();
    log!("A3");

    // I suppose this is what you tried to do in your original loop
    // judging by the function name:
    for (_, _, pixel) in rgba_image.enumerate_pixels_mut() {
        if pixel[0] > 0 {
            *pixel = image::Rgba([0, pixel[1], pixel[2], pixel[3]]);
        }
    }

    log!("B");

    // let window = web_sys::window().unwrap();
    // let document = window.document().expect("Could not get document");
    // let canvas = document
    //     .get_element_by_id(&canvas)
    //     .unwrap()
    //     .dyn_into::<web_sys::HtmlCanvasElement>()?;
    // let context = canvas
    //     .get_context("2d")?
    //     .unwrap()
    //     .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    // let clamped_buf: Clamped<&[u8]> = Clamped(rgba_image.as_raw());
    // let image_data_temp = 
    //     ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, image.width(), image.height())?;
    // context.put_image_data(&image_data_temp, 0.0, 0.0)?;
    // log!("C");
    Ok(())
}