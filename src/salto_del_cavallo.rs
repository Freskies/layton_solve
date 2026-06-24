use crate::salto_del_cavallo::Variant::*;
use crate::salto_del_cavallo::board::Board;

pub mod board;
pub mod cell;
pub mod point;
pub mod horse_move;

#[allow(unused)]
enum Variant {
	SaltoDelCavallo1,
	SaltoDelCavallo2,
	SaltoDelCavallo3,
	SaltoDelCavallo4,
}

pub const X_START: usize = 0;
pub const Y_START: usize = 0;

fn enigma_size(enigma: &Variant) -> (usize, usize) {
	match enigma {
		SaltoDelCavallo1 => (3, 4),
		SaltoDelCavallo2 => (5, 7),
		SaltoDelCavallo3 => (6, 6),
		SaltoDelCavallo4 => (8, 8),
	}
}

pub fn solve_1() {
	solve(SaltoDelCavallo1);
}

pub fn solve_2() {
	solve(SaltoDelCavallo2);
}

pub fn solve_3() {
	solve(SaltoDelCavallo3);
}

pub fn solve_4() {
	solve(SaltoDelCavallo4);
}

fn solve(enigma: Variant) {
	let (x_size, y_size) = enigma_size(&enigma);
	let board: Board = Board::init(x_size, y_size);

	for step in board.solution.iter().rev() {
		println!("{}", step);
	}
}
