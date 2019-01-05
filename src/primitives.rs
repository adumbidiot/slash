pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {
	pub fn new(x: f32, y: f32) -> Self {
		Point {
			x,
			y
		}
	}
}

pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Rect {
			x,
			y,
			width,
			height,
		}
	}
}