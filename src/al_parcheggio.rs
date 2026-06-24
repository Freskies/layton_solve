use crate::al_parcheggio::board::Board;

pub mod car;
pub mod board;

pub fn solve() {
	let b: Board = Board::init();
	b.print()
}