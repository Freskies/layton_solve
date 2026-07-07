use crate::klotski::Board;
use crate::klotski::vip::Vip;
use crate::klotski::piece_move::{DIRECTIONS, PieceMove};
use crate::klotski::point::Point;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Node {
	pub board: Board,
	pub vips: Vec<Vip>,
	pub f_score: usize,
	pub g_score: usize,
	pub last_piece: Option<u8>,
}

impl Node {
	pub fn init(
		board: Board,
		vips: Vec<Vip>,
		g_score: Option<usize>,
		last_piece: Option<u8>,
	) -> Self {
		let mut node = Node {
			board,
			vips,
			g_score: g_score.unwrap_or(0),
			f_score: 0,
			last_piece,
		};
		node.f_score = node.g_score + node.manhattan();
		node
	}

	fn manhattan(&self) -> usize {
		let mut distance: usize = 0;
		for ball in &self.vips {
			distance += ball.position.0.abs_diff(ball.victory_slot.0)
				+ ball.position.1.abs_diff(ball.victory_slot.1)
		}
		distance.div_ceil(3)
	}

	pub fn win(&self) -> bool {
		self.g_score == self.f_score
	}

	pub fn possible_nodes(&self, width: usize, height: usize) -> Vec<(Node, PieceMove)> {
		let mut possible_nodes: Vec<(Node, PieceMove)> = vec![];
		let mut movable_points = [[Point(0, 0); 4]; 128];
		let mut movable_counts = [0_usize; 128];

		for (i, &c) in self.board.iter().enumerate() {
			if c == b'9' || c == b'0' {
				continue;
			}
			let idx = c as usize;
			let count = movable_counts[idx];
			movable_points[idx][count] = Point::from_index(i, width);
			movable_counts[idx] += 1;
		}

		for direction in DIRECTIONS {
			for letter_index in 0..128 {
				let mut do_move = true;
				let count = movable_counts[letter_index];
				if count == 0 {
					continue;
				}
				let letter = letter_index as u8;
				let points = &movable_points[letter_index][..count];

				for point in points {
					if let Some(new_point) = point.add(&direction, width, height) {
						let index = new_point.to_index(width);
						if index < self.board.len() {
							let c = self.board[index];
							if !(c == letter || c == b'0') {
								do_move = false;
							}
						}
					} else {
						do_move = false;
					}
					if !do_move {
						break;
					}
				}

				if do_move {
					let mut new_board: Board = self.board.clone();
					for point in points {
						new_board[point.to_index(width)] = b'0';
					}
					for point in points {
						let target = point.add(&direction, width, height).unwrap();
						new_board[target.to_index(width)] = letter;
					}
					let piece_move: PieceMove = PieceMove {
						piece: letter.clone(),
						legal_move: direction.clone(),
					};

					let mut new_balls_positions: Vec<Vip> = vec![];
					for vip in &self.vips {
						new_balls_positions.push(vip.duplicate(if vip.id == letter {
							vip.position.add(&direction, width, height).unwrap()
						} else {
							vip.position.clone()
						}));
					}

					// let new_g_score = if Some(letter) == self.last_piece {
					// 	self.g_score
					// } else {
					// 	self.g_score + 1
					// };
					let new_g_score = self.g_score + 1;
					let new_node = Node::init(
						new_board,
						new_balls_positions,
						Some(new_g_score),
						Some(letter),
					);
					possible_nodes.push((new_node, piece_move))
				}
			}
		}

		possible_nodes
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		other
			.f_score
			.cmp(&self.f_score)
			.then_with(|| self.g_score.cmp(&other.g_score))
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
