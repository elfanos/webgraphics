#[derive(Copy, Clone)]
pub struct Position {
	pub top: f32,
	pub bottom: f32,
	pub right: f32,
	pub left: f32,
}

#[derive(Copy, Clone)]
pub struct HorsePosition {
	pub horse_number: f32,
	pub distance_to_finish: f32,
	pub distance_from_inner_lane: f32,
	pub distance_covered: f32,
}

#[derive(Copy, Clone)]
pub struct GameBox {
	pub size: f32,
	pub position: Position,
	pub horse_position: HorsePosition,
	pub distance_to_point: f32,
	pub distance_to_point_y: f32,
}

impl GameBox {
	pub fn new(
		height: f32,
		width: f32,
		scale: f32,
		distance_to_point: f32,
		distance_to_point_y: f32,
		horse_position: HorsePosition,
	) -> Self {

		let size = box_size(height, width, scale);
		Self {
			size,
			position: box_position_in_plane(height, width, size),
			horse_position,
			distance_to_point,
			distance_to_point_y,
		}
	}
	pub fn move_box(&mut self, velocity: f32) {
		self.distance_to_point = self.distance_to_point + velocity
	}
}

/**
 * Adding scale to the box
 */
pub fn box_size(height: f32, width: f32, scale: f32) -> f32 {
	height.min(width) / scale
}

pub fn box_position_in_plane(height: f32, width: f32, size: f32) -> Position {
	let half_height = height / 2.;
	let half_width = width / 2.;
	Position {
		top: half_height + size,
		bottom: half_height - size,
		right: half_width + size,
		left: half_width - size,
	}
}

pub fn move_horse(distance_covered: f32, velocity: f32) -> f32 {
	distance_covered + velocity
}


/**
 * Using distance/time = velocity then to calculate unit per frame velocity/(fps => 1000 / 60 or 1000 / 30) = units per frame based on the speed
 *
 */
pub fn get_next_x_y_with_velocity(distance: f32, time: f32, fps: f32) -> f32 {
    let units_per_frame = (distance / time) * fps;
    units_per_frame
}
pub fn get_leader_translation(boxes: &Vec<GameBox>) -> f32 {
    let mut leader = 0.;
    for x in boxes {
        if leader < x.horse_position.distance_covered {
            leader = x.horse_position.distance_covered;
        }
    }
    leader
}
