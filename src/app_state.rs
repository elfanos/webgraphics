use rand::Rng;
use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

extern crate serde_json;
extern crate wasm_bindgen;
use crate::objects::game_box::GameBox;
use crate::objects::position::Position;

lazy_static! {
	static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

/*
elements: [
	horseNumber,
	distanceToFinish,
	distanceFromInnerLane,
	distanceCovered,
]
*/

fn addGameBox() {}
// distanceToFinish - distanceCovered
// first horse,
// last horse,
//
pub fn update_dynamic_data(
	time: f32,
	canvas_height: f32,
	canvas_width: f32,
	elements: Vec<Vec<f32>>,
) {
	let min_heigth_width = canvas_height.min(canvas_width);

	let display_size = min_heigth_width;
	let half_display_size = display_size / 14.; // scaling the box or something?
	
	let half_canvas_height = canvas_height / 2.;
	let half_canvas_width = canvas_width / 2.;

	// let together = format!(
	// 	"aspecto {:?} {:?} {:?} {:?} {:?}",
	// 	min_heigth_width, display_size, half_display_size, half_canvas_height, half_canvas_width
	// );
	// log(&together);

	// // creating a game box
	// let mut test = elements;
	// test.sort_by(|a, b| {
	// 	let a_distance = a[1] - a[3];
	// 	let b_distance = b[1] - b[3];
	// 	a_distance.partial_cmp(&b_distance).unwrap()
	// });
	// let mut boxes = Vec::new();
	// let mut rng = rand::thread_rng();
	// for i in 0..test.len() {
	// 	let position = Position::new(
	// 		rng.gen_range(0.0..100.0),
	// 		rng.gen_range(0.0..100.0),
	// 		rng.gen_range(0.0..100.0),
	// 		rng.gen_range(0.0..100.0),
	// 	);
	// 	boxes.push(GameBox::new(10., 10., position))
	// }
	// let together = format!("test colio {:?}", test[0][0]);
	// log(&together);


	let mut boxes = Vec::new();
	let mut data = APP_STATE.lock().unwrap();

	*data = Arc::new(AppState {
		canvas_height,
		canvas_width,
		control_bottom: half_canvas_height - half_display_size ,
		control_top: half_canvas_height + half_display_size,
		control_left: half_canvas_width - half_display_size,
		control_right: half_canvas_width + half_display_size,
		boxes,
		time,
		..*data.clone()
	})
}

pub fn get_curr_state() -> Arc<AppState> {
	APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
	pub canvas_height: f32,
	pub canvas_width: f32,
	pub control_bottom: f32,
	pub control_top: f32,
	pub control_left: f32,
	pub control_right: f32,
	pub boxes: Vec<GameBox>,
	pub time: f32,
}

impl AppState {
	fn new() -> Self {
		Self {
			canvas_height: 0.,
			canvas_width: 0.,
			control_bottom: 0.,
			control_top: 0.,
			control_left: 0.,
			control_right: 0.,
			boxes: Vec::new(),
			time: 0.,
		}
	}
}
