use crate::salto_del_cavallo::cell::Cell::{self, CHECKED, UNCHECKED};
use crate::salto_del_cavallo::horse_move::{HorseMove, HORSE_MOVES};
use crate::salto_del_cavallo::point::Point;

pub struct Board {
	board: Vec<Vec<Cell>>,
	x_size: usize,
	y_size: usize,
	num_checked: usize,
	pub solution: Vec<String>,
}

impl Board {
	pub fn init(x_size: usize, y_size: usize) -> Self {
		let mut b = Self {
			board: vec![vec![UNCHECKED; x_size]; y_size],
			x_size,
			y_size,
			num_checked: 0,
			solution: vec![],
		};
		let horse_start_cell = &HorseMove::init();
		b.move_in(&horse_start_cell.point);
		b.solve(horse_start_cell);
		b
	}

	pub fn solve(&mut self, horse_move: &HorseMove) -> bool {
		if self.win() {
			return true;
		}

		let mut legal_moves = self.legal_moves(&horse_move.point);
		legal_moves.sort_by_key(|m| self.legal_moves(&m.point).len());

		for legal_move in legal_moves {
			self.move_in(&legal_move.point);
			if self.solve(&legal_move) {
				self.solution.push(legal_move.name.to_string());
				return true;
			}
			self.move_out(&legal_move.point);
		}

		false
	}

	fn move_in(&mut self, cell: &Point) {
		self.board[cell.y as usize][cell.x as usize] = CHECKED;
		self.num_checked += 1;
	}

	fn move_out(&mut self, cell: &Point) {
		self.board[cell.y as usize][cell.x as usize] = UNCHECKED;
		self.num_checked -= 1;
	}

	fn legal_moves(&self, start: &Point) -> Vec<HorseMove> {
		let mut moves = vec![];
		for horse_move in HORSE_MOVES {
			let horse_move: HorseMove = start.add(&horse_move);
			if !self.point_in_boundaries(&horse_move.point) {
				continue;
			}
			if self.point_is_checked(&horse_move.point) {
				continue;
			}
			moves.push(horse_move);
		}
		moves
	}

	pub fn win(&self) -> bool {
		self.num_checked == self.x_size * self.y_size
	}

	fn point_in_boundaries(&self, p: &Point) -> bool {
		(0..self.x_size as i8).contains(&p.x) && (0..self.y_size as i8).contains(&p.y)
	}

	fn point_is_checked(&self, p: &Point) -> bool {
		match self.board[p.y as usize][p.x as usize] {
			CHECKED => true,
			UNCHECKED => false,
		}
	}

	pub fn print(&self) {
		for row in &self.board {
			for cell in row {
				print!("{}", cell.to_string())
			}
			println!();
		}
		println!();
	}
}
