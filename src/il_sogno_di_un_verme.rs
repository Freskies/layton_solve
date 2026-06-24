use crate::il_sogno_di_un_verme::board::Board;

pub mod mela;
pub mod board;
pub mod point;

pub fn solve() {
	let b: Board = Board::init();
	let solution = b.solve();
	for step in solution {
		println!("{}", step)
	}
}
