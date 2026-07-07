use crate::pallina_fuori::ball::Ball;
use crate::pallina_fuori::board::Board;
use crate::pallina_fuori::node::Node;
use crate::pallina_fuori::piece_move::PieceMove;
use crate::pallina_fuori::symmetry_meta::SymmetryMeta;
use crate::pallina_fuori::visited_record::VisitedRecord;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

type CameFrom = HashMap<Board, VisitedRecord>;

pub struct PallinaFuoriSolve {
	board: Board,
	width: usize,
	height: usize,
	pub victory_path: Option<Vec<PieceMove>>,
	balls: Vec<Ball>,
	symmetry_meta: SymmetryMeta,
}

impl PallinaFuoriSolve {
	pub fn init(board: Board, width: usize, height: usize, balls: Vec<Ball>) -> Self {
		let symmetry_meta: SymmetryMeta = SymmetryMeta::build(&board, width);
		PallinaFuoriSolve {
			board,
			width,
			height,
			victory_path: None,
			symmetry_meta,
			balls,
		}
	}

	pub fn a_star(&mut self) {
		let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
		let mut came_from: CameFrom = HashMap::new();
		self.board.normalize(&self.symmetry_meta);

		let start_node: Node = Node::init(self.board.clone(), self.balls.clone(), None, None);
		came_from.insert(
			start_node.board.clone(),
			VisitedRecord {
				g_score: 0,
				parent: None,
				move_made: None,
			},
		);
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

			for (mut possible_node, piece_move) in
				current_node.possible_nodes(self.width, self.height)
			{
				possible_node.board.normalize(&self.symmetry_meta);

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
