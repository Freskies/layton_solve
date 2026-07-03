use crate::pallina_fuori::point::Point;
use std::collections::{BTreeMap, HashMap};

pub mod pallina_fuori_solve;
pub mod point;
pub mod legal_move;
pub mod node;
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

fn solve(board: Board) {}

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
	println!("{:?}", board);
	solve(board);
}

pub fn solve2() {
	//solve()
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

