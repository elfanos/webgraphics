use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

extern crate serde_json;
extern crate wasm_bindgen;
use crate::objects::game_box::{GameBox, HorsePosition};

lazy_static! {
	static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}


pub fn update_dynamic_data(
	time: f32,
	canvas_height: f32,
	canvas_width: f32,
	elements: &Vec<Vec<f32>>,
) {

	let mut boxes = Vec::new();
	for item in elements {
		boxes.push(GameBox::new(
			canvas_height,
			canvas_width,
			14.,
			HorsePosition {
				horse_number: item[0],
				distance_to_finish: item[1],
				distance_from_inner_lane: item[2],
				distance_covered: item[3],
			},
		))
	}
	let mut data = APP_STATE.lock().unwrap();

	let box_2d = GameBox::new(
		canvas_height,
		canvas_width,
		14.,
		HorsePosition {
			distance_covered: 0.,
			distance_from_inner_lane: 0.,
			distance_to_finish: 0.2,
			horse_number: 0.2,
		},
	);

	*data = Arc::new(AppState {
		canvas_height,
		canvas_width,
		boxes,
		box_2d,
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
	pub boxes: Vec<GameBox>,
	pub box_2d: GameBox,
	pub time: f32,
}

impl AppState {
	fn new() -> Self {
		Self {
			canvas_height: 0.,
			canvas_width: 0.,
			boxes: Vec::new(),
			box_2d: GameBox::new(
				0.,
				0.,
				0.,
				HorsePosition {
					distance_covered: 0.,
					distance_from_inner_lane: 0.,
					distance_to_finish: 0.,
					horse_number: 0.,
				},
			),
			time: 0.,
		}
	}
}
