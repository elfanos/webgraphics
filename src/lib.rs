extern crate serde_json;
extern crate wasm_bindgen;
use coordinates as Coordinates;
use objects::{GameBox as GameBox2d, MatrixtTranslation};
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
    run: bool,
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
            run: false,
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
        let leader = Coordinates::get_race_leader(&curr_state.boxes);
        let prev_leader = Coordinates::get_race_leader(&curr_state.prev_boxes);

        let box_2d = &curr_state.boxes[0];
        let prev_box_2d = curr_state.prev_boxes.get(0);
        // let frame_string = format!(
        //     "current frame {}",
        //     curr_state.frame,
        // );
        // log(&frame_string);

        // match prev_box_2d {
        //     None => println!("i"),
        //     Some(prev_box_2d) => {
        //         let new_tarnslation = get_vector_point(
        //             curr_state.canvas_width,
        //             curr_state.canvas_height,
        //             box_2d.horse_position.distance_covered,
        //             box_2d.horse_position.distance_from_inner_lane,
        //             leader,
        //         );

        //         let old_tarnslation = get_vector_point(
        //             curr_state.canvas_width,
        //             curr_state.canvas_height,
        //             prev_box_2d.horse_position.distance_covered,
        //             prev_box_2d.horse_position.distance_from_inner_lane,
        //             leader,
        //         );

        //             // let test = format!(
        //             //     "x difference in translation = {} and distance from inner lane {}",
        //             //     prev_box_2d.distance_to_point_y,
        //             //     box_2d.horse_position.distance_from_inner_lane
        //             // );
        //             // log(&test);
        //         self.program_game_box.render(
        //             &self.gl,
        //             curr_state.canvas_height,
        //             curr_state.canvas_width,
        //             box_2d,
        //             get_vector_point_in_plane(
        //                 curr_state.canvas_width,
        //                 curr_state.canvas_height,
        //                 prev_box_2d.distance_to_point,
        //                 prev_box_2d.distance_to_point_y,
        //                 prev_leader,
        //             ),
        //         );
        //     }
        // }

        for box_2d in &curr_state.boxes {
            let prev_box_2d = curr_state
                .prev_boxes
                .clone()
                .into_iter()
                .find(|&x| x.horse_position.horse_number == box_2d.horse_position.horse_number);
            match prev_box_2d {
                None => println!("!"),
                Some(prev_box_2d) => {
                    self.program_game_box.render(
                        &self.gl,
                        curr_state.canvas_height,
                        curr_state.canvas_width,
                        box_2d,
                        Coordinates::get_vector_point_in_plane(
                            curr_state.canvas_width,
                            curr_state.canvas_height,
                            prev_box_2d.distance_to_point,
                            prev_box_2d.distance_to_point_y,
                            prev_leader,
                        ),
                    );
                }
            }
        }
    }
}
