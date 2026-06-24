#[derive(Copy, Clone)]
pub struct Car {
	pub id: u8,
	pub direction: CarDirection,
	pub point: Point,
}

#[derive(Clone, Copy)]
pub enum CarDirection {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
	pub x: i8,
	pub y: i8,
}
