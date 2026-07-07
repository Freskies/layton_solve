use crate::klotski::point::Point;

// Very Important Piece

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Vip {
	pub id: u8,
	pub victory_slot: Point,
	pub position: Point,
}

impl Vip {
	pub fn duplicate(self, point: Point) -> Vip {
		Vip {
			id: self.id,
			victory_slot: self.victory_slot.clone(),
			position: point,
		}
	}
}
