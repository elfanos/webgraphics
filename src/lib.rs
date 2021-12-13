extern crate serde_json;
extern crate wasm_bindgen;
use programs::{Color2D, Color2DGradient, Box2d,GameBox};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[macro_use]
extern crate lazy_static;
extern crate serde_derive;

mod app_state;
mod common_funcs;
mod gl_setup;
mod objects;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    gl: WebGlRenderingContext,
    program_color_2d: Color2D,
    program_game_box: GameBox,
    program_color_2d_gradient: Color2DGradient,
    program_box_2d: Box2d
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Self {
            program_color_2d: Color2D::new(&gl),
            program_game_box: GameBox::new(&gl),
            program_color_2d_gradient: Color2DGradient::new(&gl),
            program_box_2d:Box2d::new(&gl),
            gl,
        }
    }

    pub fn update(
        &mut self,
        time: f32,
        height: f32,
        width: f32,
        array: JsValue,
    ) -> Result<(), JsValue> {
        let elements: Vec<Vec<f32>> = array.into_serde().unwrap();
        app_state::update_dynamic_data(time, height, width, elements);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let curr_state = app_state::get_curr_state();
        self.program_game_box.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
        );

        // self.program_color_2d_gradient.render(
        //     &self.gl,
        //     curr_state.control_bottom + 20.,
        //     curr_state.control_top - 20.,
        //     curr_state.control_left + 20.,
        //     curr_state.control_right - 20.,
        //     curr_state.canvas_height,
        //     curr_state.canvas_width,
        // );
        // for game_box in &curr_state.boxes {
        //     self.program_box_2d.render(&self.gl, &game_box)
        // }
    }
}
