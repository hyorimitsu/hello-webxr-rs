
/// A macro to provide syntax for create `js_sys::Float32Array`.
#[macro_export]
macro_rules! float32_array {
    ($arr:expr) => {{
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let arr_location = $arr.as_ptr() as u32 / 4;

        js_sys::Float32Array::new(&memory_buffer)
            .subarray(arr_location, arr_location + $arr.len() as u32)
    }};
}

/// A macro to provide syntax for create `js_sys::Uint16Array`.
#[macro_export]
macro_rules! uint16_array {
    ($arr:expr) => {{
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let arr_location = $arr.as_ptr() as u32 / 2;

        js_sys::Uint16Array::new(&memory_buffer)
            .subarray(arr_location, arr_location + $arr.len() as u32)
    }};
}
