use crate::pallina_fuori::pallina_fuori_solve::PallinaFuoriSolve;
use crate::pallina_fuori::piece_move::PieceMove;
use crate::pallina_fuori::point::Point;
use std::collections::{BTreeMap, HashMap};

pub mod node;
pub mod pallina_fuori_solve;
pub mod piece_move;
pub mod point;
pub mod visited_record;
/*
0 -> empty space
1 -> ball
2 -> victory slot
9 -> wall
letters -> pieces
*/

pub type Board = BTreeMap<Point, char>;
pub type Grid = Vec<Vec<char>>;

fn solve(board: Board, translations: HashMap<char, &str>) {
	let victory_slot: Point = get_victory_slot(&board);
	let ball_slot: Point = get_ball_slot(&board);
	let mut pallina_solve: PallinaFuoriSolve = PallinaFuoriSolve::init(board, victory_slot);
	pallina_solve.a_star(ball_slot);
	print_solution(&pallina_solve.victory_path.unwrap(), translations);
}

pub fn solve1() {
	let board: Grid = vec![
		vec!['9', '9', '1', '9', '9'],
		vec!['a', 'b', 'b', '0', '0'],
		vec!['a', 'b', 'b', 'c', 'c'],
		vec!['d', 'd', 'e', 'e', 'f'],
		vec!['0', '0', 'e', 'e', 'g'],
		vec!['9', '9', '2', '9', '9'],
	];

	let mut translations: HashMap<char, &str> = HashMap::new();
	translations.insert('a', "Verde");
	translations.insert('b', "Giallo");
	translations.insert('c', "Blu");
	translations.insert('d', "Blu");
	translations.insert('e', "Giallo");
	translations.insert('f', "Viola");
	translations.insert('g', "Viola");

	let board: Board = board_to_hashmap(board);
	solve(board, translations);
}

pub fn solve2() {
	//solve()
}

fn get_slot(board: &Board, c: char) -> Option<Point> {
	for (coords, letter) in board.iter() {
		if c.to_string() == letter.to_string() {
			return Some(coords.clone());
		}
	}
	None
}

fn get_ball_slot(board: &Board) -> Point {
	get_slot(board, '1').unwrap()
}

fn get_victory_slot(board: &Board) -> Point {
	get_slot(board, '2').unwrap()
}

fn board_to_hashmap(board: Grid) -> Board {
	let mut new_board: Board = BTreeMap::new();

	for (r, row) in board.iter().enumerate() {
		for (c, col) in row.iter().enumerate() {
			new_board.insert(Point(c, r), *col);
		}
	}

	new_board
}

fn print_solution(solution: &Vec<PieceMove>, translations: HashMap<char, &str>) {
	for piece_move in solution {
		println!(
			"{} {}",
			translations.get(&piece_move.piece).unwrap().to_string(),
			piece_move.legal_move
		)
	}
}
