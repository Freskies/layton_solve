use std::collections::{HashMap, VecDeque};

type Plate = Vec<u8>;
type Plates = [Plate; 3];
type PossibleMove = (u8, Plates);
type PossibleMoves = Vec<PossibleMove>;

const TRANSLATIONS: [&str; 3] = ["Sinistra", "Centro", "Destra"];

pub struct VisitedRecord {
	pub parent: Option<Plates>,
	pub move_made: Option<u8>,
}

pub struct FrittelleImpilateSolver {
	initial_state: Plates,
	pub victory_path: Option<Vec<String>>,
}

impl FrittelleImpilateSolver {
	pub fn init(num_frittelle: u8) -> Self {
		Self {
			initial_state: [(1..=num_frittelle).collect(), vec![], vec![]],
			victory_path: None,
		}
	}

	pub fn bfs(&mut self) {
		let mut frontier: VecDeque<Plates> = VecDeque::new();
		let mut came_from: HashMap<Plates, VisitedRecord> = HashMap::new();
		frontier.push_back(self.initial_state.clone());
		came_from.insert(
			self.initial_state.clone(),
			VisitedRecord {
				parent: None,
				move_made: None,
			},
		);

		while !frontier.is_empty() {
			let current_plates: Plates = frontier.pop_front().unwrap();

			if Self::win(&current_plates) {
				self.victory_path = Some(Self::reconstruct_path(current_plates, came_from));
				return;
			}

			for possible_move in Self::possible_moves(&current_plates) {
				let possible_record = VisitedRecord {
					parent: Some(current_plates.clone()),
					move_made: Some(possible_move.0),
				};

				let possible_plates: Plates = possible_move.1;

				if !came_from.contains_key(&possible_plates) {
					came_from.insert(possible_plates.clone(), possible_record);
					frontier.push_back(possible_plates)
				}
			}
		}
	}

	fn win(plates: &Plates) -> bool {
		plates[0].is_empty() && plates[1].is_empty()
	}

	fn possible_moves(plates: &Plates) -> PossibleMoves {
		let mut moves: PossibleMoves = vec![];

		for i in 0..3 {
			if let Some(start_frittella) = plates[i].last() {
				for k in 0..3 {
					if i == k {
						continue;
					}
					if start_frittella > plates[k].last().unwrap_or(&0) {
						let mut plates: Plates = plates.clone();
						plates[i].pop();
						plates[k].push(*start_frittella);
						let move_int = ((i << 2) + k) as u8;
						moves.push((move_int, plates))
					}
				}
			}
		}

		moves
	}

	fn reconstruct_path(
		cur_plates: Plates,
		came_from: HashMap<Plates, VisitedRecord>,
	) -> Vec<String> {
		let mut path: Vec<String> = vec![];
		let mut cur_plates = cur_plates;

		while let Some(visited_record) = came_from.get(&cur_plates) {
			if let (Some(parent), Some(move_made)) =
				(&visited_record.parent, visited_record.move_made)
			{
				path.push(Self::decipher_move(move_made));
				cur_plates = parent.clone();
			} else {
				break;
			}
		}

		path.reverse();
		path
	}

	fn decipher_move(m: u8) -> String {
		let from: u8 = m >> 2;
		let to: u8 = m & 0b11;
		format!(
			"{} {}",
			TRANSLATIONS[from as usize], TRANSLATIONS[to as usize]
		)
	}
}
