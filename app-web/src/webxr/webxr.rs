use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use js_sys::Promise;
use wasm_bindgen::{JsCast, JsValue, prelude::*};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{EventTarget, MouseEvent, Navigator, XrFrame, XrRenderStateInit, XrSession, XrSessionMode, XrWebGlLayer};

use crate::webgl::{buffer::Buffers, program::ProgramInfo, webgl::WebGL};

const AMORTIZATION: f32 = 0.95;

pub struct WebXR {
    session: Rc<RefCell<Option<XrSession>>>,
    gl: WebGL,
}

impl WebXR {
    pub fn new() -> WebXR {
        WebXR {
            session: Rc::new(RefCell::new(None)),
            gl: WebGL::new(true).unwrap(),
        }
    }

    pub fn initialize_session(&self) -> Promise {
        let session = self.session.clone();
        let ctx = self.gl.context.clone();

        let navigator: Navigator = web_sys::window().unwrap().navigator();
        let xr = navigator.xr();

        let session_mode = XrSessionMode::Inline;
        let session_supported_promise = xr.is_session_supported(session_mode);

        let future = async move {
            let supports_session = JsFuture::from(session_supported_promise).await.unwrap();
            if supports_session == false {
                return Ok(JsValue::from("unsupported XR session"));
            }

            let xr_session_promise = xr.request_session(session_mode);
            let xr_session: XrSession = JsFuture::from(xr_session_promise).await.unwrap().into();

            let xr_gl_layer = XrWebGlLayer::new_with_web_gl2_rendering_context(&xr_session, &ctx)?;
            let mut render_state_init = XrRenderStateInit::new();
            render_state_init.base_layer(Some(&xr_gl_layer));
            xr_session.update_render_state_with_state(&render_state_init);

            let mut session = session.borrow_mut();
            session.replace(xr_session);

            Ok(JsValue::from("complete initialize session"))
        };

        future_to_promise(future)
    }

    pub fn start(&self) -> Result<(), JsValue> {
        let webgl = self.gl.clone();
        let ctx = webgl.context.clone();
        let canvas = webgl.canvas.clone();

        let shader_program = webgl.init_shader_program(
            include_str!("../../shader/vs.vert"),
            include_str!("../../shader/fs.frag"),
        )?;
        let program_info = {
            let vertex_position = ctx.get_attrib_location(&shader_program, "aVertexPosition") as u32;
            let vertex_color = ctx.get_attrib_location(&shader_program, "aVertexColor") as u32;
            let projection_matrix = ctx.get_uniform_location(&shader_program, "uProjectionMatrix")
                .ok_or_else(|| String::from("cannot get uProjectionMatrix"));
            let model_view_matrix = ctx.get_uniform_location(&shader_program, "uModelViewMatrix")
                .ok_or_else(|| String::from("cannot get uModelViewMatrix"));
            ProgramInfo(
                shader_program,
                (vertex_position, vertex_color),
                (projection_matrix, model_view_matrix),
            )
        };
        let buffers: Buffers = webgl.init_buffers()?;

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let drag = Rc::new(RefCell::new(false));
        let canvas_width = Rc::new(RefCell::new(canvas.width() as f32));
        let canvas_height = Rc::new(RefCell::new(canvas.height() as f32));
        let dx = Rc::new(RefCell::new(0.0));
        let dy = Rc::new(RefCell::new(0.0));
        let theta = Rc::new(RefCell::new(0.0));
        let phi = Rc::new(RefCell::new(0.0));

        let event_target: EventTarget = canvas.into();

        // add event listeners
        // mouse down
        {
            let drag = drag.clone();

            let callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *drag.borrow_mut() = true;
            }) as Box<dyn FnMut(MouseEvent)>);

            event_target
                .add_event_listener_with_callback("mousedown", callback.as_ref().unchecked_ref())
                .unwrap();
            callback.forget();
        }
        // mouse up & mouse out
        {
            let drag = drag.clone();

            let callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *drag.borrow_mut() = false;
            }) as Box<dyn FnMut(MouseEvent)>);

            event_target
                .add_event_listener_with_callback("mouseup", callback.as_ref().unchecked_ref())
                .unwrap();
            event_target
                .add_event_listener_with_callback("mouseout", callback.as_ref().unchecked_ref())
                .unwrap();
            callback.forget();
        }
        // mouse move
        {
            let drag = drag.clone();
            let canvas_width = canvas_width.clone();
            let canvas_height = canvas_height.clone();
            let dx = dx.clone();
            let dy = dy.clone();
            let theta = theta.clone();
            let phi = phi.clone();

            let callback = Closure::wrap(Box::new(move |event: MouseEvent| {
                if *drag.borrow() {
                    let cw = *canvas_width.borrow();
                    let ch = *canvas_height.borrow();
                    *dx.borrow_mut() = (event.movement_x() as f32) * 2.0 * PI / cw;
                    *dy.borrow_mut() = (event.movement_y() as f32) * 2.0 * PI / ch;
                    *theta.borrow_mut() += *dx.borrow();
                    *phi.borrow_mut() += *dy.borrow();
                }
            }) as Box<dyn FnMut(web_sys::MouseEvent)>);

            event_target
                .add_event_listener_with_callback("mousemove", callback.as_ref().unchecked_ref())
                .unwrap();
            callback.forget();
        }
        // request animation frame
        {
            let drag = drag.clone();
            let dx = dx.clone();
            let dy = dy.clone();

            *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_time: f64, frame: XrFrame| {
                if !*drag.borrow() {
                    *dx.borrow_mut() *= AMORTIZATION;
                    *dy.borrow_mut() *= AMORTIZATION;
                    *theta.borrow_mut() += *dx.borrow();
                    *phi.borrow_mut() += *dy.borrow();
                }

                webgl.draw_scene(
                    program_info.clone(),
                    buffers.clone(),
                    *theta.borrow(),
                    *phi.borrow(),
                ).unwrap();

                let sess: XrSession = frame.session();
                request_animation_frame(&sess, f.borrow().as_ref().unwrap());
            }) as Box<dyn FnMut(f64, XrFrame)>));

            let session: &Option<XrSession> = &self.session.borrow();
            let sess: &XrSession = if let Some(sess) = session {
                sess
            } else {
                return Ok(());
            };
            request_animation_frame(sess, g.borrow().as_ref().unwrap());
        }

        Ok(())
    }
}

/// Turns the Closure into a `js_sys::Function`
///
/// See https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html#casting-a-closure-to-a-js_sysfunction
pub fn request_animation_frame(session: &XrSession, f: &Closure<dyn FnMut(f64, XrFrame)>) -> u32 {
    session.request_animation_frame(f.as_ref().unchecked_ref())
}
