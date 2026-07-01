use std::cmp::Ordering;
use crate::il_sogno_di_un_verme::board::Board;

#[derive(Eq, PartialEq)]
pub struct BoardState {
	pub f_score: usize, // g + h
	pub g_score: usize,
	pub board: Board,
}

impl Ord for BoardState {
	fn cmp(&self, other: &Self) -> Ordering {
		other
			.f_score
			.cmp(&self.f_score)
			.then_with(|| self.g_score.cmp(&other.g_score))
	}
}

impl PartialOrd for BoardState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}