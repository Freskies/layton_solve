use crate::klotski::board::Board;
use crate::klotski::klotski_solver::KlotskiSolver;
use crate::klotski::piece_move::PieceMove;
use crate::klotski::point::Point;
use crate::klotski::vip::Vip;
use std::collections::HashMap;

pub mod board;
pub mod klotski_solver;
pub mod node;
pub mod piece_move;
pub mod point;
pub mod symmetry_meta;
pub mod vip;
pub mod visited_record;
/*
0 -> empty space
1/8 -> balls
9 -> wall
letters -> pieces
*/

pub type Grid = Vec<Vec<u8>>;

fn solve(
	board: Board,
	width: usize,
	height: usize,
	vips: Vec<Vip>,
	translations: HashMap<u8, &str>,
) {
	let mut pallina_solve: KlotskiSolver = KlotskiSolver::init(board, width, height, vips);
	pallina_solve.a_star();
	if let Some(path) = &pallina_solve.victory_path {
		print_solution(path, translations);
	} else {
		println!("L'enigma è impossibile o io sono un cane a programmare.");
	}
}

fn board_to_board(grid: Grid) -> Board {
	let mut data = [0; 72];
	let mut len = 0;

	for row in grid {
		for col in row {
			if len < 72 {
				data[len] = col;
				len += 1;
			}
		}
	}

	Board { data, len }
}

fn print_solution(solution: &Vec<PieceMove>, translations: HashMap<u8, &str>) {
	for piece_move in solution {
		println!();
		print!(
			"{}",
			translations.get(&piece_move.piece).unwrap().to_string()
		);
		for legal_move in &piece_move.moves {
			print!(" {}", legal_move);
		}
	}
}

// ENIGMI

pub fn solve_pallina_fuori_1() {
	let board: Grid = vec![
		vec![b'9', b'9', b'1', b'9', b'9'],
		vec![b'a', b'b', b'b', b'0', b'0'],
		vec![b'a', b'b', b'b', b'c', b'c'],
		vec![b'd', b'd', b'e', b'e', b'f'],
		vec![b'0', b'0', b'e', b'e', b'g'],
		vec![b'9', b'9', b'0', b'9', b'9'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Pallina");
	translations.insert(b'a', "Verde");
	translations.insert(b'b', "Giallo");
	translations.insert(b'c', "Blu");
	translations.insert(b'd', "Blu");
	translations.insert(b'e', "Giallo");
	translations.insert(b'f', "Viola");
	translations.insert(b'g', "Viola");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		victory_slot: Point(2, 5),
		position: Point(2, 0),
	}];

	let board: Board = board_to_board(board);
	solve(board, 5, 6, balls, translations);
}

pub fn solve_pallina_fuori_2() {
	let board: Grid = vec![
		vec![b'9', b'9', b'1', b'9', b'9', b'9'],
		vec![b'0', b'a', b'a', b'b', b'c', b'0'],
		vec![b'9', b'd', b'e', b'e', b'c', b'9'],
		vec![b'9', b'f', b'e', b'e', b'g', b'9'],
		vec![b'0', b'f', b'h', b'i', b'i', b'0'],
		vec![b'9', b'9', b'9', b'0', b'9', b'9'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Pallina");
	translations.insert(b'a', "Blu");
	translations.insert(b'b', "Viola");
	translations.insert(b'c', "Verde");
	translations.insert(b'd', "Viola");
	translations.insert(b'e', "Giallo");
	translations.insert(b'f', "Verde");
	translations.insert(b'g', "Viola");
	translations.insert(b'h', "Viola");
	translations.insert(b'i', "Blu");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		victory_slot: Point(3, 5),
		position: Point(2, 0),
	}];

	let board: Board = board_to_board(board);
	solve(board, 6, 6, balls, translations);
}

pub fn solve_pallina_fuori_3() {
	let board: Grid = vec![
		vec![b'9', b'9', b'1', b'9', b'9', b'9'],
		vec![b'0', b'a', b'a', b'b', b'c', b'0'],
		vec![b'd', b'd', b'e', b'e', b'c', b'9'],
		vec![b'9', b'f', b'e', b'e', b'g', b'g'],
		vec![b'0', b'f', b'h', b'i', b'i', b'0'],
		vec![b'9', b'9', b'9', b'0', b'9', b'9'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Pallina");
	translations.insert(b'a', "Blu");
	translations.insert(b'b', "Viola");
	translations.insert(b'c', "Verde");
	translations.insert(b'd', "Blu");
	translations.insert(b'e', "Giallo");
	translations.insert(b'f', "Verde");
	translations.insert(b'g', "Blu");
	translations.insert(b'h', "Viola");
	translations.insert(b'i', "Blu");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		victory_slot: Point(3, 5),
		position: Point(2, 0),
	}];

	let board: Board = board_to_board(board);
	solve(board, 6, 6, balls, translations);
}

pub fn solve_pallina_fuori_4() {
	let board: Grid = vec![
		vec![b'1', b'9', b'9', b'0'],
		vec![b'a', b'a', b'b', b'c'],
		vec![b'd', b'e', b'e', b'f'],
		vec![b'd', b'e', b'e', b'f'],
		vec![b'g', b'h', b'i', b'i'],
		vec![b'0', b'9', b'9', b'0'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Pallina");
	translations.insert(b'a', "Blu");
	translations.insert(b'b', "Viola");
	translations.insert(b'c', "Viola");
	translations.insert(b'd', "Verde");
	translations.insert(b'e', "Giallo");
	translations.insert(b'f', "Verde");
	translations.insert(b'g', "Viola");
	translations.insert(b'h', "Viola");
	translations.insert(b'i', "Blu");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		victory_slot: Point(3, 5),
		position: Point(0, 0),
	}];

	let board: Board = board_to_board(board);
	solve(board, 4, 6, balls, translations);
}

pub fn solve_le_4_palline() {
	let board: Grid = vec![
		vec![b'9', b'9', b'9', b'9', b'1', b'9'],
		vec![b'2', b'0', b'a', b'a', b'0', b'9'],
		vec![b'9', b'b', b'c', b'c', b'd', b'9'],
		vec![b'9', b'b', b'c', b'c', b'd', b'9'],
		vec![b'9', b'0', b'e', b'e', b'0', b'3'],
		vec![b'9', b'4', b'9', b'9', b'9', b'9'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Pallina Verde");
	translations.insert(b'2', "Pallina Blu");
	translations.insert(b'3', "Pallina Gialla");
	translations.insert(b'4', "Pallina Rossa");
	translations.insert(b'a', "Blu");
	translations.insert(b'b', "Verde");
	translations.insert(b'c', "Giallo");
	translations.insert(b'd', "Verde");
	translations.insert(b'e', "Blu");

	let balls: Vec<Vip> = vec![
		Vip {
			id: b'1',
			victory_slot: Point(1, 5),
			position: Point(4, 0),
		},
		Vip {
			id: b'2',
			victory_slot: Point(5, 4),
			position: Point(0, 1),
		},
		Vip {
			id: b'3',
			victory_slot: Point(0, 1),
			position: Point(5, 4),
		},
		Vip {
			id: b'4',
			victory_slot: Point(4, 0),
			position: Point(1, 5),
		},
	];

	let board: Board = board_to_board(board);
	solve(board, 6, 6, balls, translations);
}

pub fn solve_fuggi_principessa_1() {
	let board: Grid = vec![
		vec![b'a', b'a', b'b', b'c', b'd'],
		vec![b'1', b'1', b'e', b'f', b'0'],
		vec![b'1', b'1', b'e', b'g', b'0'],
		vec![b'h', b'h', b'i', b'j', b'k'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Rosso");
	translations.insert(b'a', "Blu");
	translations.insert(b'b', "Verde");
	translations.insert(b'c', "Verde");
	translations.insert(b'd', "Verde");
	translations.insert(b'e', "Viola");
	translations.insert(b'f', "Verde");
	translations.insert(b'g', "Verde");
	translations.insert(b'h', "Blu");
	translations.insert(b'i', "Verde");
	translations.insert(b'j', "Verde");
	translations.insert(b'k', "Verde");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		victory_slot: Point(3, 1),
		position: Point(0, 1),
	}];

	let board: Board = board_to_board(board);
	solve(board, 5, 4, balls, translations);
}

pub fn solve_fuggi_principessa_2() {
	let board: Grid = vec![
		vec![b'a', b'a', b'b', b'c', b'd'],
		vec![b'1', b'1', b'e', b'c', b'0'],
		vec![b'1', b'1', b'f', b'g', b'0'],
		vec![b'h', b'h', b'i', b'g', b'j'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Rosso");
	translations.insert(b'a', "Blu");
	translations.insert(b'b', "Verde");
	translations.insert(b'c', "Viola");
	translations.insert(b'd', "Verde");
	translations.insert(b'e', "Verde");
	translations.insert(b'f', "Verde");
	translations.insert(b'g', "Viola");
	translations.insert(b'h', "Blu");
	translations.insert(b'i', "Verde");
	translations.insert(b'j', "Verde");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		victory_slot: Point(3, 1),
		position: Point(0, 1),
	}];

	let board: Board = board_to_board(board);
	solve(board, 5, 4, balls, translations);
}

pub fn solve_fuggi_principessa_3() {
	let board: Grid = vec![
		vec![b'a', b'a', b'b', b'b', b'c'],
		vec![b'1', b'1', b'd', b'e', b'0'],
		vec![b'1', b'1', b'd', b'f', b'0'],
		vec![b'g', b'g', b'h', b'h', b'i'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Rosso");
	translations.insert(b'a', "Blu");
	translations.insert(b'b', "Blu");
	translations.insert(b'c', "Verde");
	translations.insert(b'd', "Viola");
	translations.insert(b'e', "Verde");
	translations.insert(b'f', "Verde");
	translations.insert(b'g', "Blu");
	translations.insert(b'h', "Blu");
	translations.insert(b'i', "Verde");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		victory_slot: Point(3, 1),
		position: Point(0, 1),
	}];

	let board: Board = board_to_board(board);
	solve(board, 5, 4, balls, translations);
}

pub fn solve_rosso_blu_2() {
	let board: Grid = vec![
		vec![b'1', b'9', b'9', b'9', b'5'],
		vec![b'2', b'9', b'0', b'9', b'6'],
		vec![b'3', b'0', b'0', b'0', b'7'],
		vec![b'4', b'9', b'9', b'9', b'8'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Rosso-A");
	translations.insert(b'2', "Rosso-B");
	translations.insert(b'3', "Rosso-C");
	translations.insert(b'4', "Rosso-D");
	translations.insert(b'5', "Blu-A");
	translations.insert(b'6', "Blu-B");
	translations.insert(b'7', "Blu-C");
	translations.insert(b'8', "Blu-D");

	let balls: Vec<Vip> = vec![
		Vip {
			id: b'1',
			victory_slot: Point(4, 0),
			position: Point(0, 0),
		},
		Vip {
			id: b'2',
			victory_slot: Point(4, 1),
			position: Point(0, 1),
		},
		Vip {
			id: b'3',
			victory_slot: Point(4, 2),
			position: Point(0, 2),
		},
		Vip {
			id: b'4',
			victory_slot: Point(4, 3),
			position: Point(0, 3),
		},
		Vip {
			id: b'5',
			victory_slot: Point(0, 0),
			position: Point(4, 0),
		},
		Vip {
			id: b'6',
			victory_slot: Point(0, 1),
			position: Point(4, 1),
		},
		Vip {
			id: b'7',
			victory_slot: Point(0, 2),
			position: Point(4, 2),
		},
		Vip {
			id: b'8',
			victory_slot: Point(0, 3),
			position: Point(4, 3),
		},
	];

	let board: Board = board_to_board(board);
	solve(board, 5, 4, balls, translations);
}

pub fn solve_l_ora_delle_pulizie_1() {
	let board: Grid = vec![
		vec![b'1', b'1', b'9', b'9', b'9', b'9'],
		vec![b'1', b'1', b'9', b'9', b'9', b'9'],
		vec![b'a', b'a', b'b', b'b', b'0', b'0'],
		vec![b'a', b'a', b'b', b'b', b'9', b'9'],
		vec![b'c', b'c', b'd', b'd', b'9', b'9'],
		vec![b'e', b'e', b'f', b'f', b'9', b'9'],
		vec![b'9', b'9', b'0', b'0', b'9', b'9'],
		vec![b'9', b'9', b'0', b'0', b'9', b'9'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Spazzatura");
	translations.insert(b'a', "Giallo");
	translations.insert(b'b', "Giallo");
	translations.insert(b'c', "Azzurro");
	translations.insert(b'd', "Azzurro");
	translations.insert(b'e', "Azzurro");
	translations.insert(b'f', "Azzurro");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		position: Point(0, 0),
		victory_slot: Point(2, 6),
	}];

	let board: Board = board_to_board(board);
	solve(board, 6, 8, balls, translations);
}

pub fn solve_l_ora_delle_pulizie_2() {
	let board: Grid = vec![
		vec![b'9', b'9', b'1', b'1', b'9', b'9'],
		vec![b'9', b'9', b'1', b'1', b'9', b'9'],
		vec![b'0', b'0', b'0', b'0', b'0', b'0'],
		vec![b'0', b'a', b'a', b'b', b'b', b'0'],
		vec![b'9', b'c', b'a', b'b', b'd', b'9'],
		vec![b'9', b'c', b'c', b'd', b'd', b'9'],
		vec![b'9', b'9', b'e', b'e', b'9', b'9'],
		vec![b'9', b'9', b'0', b'0', b'9', b'9'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Spazzatura");
	translations.insert(b'a', "Viola");
	translations.insert(b'b', "Arancione");
	translations.insert(b'c', "Giallo");
	translations.insert(b'd', "Verde");
	translations.insert(b'e', "Azzurro");

	let balls: Vec<Vip> = vec![Vip {
		id: b'1',
		position: Point(2, 0),
		victory_slot: Point(2, 6),
	}];

	let board: Board = board_to_board(board);
	solve(board, 6, 8, balls, translations);
}

pub fn solve_chi_e_tom() {
	let board: Grid = vec![
		vec![b'0', b'1', b'1', b'2', b'2', b'0'],
		vec![b'0', b'1', b'a', b'b', b'2', b'0'],
		vec![b'0', b'3', b'c', b'd', b'4', b'0'],
		vec![b'0', b'3', b'3', b'4', b'4', b'0'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "┌");
	translations.insert(b'2', "┐");
	translations.insert(b'3', "└");
	translations.insert(b'4', "┘");
	translations.insert(b'a', "Blocco");
	translations.insert(b'b', "Blocco");
	translations.insert(b'c', "Blocco");
	translations.insert(b'd', "Blocco");

	let balls: Vec<Vip> = vec![
		Vip {
			id: b'1',
			position: Point(1, 0),
			victory_slot: Point(3, 2),
		},
		Vip {
			id: b'2',
			position: Point(3, 0),
			victory_slot: Point(1, 2),
		},
		Vip {
			id: b'3',
			position: Point(1, 2),
			victory_slot: Point(3, 0),
		},
		Vip {
			id: b'4',
			position: Point(4, 2),
			victory_slot: Point(2, 0),
		},
	];

	let board: Board = board_to_board(board);
	solve(board, 6, 4, balls, translations);
}

pub fn solve_ritiro_bagagli() {
	let board: Grid = vec![
		vec![b'9', b'9', b'1', b'9', b'9'],
		vec![b'0', b'a', b'b', b'b', b'0'],
		vec![b'c', b'a', b'a', b'b', b'd'],
		vec![b'e', b'f', b'f', b'g', b'g'],
		vec![b'9', b'9', b'0', b'9', b'9'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Bagaglio");
	translations.insert(b'a', "Blu");
	translations.insert(b'b', "Giallo");
	translations.insert(b'c', "Verde");
	translations.insert(b'd', "Verde");
	translations.insert(b'e', "Verde");
	translations.insert(b'f', "Rosso");
	translations.insert(b'g', "Rosso");

	let balls: Vec<Vip> = vec![
		Vip {
			id: b'1',
			position: Point(2, 0),
			victory_slot: Point(2, 4),
		},
	];

	let board: Board = board_to_board(board);
	solve(board, 5, 5, balls, translations);
}

pub fn solve_inverti_le_sfere() {
	let board: Grid = vec![
		vec![b'9', b'9', b'9', b'1', b'1', b'9', b'9', b'9'],
		vec![b'9', b'9', b'9', b'1', b'1', b'9', b'9', b'9'],
		vec![b'0', b'0', b'0', b'a', b'0', b'0', b'0', b'0'],
		vec![b'b', b'b', b'a', b'a', b'c', b'c', b'0', b'0'],
		vec![b'b', b'b', b'9', b'd', b'c', b'9', b'e', b'e'],
		vec![b'0', b'0', b'd', b'd', b'f', b'f', b'e', b'e'],
		vec![b'0', b'0', b'0', b'0', b'f', b'0', b'0', b'0'],
		vec![b'9', b'9', b'9', b'2', b'2', b'9', b'9', b'9'],
		vec![b'9', b'9', b'9', b'2', b'2', b'9', b'9', b'9'],
	];

	let mut translations: HashMap<u8, &str> = HashMap::new();
	translations.insert(b'1', "Sfera Rossa");
	translations.insert(b'2', "Sfera Blu");
	translations.insert(b'a', "Verde");
	translations.insert(b'b', "Rosa");
	translations.insert(b'c', "Azzurro");
	translations.insert(b'd', "Verde");
	translations.insert(b'e', "Rosa");
	translations.insert(b'f', "Azzurro");
	let balls: Vec<Vip> = vec![
		Vip {
			id: b'1',
			position: Point(3, 0),
			victory_slot: Point(3, 7),
		},
		Vip {
			id: b'2',
			position: Point(3, 7),
			victory_slot: Point(3, 0),
		},
	];

	let board: Board = board_to_board(board);
	solve(board, 8, 9, balls, translations);
}