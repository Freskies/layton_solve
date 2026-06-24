use crate::salto_del_cavallo::{X_START, Y_START};
use crate::salto_del_cavallo::horse_move::HorseMove;

#[derive(Debug)]
pub struct Point {
	pub x: i8,
	pub y: i8,
}

impl Point {
	pub fn init() -> Self {
		Self {
			x: X_START as i8,
			y: Y_START as i8,
		}
	}

	pub fn add(&self, p: &HorseMove) -> HorseMove {
		HorseMove {
			point: Point {
				x: self.x + p.point.x,
				y: self.y + p.point.y,
			},
			name: p.name,
		}
	}

	pub fn print(&self) {
		print!("({:?},{:?})", self.x, self.y);
	}
}
