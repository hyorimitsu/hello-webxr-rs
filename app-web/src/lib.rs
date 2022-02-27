use wasm_bindgen::prelude::*;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod utils;

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();

    // create window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    // required to use window.canvas()
    use winit::platform::web::WindowExtWebSys;

    // draw canvas
    let canvas = window.canvas();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&canvas)
        .unwrap();
}
