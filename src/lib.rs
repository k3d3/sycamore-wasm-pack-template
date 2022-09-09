#![recursion_limit = "512"]

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    sycamore::render(|cx| {
        view! { cx,
            p { "Hello, World!" }
        }
    });
    Ok(())
}
