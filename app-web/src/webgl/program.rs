use web_sys::{WebGlProgram, WebGlUniformLocation};

#[derive(Debug, Clone)]
pub struct ProgramInfo(
    pub WebGlProgram,
    pub (u32, u32),
    pub (
        Result<WebGlUniformLocation, String>,
        Result<WebGlUniformLocation, String>,
    ),
);
