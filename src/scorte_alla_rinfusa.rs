use crate::scorte_alla_rinfusa::Tin::{CHERRY, FISH, MEAT, YELLOW};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const MAX_HEIGHT: usize = 6;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum Tin {
	FISH,
	MEAT,
	CHERRY,
	YELLOW,
}

#[derive(Clone)]
pub struct PlayedMove {
	column_from: u8,
	tins: u8,
	column_to: u8,
}

pub struct FromBoard {
	board: Board,
	played_move: PlayedMove,
}

pub type Board = Vec<Vec<Tin>>;

#[derive(Eq, PartialEq)]
pub struct Node {
	board: Board,
	g_score: usize,
	f_score: usize,
}

impl PartialOrd<Self> for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		other
			.f_score
			.cmp(&self.f_score)
			.then_with(|| self.g_score.cmp(&other.g_score))
			.then_with(|| self.board.cmp(&other.board))
	}
}

pub fn solve_1() {
	let board: Board = vec![
		vec![CHERRY, FISH, YELLOW],
		vec![FISH, YELLOW, CHERRY],
		vec![YELLOW, CHERRY, FISH],
	];
	solve(board);
}

pub fn solve_2() {
	let board: Board = vec![
		vec![CHERRY, MEAT, YELLOW, FISH],
		vec![CHERRY, FISH, MEAT, YELLOW],
		vec![YELLOW, FISH, MEAT, CHERRY],
		vec![MEAT, CHERRY, YELLOW, FISH],
	];
	solve(board);
}

fn solve(board: Board) {
	let played_moves: Vec<PlayedMove> = a_star(board);
	for (i, played_move) in played_moves.iter().enumerate() {
		println!(
			"|{}|{}|{}|{}|",
			i + 1,
			played_move.tins,
			played_move.column_from + 1,
			played_move.column_to + 1
		);
	}
}

fn a_star(root: Board) -> Vec<PlayedMove> {
	let mut visited: HashMap<Board, Option<FromBoard>> = HashMap::new(); // K: state, V: father
	let mut best_cost: HashMap<Board, usize> = HashMap::new();
	let mut priority_queue: BinaryHeap<Node> = BinaryHeap::new();

	visited.insert(root.clone(), None);
	best_cost.insert(root.clone(), 0);
	priority_queue.push(Node {
		board: root.clone(),
		g_score: 0,
		f_score: h_score(&root),
	});

	while let Some(current) = priority_queue.pop() {
		if best_cost.get(&current.board) != Some(&current.g_score) {
			continue;
		}

		if win(&current.board) {
			return traverse_solution(current.board, visited);
		}

		for (board, played_move) in get_possible_moves(&current.board) {
			let g_score = current.g_score + 1;

			if let Some(&previous_cost) = best_cost.get(&board) {
				if g_score >= previous_cost {
					continue;
				}
			}

			best_cost.insert(board.clone(), g_score);
			priority_queue.push(Node {
				board: board.clone(),
				g_score,
				f_score: g_score + h_score(&board),
			});
			visited.insert(
				board,
				Some(FromBoard {
					board: current.board.clone(),
					played_move,
				}),
			);
		}
	}

	panic!("No solutions found.")
}

fn win(board: &Board) -> bool {
	for column in board {
		let mut current_tin: Option<Tin> = None;
		for tin in column {
			match current_tin {
				None => current_tin = Some(tin.clone()),
				Some(current) => {
					if current != *tin {
						return false;
					}
				}
			}
		}
	}
	true
}

fn traverse_solution(
	win_board: Board,
	visited: HashMap<Board, Option<FromBoard>>,
) -> Vec<PlayedMove> {
	let mut solution: Vec<PlayedMove> = vec![];
	let mut current: Board = win_board;

	while let Some(father) = visited.get(&current) {
		match father {
			None => break,
			Some(from_board) => {
				solution.push(from_board.played_move.clone());
				current = from_board.board.clone();
			}
		}
	}

	solution.reverse();
	solution
}

fn get_possible_moves(board: &Board) -> Vec<(Board, PlayedMove)> {
	let mut move_list: Vec<(Board, PlayedMove)> = vec![];

	for (i_from, column_from) in board.iter().enumerate() {
		for tins in 1..=column_from.len() {
			for (i_to, column_to) in board.iter().enumerate() {
				if i_from == i_to {
					continue;
				}
				if column_to.len() + tins > MAX_HEIGHT {
					continue;
				}

				let played_move: PlayedMove = PlayedMove {
					column_from: i_from as u8,
					tins: tins as u8,
					column_to: i_to as u8,
				};
				let new_board: Board = do_move(board, &played_move);
				move_list.push((new_board, played_move));
			}
		}
	}

	move_list
}

fn do_move(board: &Board, played_move: &PlayedMove) -> Board {
	let mut new_board: Board = board.clone();

	let from: usize = played_move.column_from as usize;
	let to: usize = played_move.column_to as usize;
	let tins_count: usize = played_move.tins as usize;

	let split_index: usize = new_board[from].len() - tins_count;
	let tins: Vec<Tin> = new_board[from].split_off(split_index);
	new_board[to].extend(tins);

	new_board
}

fn h_score(board: &Board) -> usize {
	board
		.iter()
		.map(|column| column.windows(2).filter(|pair| pair[0] != pair[1]).count())
		.sum()
}
