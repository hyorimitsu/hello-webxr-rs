use std::{collections::HashMap, f32::consts::PI, rc::Rc};

use js_sys::{WebAssembly};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use crate::{webgl::buffer::Buffers, webgl::program::ProgramInfo};
use crate::{float32_array, uint16_array};

#[derive(Debug, Clone)]
pub struct WebGL {
    pub context: Rc<WebGl2RenderingContext>,
    pub canvas: HtmlCanvasElement,
}

impl WebGL {
    pub fn new(xr_mode: bool) -> Result<WebGL, JsValue> {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let ctx: WebGl2RenderingContext = if xr_mode {
            let mut gl_attribs = HashMap::new();
            gl_attribs.insert(String::from("xrCompatible"), true);

            let js_gl_attribs = JsValue::from_serde(&gl_attribs).unwrap();
            canvas.get_context_with_context_options("webgl2", &js_gl_attribs)?.unwrap().dyn_into()?
        } else {
            canvas.get_context("webgl2")?.unwrap().dyn_into()?
        };

        Ok(WebGL {
            context: Rc::new(ctx),
            canvas,
        })
    }

    pub fn init_shader_program(&self, vs_source: &str, fs_source: &str) -> Result<WebGlProgram, String> {
        let v_shader = self.compile_shader(WebGl2RenderingContext::VERTEX_SHADER, vs_source);
        let f_shader = self.compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, fs_source);
        self.link_program(&v_shader?, &f_shader?)
    }

    pub fn init_buffers(&self) -> Result<Buffers, JsValue> {
        let ctx = self.context.clone();

        let positions: [f32; 72] = [
            // front face
            -1.0, -1.0, 1.0,
            1.0, -1.0, 1.0,
            1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0,
            // back face
            -1.0, -1.0, -1.0,
            -1.0, 1.0, -1.0,
            1.0, 1.0, -1.0,
            1.0, -1.0, -1.0,
            // top face
            -1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0, -1.0,
            // bottom face
            -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, 1.0,
            -1.0, -1.0, 1.0,
            // right face
            1.0, -1.0, -1.0,
            1.0, 1.0, -1.0,
            1.0, 1.0, 1.0,
            1.0, -1.0, 1.0,
            // left face
            -1.0, -1.0, -1.0,
            -1.0, -1.0, 1.0,
            -1.0, 1.0, 1.0,
            -1.0, 1.0, -1.0,
        ];
        let face_colors = [
            [1.0, 1.0, 1.0, 1.0], // front face: white
            [1.0, 0.0, 0.0, 1.0], // back face: red
            [0.0, 1.0, 0.0, 1.0], // top face: green
            [0.0, 0.0, 1.0, 1.0], // bottom face: blue
            [1.0, 1.0, 0.0, 1.0], // right face: yellow
            [1.0, 0.0, 1.0, 1.0], // left face: purple
        ];
        let indices: [u16; 36] = [
            0, 1, 2, 0, 2, 3,       // front face
            4, 5, 6, 4, 6, 7,       // back face
            8, 9, 10, 8, 10, 11,    // top face
            12, 13, 14, 12, 14, 15, // bottom face
            16, 17, 18, 16, 18, 19, // right face
            20, 21, 22, 20, 22, 23, // left face
        ];

        let position_buffer = ctx.create_buffer().ok_or("unable to create position buffer")?;
        let position_array = float32_array!(positions);
        ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &position_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let color_buffer = ctx.create_buffer().ok_or("unable to create color buffer")?;
        let color_array = {
            let color_vec: Vec<f32> = face_colors
                .iter()
                .map(|row| vec![row, row, row, row])
                .flatten()
                .flatten()
                .map(|x| *x)
                .collect();

            let mut color_arr: [f32; 96] = [0f32; 96];
            color_arr.copy_from_slice(color_vec.as_slice());

            float32_array!(color_arr)
        };
        ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let index_buffer = ctx.create_buffer().ok_or("unable to create index buffer")?;
        let index_array = uint16_array!(indices);
        ctx.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        Ok(Buffers(position_buffer, color_buffer, index_buffer))
    }

    pub fn draw_scene(&self, program_info: ProgramInfo, buffers: Buffers, theta: f32, phi: f32) -> Result<(), JsValue> {
        let ctx = self.context.clone();

        let Buffers(
            position_buffer,
            color_buffer,
            index_buffer,
        ) = buffers;
        let ProgramInfo(
            shader_program,
            (vertex_position, vertex_color),
            (location_projection_matrix, location_model_view_matrix),
        ) = program_info;

        ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear_depth(1.0);
        ctx.enable(WebGl2RenderingContext::DEPTH_TEST);
        ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        let canvas: web_sys::HtmlCanvasElement = ctx
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        ctx.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        let mut projection_matrix = mat4::new_zero();
        let field_of_view = 45.0 * PI / 180.0;
        let aspect: f32 = canvas.width() as f32 / canvas.height() as f32;
        let z_near = 1.0;
        let z_far = 100.0;
        mat4::perspective(&mut projection_matrix, &field_of_view, &aspect, &z_near, &z_far);

        let mut model_view_matrix = mat4::new_identity();
        let mat_to_translate = model_view_matrix.clone();
        mat4::translate(&mut model_view_matrix, &mat_to_translate, &[-0.0, 0.0, -6.0]);

        let mat_to_rotate = model_view_matrix.clone();
        mat4::rotate_x(&mut model_view_matrix, &mat_to_rotate, &phi);

        let mat_to_rotate = model_view_matrix.clone();
        mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &theta);

        {
            let num_components = 3;
            let type_ = WebGl2RenderingContext::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;

            ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
            ctx.vertex_attrib_pointer_with_i32(
                vertex_position,
                num_components,
                type_,
                normalize,
                stride,
                offset,
            );
            ctx.enable_vertex_attrib_array(vertex_position);
        }
        {
            let num_components = 4;
            let type_ = WebGl2RenderingContext::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;

            ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
            ctx.vertex_attrib_pointer_with_i32(
                vertex_color,
                num_components,
                type_,
                normalize,
                stride,
                offset,
            );
            ctx.enable_vertex_attrib_array(vertex_color);
        }

        ctx.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        ctx.use_program(Some(&shader_program));
        ctx.uniform_matrix4fv_with_f32_array(Some(&location_projection_matrix?), false, &projection_matrix);
        ctx.uniform_matrix4fv_with_f32_array(Some(&location_model_view_matrix?), false, &model_view_matrix);

        {
            let vertex_count = 36;
            let type_ = WebGl2RenderingContext::UNSIGNED_SHORT;
            let offset = 0;

            ctx.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, vertex_count, type_, offset);
        }

        Ok(())
    }

    fn compile_shader(&self, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        let ctx = self.context.clone();

        let shader = ctx.create_shader(shader_type)
            .ok_or_else(|| String::from("unable to create shader"))?;
        ctx.shader_source(&shader, source);
        ctx.compile_shader(&shader);

        if ctx.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
            Ok(shader)
        } else {
            Err(ctx.get_shader_info_log(&shader).unwrap_or_else(|| String::from("unable to get shader parameter")))
        }
    }

    fn link_program(&self, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
        let ctx = self.context.clone();

        let program = ctx.create_program()
            .ok_or_else(|| String::from("unable to create program"))?;
        ctx.attach_shader(&program, vert_shader);
        ctx.attach_shader(&program, frag_shader);
        ctx.link_program(&program);

        if ctx.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            Ok(program)
        } else {
            Err(ctx.get_program_info_log(&program).unwrap_or_else(|| String::from("unable to get program parameter")))
        }
    }
}
