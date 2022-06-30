use web_sys::WebGlBuffer;

#[derive(Debug, Clone)]
pub struct Buffers(
    pub WebGlBuffer,
    pub WebGlBuffer,
    pub WebGlBuffer,
);
