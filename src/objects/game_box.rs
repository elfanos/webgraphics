pub struct GameBox {
	width: f32,
	height: f32,
	position: f32,
}

impl GameBox {
	pub fn new(width: f32, height: f32, position: f32) -> Self {
		Self {
			width,
			height,
			position,
		}
	}
}
