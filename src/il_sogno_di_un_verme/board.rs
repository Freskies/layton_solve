use crate::il_sogno_di_un_verme::mela::Mela;
use crate::il_sogno_di_un_verme::point::Point;
use crate::il_sogno_di_un_verme::board_state::BoardState;
use std::collections::{BinaryHeap, HashMap};

const COORDS_ADDENDS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Board {
	board: [[Option<Mela>; 3]; 3],
	empty_pos: Point,
}

impl Board {
	pub fn init() -> Self {
		let mut board = [[None; 3]; 3];
		board[0][0] = Some(Mela::init(9));
		board[0][1] = Some(Mela::init(2));
		board[0][2] = Some(Mela::init(3));
		board[1][0] = Some(Mela::init(4));
		board[1][1] = Some(Mela::init(8));
		board[1][2] = Some(Mela::init(7));
		board[2][0] = Some(Mela::init(6));
		board[2][2] = Some(Mela::init(1));
		Self {
			board,
			empty_pos: Point(2, 1),
		}
	}

	pub fn solve(&self) -> Vec<String> {
		let mut heap: BinaryHeap<BoardState> = BinaryHeap::new();
		heap.push(BoardState {
			f_score: self.manhattan(),
			g_score: 0,
			board: *self,
		});

		// stato (mossa, stato_padre)
		let mut states: HashMap<Board, Option<(u8, Board)>> = HashMap::new();
		states.insert(*self, None);

		while let Some(BoardState { g_score, board: cur_board, .. }) = heap.pop()
		{
			if cur_board.manhattan() == 0 {
				return cur_board.traverse_solve(states);
			}

			for legal_move in cur_board.legal_moves() {
				let mut next_board = cur_board;
				let move_int = next_board.do_move(&legal_move);

				if !states.contains_key(&next_board) {
					let next_g = g_score + 1;
					let next_f = next_g + next_board.manhattan();

					heap.push(BoardState {
						f_score: next_f,
						g_score: next_g,
						board: next_board,
					});

					states.insert(next_board, Some((move_int, cur_board)));
				}
			}
		}

		vec![]
	}

	fn traverse_solve(&self, states: HashMap<Board, Option<(u8, Board)>>) -> Vec<String> {
		let mut move_list: Vec<String> = vec![];
		let mut state: Board = *self;

		while let Some(Some((move_int, parent_state))) = states.get(&state) {
			move_list.push(format!("{move_int}"));
			state = *parent_state;
		}

		move_list.reverse();
		move_list
	}

	fn legal_moves(&self) -> Vec<Point> {
		let mut legal_moves = vec![];

		for (x, y) in COORDS_ADDENDS {
			let board_x: i8 = (self.empty_pos.0 as i8) + x;
			let board_y: i8 = (self.empty_pos.1 as i8) + y;
			if (0..3i8).contains(&board_x) && (0..3i8).contains(&board_y) {
				legal_moves.push(Point(board_x as usize, board_y as usize))
			}
		}

		legal_moves
	}

	fn do_move(&mut self, point: &Point) -> u8 {
		let puzzle_piece: Option<Mela> = self.board[point.0][point.1];
		self.board[self.empty_pos.0][self.empty_pos.1] = puzzle_piece;
		self.board[point.0][point.1] = None;
		self.empty_pos = point.clone();
		(point.0 * 3 + point.1 + 1) as u8
	}

	fn manhattan(&self) -> usize {
		let mut distance = 0;

		for row in 0..3 {
			for col in 0..3 {
				if let Some(puzzle_piece) = self.board[row][col] {
					let target_index = puzzle_piece.pos_to_win - 1;
					let target_row = target_index / 3;
					let target_col = target_index % 3;
					distance += row.abs_diff(target_row) + col.abs_diff(target_col);
				}
			}
		}

		distance
	}
}
