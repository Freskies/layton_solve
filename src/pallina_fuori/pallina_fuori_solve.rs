use crate::pallina_fuori::Board;
use crate::pallina_fuori::node::Node;
use crate::pallina_fuori::point::Point;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct PallinaFuoriSolve {
	board: Board,
	victory_path: Option<Vec<String>>,
}

impl PallinaFuoriSolve {
	pub fn init(board: Board) -> Self {
		PallinaFuoriSolve {
			board,
			victory_path: None,
		}
	}

	pub fn a_star(&mut self) {
		let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
		let mut came_from: HashMap<Board, Board> = HashMap::new();
		let mut visited: HashSet<Node> = HashSet::new();

		let start_node: Node = Node::init(self.board.clone(), 0);
		frontier.push(start_node);

		while !frontier.is_empty() {
			let current_node: Node = frontier.pop().unwrap();

			if current_node.win() {
				self.victory_path = Some(Self::reconstruct_path());
				return;
			}

			visited.insert(current_node.clone());

			for possible_node in current_node.possible_nodes() {
				if visited.contains(&possible_node) {
					continue;
				}
			}
		}
	}

	fn legal_moves() -> Vec<Point> {
		vec![]
	}

	const fn reconstruct_path() -> Vec<String> {
		vec![]
	}

	// fn do_move() -> Point {}
}
