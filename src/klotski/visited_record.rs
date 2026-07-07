use crate::klotski::Board;
use crate::klotski::piece_move::{PieceMove};

pub struct VisitedRecord {
	pub g_score: usize,
	pub parent: Option<Board>,
	pub move_made: Option<PieceMove>,
}
