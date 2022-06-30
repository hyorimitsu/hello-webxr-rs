use js_sys::Promise;
use wasm_bindgen::{JsValue, prelude::*};

use utils::logs::set_panic_hook;
use webxr::webxr::WebXR;

mod utils;
mod webgl;
mod webxr;

#[wasm_bindgen]
pub struct App {
    xr: WebXR,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {
        set_panic_hook();
        App { xr: WebXR::new() }
    }

    pub fn initialize(&self) -> Promise {
        log!("initializing...");
        self.xr.initialize_session()
    }

    pub fn run(&self) -> Result<(), JsValue> {
        log!("running...");
        self.xr.start()
    }
}
