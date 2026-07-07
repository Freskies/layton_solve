use crate::pallina_fuori::ball::Ball;
use crate::pallina_fuori::board::Board;
use crate::pallina_fuori::pallina_fuori_solve::PallinaFuoriSolve;
use crate::pallina_fuori::piece_move::PieceMove;
use crate::pallina_fuori::point::Point;
use std::collections::HashMap;

pub mod ball;
pub mod board;
pub mod node;
pub mod pallina_fuori_solve;
pub mod piece_move;
pub mod point;
pub mod symmetry_meta;
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
	balls: Vec<Ball>,
	translations: HashMap<u8, &str>,
) {
	let mut pallina_solve: PallinaFuoriSolve = PallinaFuoriSolve::init(board, width, height, balls);
	pallina_solve.a_star();
	if let Some(path) = &pallina_solve.victory_path {
		print_solution(path, translations);
	} else {
		println!("L'enigma è impossibile o io sono un cane a programmare.");
	}
}

pub fn solve1() {
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

	let balls: Vec<Ball> = vec![Ball {
		id: b'1',
		victory_slot: Point(2, 5),
		position: Point(2, 0),
	}];

	let board: Board = board_to_board(board);
	solve(board, 5, 6, balls, translations);
}

pub fn solve2() {
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

	let balls: Vec<Ball> = vec![Ball {
		id: b'1',
		victory_slot: Point(3, 5),
		position: Point(2, 0),
	}];

	let board: Board = board_to_board(board);
	solve(board, 6, 6, balls, translations);
}

pub fn solve4() {
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

	let balls: Vec<Ball> = vec![Ball {
		id: b'1',
		victory_slot: Point(3, 5),
		position: Point(0, 0),
	}];

	let board: Board = board_to_board(board);
	solve(board, 4, 6, balls, translations);
}

pub fn solve4e() {
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

	let balls: Vec<Ball> = vec![
		Ball { id: b'1', victory_slot: Point(1, 5), position: Point(4, 0) },
		Ball { id: b'2', victory_slot: Point(5, 4), position: Point(0, 1) },
		Ball { id: b'3', victory_slot: Point(0, 1), position: Point(5, 4) },
		Ball { id: b'4', victory_slot: Point(4, 0), position: Point(1, 5) },
	];

	let board: Board = board_to_board(board);
	solve(board, 6, 6, balls, translations);
}

fn board_to_board(grid: Grid) -> Board {
	let mut data = [0; 64];
	let mut len = 0;

	for row in grid {
		for col in row {
			if len < 64 {
				data[len] = col;
				len += 1;
			}
		}
	}

	Board { data, len }
}

fn print_solution(solution: &Vec<PieceMove>, translations: HashMap<u8, &str>) {
	for piece_move in solution {
		println!(
			"{} {}",
			translations.get(&piece_move.piece).unwrap().to_string(),
			piece_move.legal_move
		)
	}
}
