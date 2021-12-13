use super::position::Position;

#[derive(Copy, Clone)]
pub struct GameBox {
	width: f32,
	height: f32,
	position: Position,
}

impl GameBox {
	pub fn new(width: f32, height: f32, position: Position) -> Self {
		Self {
			width,
			height,
			position,
		}
	}
	pub fn get_width(&self) -> f32 {
		self.width
	}
	pub fn get_height(&self) -> f32 {
		self.height
	}
	pub fn get_position(&self) -> Position {
		let top = self.position.get_top();
		let left = self.position.get_left();
		let bottom = self.position.get_bottom();
		let right = self.position.get_right();

		Position::new(
			bottom,
			top,
			left,
			right
		)
	}
}
