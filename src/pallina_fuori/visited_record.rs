use crate::pallina_fuori::Board;
use crate::pallina_fuori::piece_move::{PieceMove};

pub struct VisitedRecord {
	pub g_score: usize,
	pub parent: Option<Board>,
	pub move_made: Option<PieceMove>,
}
