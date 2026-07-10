use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum LegalMove {
	UP,
	RIGHT,
	DOWN,
	LEFT,
}

pub const DIRECTIONS: [LegalMove; 4] = [
	LegalMove::UP, LegalMove::RIGHT, LegalMove::DOWN, LegalMove::LEFT
];

impl Display for LegalMove {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let direction = match self {
			LegalMove::UP => "su",
			LegalMove::RIGHT => "destra",
			LegalMove::DOWN => "giù",
			LegalMove::LEFT => "sinistra",
		};

		f.write_str(direction)
	}
}

#[derive(Clone, Debug)]
pub struct PieceMove {
	pub piece: u8,
	pub moves: Vec<LegalMove>,
}
