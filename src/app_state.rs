use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

extern crate serde_json;
extern crate wasm_bindgen;
use crate::coordinates as Coordinates;
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
	let mut data = APP_STATE.lock().unwrap();

	let col = data.clone();

	let mut prev_boxes = Vec::new();

	let mut boxes = Vec::new();
	for item in elements {
		boxes.push(GameBox::new(
			canvas_height,
			canvas_width,
			14.,
			item[3],
			item[2],
			HorsePosition {
				horse_number: item[0],
				distance_to_finish: item[1],
				distance_from_inner_lane: item[2],
				distance_covered: item[3],
			},
		))
	}
	let comparing = elements[0][3];
	let take = col.boxes.get(0);
	match take {
		None => println!("not goodie"),
		Some(take) => {
			if take.horse_position.distance_covered.to_bits() == comparing.to_bits() {
				//increase distance to finish
				for prev_box_2d in &col.prev_boxes {
					let box_2d = &col.boxes.clone().into_iter().find(|&x| {
						x.horse_position.horse_number == prev_box_2d.horse_position.horse_number
					});
					match box_2d {
						None => println!("not good in getting next unit in velocity calculation"),
						Some(box_2d) => {
							let coming_leader = Coordinates::get_race_leader(&col.boxes);
							let coming_x_y = Coordinates::get_vector_point(
								canvas_width,
								canvas_height,
								box_2d.horse_position.distance_covered,
								box_2d.horse_position.distance_from_inner_lane,
								coming_leader,
							);

							let next_x_y = Coordinates::get_vector_point(
								canvas_width,
								canvas_height,
								prev_box_2d.horse_position.distance_covered,
								prev_box_2d.horse_position.distance_from_inner_lane,
								coming_leader,
							);

							let next_distance_covered = Coordinates::get_next_distance(
								coming_x_y.xt - next_x_y.xt,
								1000.,
								1000. / 24.,
							);

							let next_prev_box_2d = crate::objects::game_box::move_horse(
								prev_box_2d.distance_to_point,
								next_distance_covered,
							);


							let next_inner_lane = Coordinates::get_next_distance(
								box_2d.horse_position.distance_from_inner_lane - prev_box_2d.horse_position.distance_from_inner_lane,
								1000.,
								1000. / 24.,
							);

							let next_prev_box_inner_lane = crate::objects::game_box::move_horse(
								prev_box_2d.distance_to_point_y,
								next_inner_lane,
							);

							prev_boxes.push(GameBox::new(
								canvas_height,
								canvas_width,
								14.,
								next_prev_box_2d,
								next_prev_box_inner_lane,
								prev_box_2d.horse_position,
							))
						}
					}
				}
			} else {
				// reset
				for box_2d in &col.boxes {
					prev_boxes.push(*box_2d)
				}
			}
		}
	}
	let mut new_frame = col.frame + 1.;
	if new_frame > 30. {
		new_frame = 0.;
	}

	*data = Arc::new(AppState {
		canvas_height,
		canvas_width,
		boxes,
		time,
		prev_boxes,
		frame: new_frame,
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
	pub prev_boxes: Vec<GameBox>,
	pub time: f32,
	pub frame: f32,
}

impl AppState {
	fn new() -> Self {
		Self {
			canvas_height: 0.,
			canvas_width: 0.,
			boxes: Vec::new(),
			prev_boxes: Vec::new(),
			time: 0.,
			frame: 0.,
		}
	}
}
