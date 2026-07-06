use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum LegalMove {
	UP,
	RIGHT,
	DOWN,
	LEFT,
}

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

#[derive(Copy, Clone)]
pub struct PieceMove {
	pub piece: char,
	pub legal_move: LegalMove,
}
