use crate::pallina_fuori::Board;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Node {
	pub board: Board,
	pub f_score: usize,
	pub g_score: usize,
}

impl Node {
	pub fn init(board: Board, g_score: usize) -> Self {
		let mut node = Node {
			board,
			g_score,
			f_score: g_score,
		};
		node.f_score += node.manhattan();
		node
	}

	fn manhattan(&self) -> usize {
		0
	}

	pub fn win(&self) -> bool {
		self.g_score == self.f_score
	}

	pub fn possible_nodes(&self) -> Vec<Node> {
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
