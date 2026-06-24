use crate::il_sogno_di_un_verme::mela::Mela;
use crate::il_sogno_di_un_verme::point::{Point, Points};
use queues::{IsQueue, Queue, queue};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Board {
	board: [[Option<Mela>; 3]; 3],
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
		Self { board }
	}

	pub fn solve(&self) -> Vec<String> {
		let mut queue: Queue<Board> = queue![];
		let _ = queue.add(self.clone());
		let start_id = self.serialize();

		// stato (mossa, stato_padre)
		let mut states: HashMap<String, (String, String)> = HashMap::new();
		states.insert(start_id, ("".to_string(), "".to_string()));

		while queue.size() != 0 {
			let cur_state = queue.remove().unwrap();
			let cur_id = cur_state.serialize();

			if cur_state.win() {
				return cur_state.traverse_solve(states, cur_id);
			}

			let legal_moves = cur_state.legal_moves();
			for legal_move in legal_moves {
				let mut next_state = cur_state.clone();
				let move_string = next_state.do_move(&legal_move);
				let next_id = next_state.serialize();

				if !states.contains_key(&next_id) {
					let _ = queue.add(next_state);
					states.insert(next_id, (move_string, cur_id.clone()));
				}
			}
		}

		vec![]
	}

	fn traverse_solve(
		&self,
		states: HashMap<String, (String, String)>,
		cur_state: String,
	) -> Vec<String> {
		let mut move_list: Vec<String> = vec![];
		let mut cur_id: String = cur_state;

		while states.contains_key(&cur_id) {
			let (sigle_move, father_id) = states.get(&cur_id).unwrap();
			move_list.push(sigle_move.to_string());
			cur_id = father_id.to_string();
		}

		move_list.reverse();
		move_list
	}

	fn find_empty(&self) -> Point {
		for (i_row, row) in self.board.iter().enumerate() {
			for (i_cell, cell) in row.iter().enumerate() {
				if cell.is_none() {
					return Point(i_row, i_cell);
				}
			}
		}
		Point(255, 255)
	}

	fn legal_moves(&self) -> Vec<Points> {
		let mut legal_moves = vec![];
		let empty: Point = self.find_empty();

		for i in (0..360).step_by(90) {
			let rad = (i as f64).to_radians();
			let x = rad.cos().round() as i8;
			let y = rad.sin().round() as i8;
			let board_x: i8 = (empty.0 as i8) + x;
			let board_y: i8 = (empty.1 as i8) + y;
			if (0..3i8).contains(&board_x) && (0..3i8).contains(&board_y) {
				legal_moves.push(Points(Point(board_x as usize, board_y as usize), empty))
			}
		}

		legal_moves
	}

	fn do_move(&mut self, points: &Points) -> String {
		let m1: Option<Mela> = self.board[points.0.0][points.0.1];
		let m2: Option<Mela> = self.board[points.1.0][points.1.1];
		self.board[points.0.0][points.0.1] = m2;
		self.board[points.1.0][points.1.1] = m1;
		format!("{}", points.0.0 * 3 + points.0.1 + 1)
	}

	fn win(&self) -> bool {
		let mut count = 0;

		for row in &self.board {
			for cell in row {
				count += 1;
				if let Some(mela) = cell {
					if mela.pos_to_win != count {
						return false;
					}
				}
			}
		}

		true
	}

	fn serialize(&self) -> String {
		let mut serialized: String = "".to_string();
		for row in &self.board {
			for cell in row {
				if let Some(mela) = cell {
					serialized = format!("{}{}", serialized, mela.serialize())
				} else {
					serialized = format!("{}{}", serialized, "0")
				}
			}
		}
		serialized
	}
}
