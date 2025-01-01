mod app;

use wasm_bindgen::prelude::*;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Hello, this is a log entry from Rust");

    yew::Renderer::<app::App>::new().render();
}

