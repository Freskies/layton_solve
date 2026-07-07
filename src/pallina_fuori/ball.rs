use crate::pallina_fuori::point::Point;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Ball {
	pub id: u8,
	pub victory_slot: Point,
	pub position: Point,
}

impl Ball {
	pub fn duplicate(self, point: Point) -> Ball {
		Ball {
			id: self.id,
			victory_slot: self.victory_slot.clone(),
			position: point,
		}
	}
}
