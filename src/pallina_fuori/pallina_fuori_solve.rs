use crate::pallina_fuori::Board;
use crate::pallina_fuori::node::Node;
use crate::pallina_fuori::visited_record::VisitedRecord;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use crate::pallina_fuori::piece_move::PieceMove;
use crate::pallina_fuori::point::Point;

type CameFrom = HashMap<Board, VisitedRecord>;

pub struct PallinaFuoriSolve {
	board: Board,
	victory_slot: Point,
	pub victory_path: Option<Vec<PieceMove>>,
}

impl PallinaFuoriSolve {
	pub fn init(board: Board, victory_slot: Point) -> Self {
		PallinaFuoriSolve {
			board,
			victory_path: None,
			victory_slot
		}
	}

	pub fn a_star(&mut self, ball_slot: Point) {
		let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
		let mut came_from: CameFrom = HashMap::new();

		let start_node: Node = Node::init(self.board.clone(), ball_slot, None, &self.victory_slot);
		frontier.push(start_node);

		while !frontier.is_empty() {
			let current_node: Node = frontier.pop().unwrap();

			if current_node.win() {
				self.victory_path = Some(Self::reconstruct_path(&current_node, &came_from));
				return;
			}

			if let Some(record) = came_from.get(&current_node.board) {
				if current_node.g_score > record.g_score {
					continue;
				}
			}

			for (possible_node, piece_move) in current_node.possible_nodes() {
				let possible_record: VisitedRecord = VisitedRecord {
					g_score: possible_node.g_score,
					parent: Some(current_node.board.clone()),
					move_made: Some(piece_move),
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

	fn manhattan(&self, node: &Node) {

	}

	pub fn reconstruct_path(last_node: &Node, came_from: &CameFrom) -> Vec<PieceMove> {
		let mut victory_path: Vec<PieceMove> = vec![];
		let mut current_board: &Board = &last_node.board;

		while let Some(record) = came_from.get(current_board) {
			if let (Some(parent), Some(piece_move)) = (&record.parent, record.move_made) {
				victory_path.push(piece_move);
				current_board = parent;
			} else {
				break;
			}
		}

		victory_path.reverse();
		victory_path
	}
}
