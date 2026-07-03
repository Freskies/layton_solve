use crate::pallina_fuori::Board;
use crate::pallina_fuori::node::Node;
use crate::pallina_fuori::visited_record::VisitedRecord;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

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
		let mut came_from: HashMap<Board, VisitedRecord> = HashMap::new();

		let start_node: Node = Node::init(self.board.clone(), 0);
		frontier.push(start_node);

		while !frontier.is_empty() {
			let current_node: Node = frontier.pop().unwrap();

			if current_node.win() {
				self.victory_path = Some(Self::reconstruct_path());
				return;
			}

			if let Some(record) = came_from.get(&current_node.board) {
				if current_node.g_score > record.g_score {
					continue;
				}
			}

			for (possible_node, legal_move) in current_node.possible_nodes() {
				let possible_record: VisitedRecord = VisitedRecord {
					g_score: possible_node.g_score,
					parent: Some(current_node.board.clone()),
					move_made: Some(legal_move),
				};

				match came_from.entry(possible_node.board.clone()) {
					Entry::Occupied(mut occupied) => {
						if possible_node.g_score < occupied.get().g_score {
							occupied.insert(possible_record);
							frontier.push(possible_node.clone())
						}
					}
					Entry::Vacant(vacant) => {
						vacant.insert(possible_record);
						frontier.push(possible_node.clone())
					}
				}
			}
		}
	}

	const fn reconstruct_path() -> Vec<String> {
		vec![]
	}
}
