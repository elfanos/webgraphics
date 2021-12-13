extern crate serde_json;
extern crate wasm_bindgen;
use objects::GameBox as GameBox2d;
use programs::GameBox;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[macro_use]
extern crate lazy_static;
extern crate serde_derive;

mod app_state;
mod coordinates;
mod gl_setup;
mod matrix_equations;
mod objects;
mod programs;
mod shaders;
mod webgl_compiler;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    gl: WebGlRenderingContext,
    program_game_box: GameBox,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Self {
            program_game_box: GameBox::new(&gl),
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
        app_state::update_dynamic_data(time, height, width, &elements);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let curr_state = app_state::get_curr_state();
        let leader = get_leader_translation(&curr_state.boxes);
        for box_2d in &curr_state.boxes {
            self.program_game_box.render(
                &self.gl,
                curr_state.canvas_height,
                curr_state.canvas_width,
                box_2d,
                leader,
            );
        }
    }
}

pub fn get_leader_translation(boxes: &Vec<GameBox2d>) -> f32 {
    let mut leader = 0.;
    for x in boxes {
        if leader < x.horse_position.distance_covered {
            leader = x.horse_position.distance_covered;
        }
    }
    leader
}
