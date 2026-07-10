use crate::klotski::Board;
use crate::klotski::vip::Vip;
use crate::klotski::piece_move::{DIRECTIONS, LegalMove, PieceMove};
use crate::klotski::point::Point;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Node {
	pub board: Board,
	pub vips: Vec<Vip>,
	pub f_score: usize,
	pub g_score: usize,
}

impl Node {
	pub fn init(board: Board, vips: Vec<Vip>, g_score: Option<usize>) -> Self {
		let mut node = Node {
			board,
			vips,
			g_score: g_score.unwrap_or(0),
			f_score: 0,
		};
		node.f_score = node.g_score + node.manhattan();
		node
	}

	fn manhattan(&self) -> usize {
		let mut distance: usize = 0;
		for vip in &self.vips {
			if vip.position.0 != vip.victory_slot.0 || vip.position.1 != vip.victory_slot.1 {
				distance += 1;
			}
		}
		distance
	}

	pub fn win(&self) -> bool {
		self.g_score == self.f_score
	}

	fn letter_points(board: &Board, letter: u8, width: usize) -> Vec<Point> {
		let mut points: Vec<Point> = vec![];
		for (i, &c) in board.iter().enumerate() {
			if c == letter {
				points.push(Point::from_index(i, width));
			}
		}
		points
	}

	fn try_slide(
		board: &Board,
		points: &[Point],
		letter: u8,
		direction: &LegalMove,
		width: usize,
		height: usize,
	) -> Option<Board> {
		for point in points {
			if let Some(new_point) = point.add(direction, width, height) {
				let index = new_point.to_index(width);
				if index < board.len() {
					let c = board[index];
					if !(c == letter || c == b'0') {
						return None;
					}
				}
			} else {
				return None;
			}
		}

		let mut new_board: Board = board.clone();
		for point in points {
			new_board[point.to_index(width)] = b'0';
		}
		for point in points {
			let target = point.add(direction, width, height).unwrap();
			new_board[target.to_index(width)] = letter;
		}
		Some(new_board)
	}

	pub fn possible_nodes(&self, width: usize, height: usize) -> Vec<(Node, PieceMove)> {
		let mut possible_nodes: Vec<(Node, PieceMove)> = vec![];

		let mut seen = [false; 128];
		for &c in self.board.iter() {
			if c == b'9' || c == b'0' {
				continue;
			}
			seen[c as usize] = true;
		}

		for letter_index in 0..128 {
			if !seen[letter_index] {
				continue;
			}
			let letter = letter_index as u8;

			// BFS interna: trova tutte le posizioni finali raggiungibili
			// facendo scivolare liberamente il pezzo, tenendo fermi gli altri.
			let mut visited: HashMap<Board, (Vec<LegalMove>, isize, isize)> = HashMap::new();
			visited.insert(self.board.clone(), (vec![], 0, 0));
			let mut queue: VecDeque<Board> = VecDeque::new();
			queue.push_back(self.board.clone());

			while let Some(current) = queue.pop_front() {
				let (path, dx, dy) = visited.get(&current).unwrap().clone();
				let points = Self::letter_points(&current, letter, width);

				for direction in DIRECTIONS {
					if let Some(new_board) =
						Self::try_slide(&current, &points, letter, &direction, width, height)
					{
						if !visited.contains_key(&new_board) {
							let mut new_path = path.clone();
							new_path.push(direction);
							let (ddx, ddy) = match direction {
								LegalMove::UP => (0, -1),
								LegalMove::RIGHT => (1, 0),
								LegalMove::DOWN => (0, 1),
								LegalMove::LEFT => (-1, 0),
							};
							visited
								.insert(new_board.clone(), (new_path, dx + ddx, dy + ddy));
							queue.push_back(new_board);
						}
					}
				}
			}

			for (dest_board, (path, dx, dy)) in visited {
				if path.is_empty() {
					continue;
				}

				let mut new_vips: Vec<Vip> = vec![];
				for vip in &self.vips {
					let new_position = if vip.id == letter {
						Point(
							(vip.position.0 as isize + dx) as usize,
							(vip.position.1 as isize + dy) as usize,
						)
					} else {
						vip.position.clone()
					};
					new_vips.push(vip.duplicate(new_position));
				}

				let piece_move = PieceMove {
					piece: letter,
					moves: path,
				};

				let new_node = Node::init(dest_board, new_vips, Some(self.g_score + 1));
				possible_nodes.push((new_node, piece_move));
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
