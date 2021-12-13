use super::super::coordinates as Coordinates_fun;

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
}

impl GameBox {
	pub fn new(height: f32, width: f32, scale: f32, horse_position: HorsePosition) -> Self {
		let size = box_size(height, width, scale);
		Self {
			size,
			position: box_position_in_plane(height, width, size),
			horse_position,
		}
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
