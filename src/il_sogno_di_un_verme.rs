use crate::il_sogno_di_un_verme::board::Board;
use crate::il_sogno_di_un_verme::mela::Mela;
use crate::il_sogno_di_un_verme::point::Point;

pub mod mela;
pub mod board;
pub mod point;
pub mod board_state;

pub fn solve() {
	let mut board = [[None; 3]; 3];
	board[0][0] = Some(Mela::init(9));
	board[0][1] = Some(Mela::init(2));
	board[0][2] = Some(Mela::init(3));
	board[1][0] = Some(Mela::init(4));
	board[1][1] = Some(Mela::init(8));
	board[1][2] = Some(Mela::init(7));
	board[2][0] = Some(Mela::init(6));
	board[2][2] = Some(Mela::init(1));

	let b: Board = Board::init(board, Point(2, 1));
	let solution = b.solve();
	for step in solution {
		println!("{}", step)
	}
}

pub fn solve_labirinto_a_tasselli() {
	let mut board = [[None; 3]; 3];
	board[0][0] = Some(Mela::init(5));
	board[0][1] = Some(Mela::init(7));
	board[0][2] = Some(Mela::init(6));
	board[1][0] = Some(Mela::init(1));
	board[1][2] = Some(Mela::init(4));
	board[2][0] = Some(Mela::init(3));
	board[2][1] = Some(Mela::init(2));
	board[2][2] = Some(Mela::init(8));

	let b: Board = Board::init(board, Point(1, 1));
	let solution = b.solve();
	for step in solution {
		println!("{}", step)
	}
}