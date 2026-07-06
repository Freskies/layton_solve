use crate::pallina_fuori::Board;
use std::cmp::Ordering;
use crate::pallina_fuori::piece_move::{PieceMove};
use crate::pallina_fuori::point::Point;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Node {
	pub board: Board,
	pub f_score: usize,
	pub g_score: usize,
	pub coords_ball: Point
}

impl Node {
	pub fn init(board: Board, coords_ball: Point, g_score: Option<usize>, target_point: &Point) -> Self {
		let mut node = Node {
			board,
			g_score: g_score.unwrap_or(0),
			f_score: 0,
			coords_ball
		};
		node.f_score = node.g_score + node.manhattan(target_point);
		node
	}

	fn manhattan(&self, target_point: &Point) -> usize {
		0
	}

	pub fn win(&self) -> bool {
		self.g_score == self.f_score
	}

	pub fn possible_nodes(&self) -> Vec<(Node, PieceMove)> {
		vec![]
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
