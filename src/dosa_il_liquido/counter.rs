use crate::dosa_il_liquido::recipient::Recipient;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Counter {
	pub recipients: Vec<Recipient>,
	pub last_move: (usize, usize),
}

impl Counter {
	pub fn init(recipients: Vec<Recipient>) -> Self {
		Self {
			recipients,
			last_move: (255, 255),
		}
	}

	pub fn win(&self) -> bool {
		let r1 = &self.recipients[0];
		let win_condition = r1.space / 2;
		if r1.drink != win_condition {
			return false;
		}
		let r2 = &self.recipients[1];
		if r2.drink != win_condition {
			return false;
		}
		true
	}

	pub fn legal_moves(&self) -> Vec<(usize, usize)> {
		let mut legal_moves: Vec<(usize, usize)> = vec![];
		for (index1, recipient1) in self.recipients.iter().enumerate() {
			if !recipient1.can_pour() {
				continue;
			}
			for (index2, recipient2) in self.recipients.iter().enumerate() {
				if index1 == index2 {
					continue;
				}
				if !recipient2.can_hold() {
					continue;
				}
				if index1 == self.last_move.1 && index2 == self.last_move.0 {
					continue;
				}
				legal_moves.push((index1, index2))
			}
		}
		legal_moves
	}

	pub fn solve(&mut self) -> Vec<String> {
		let mut queue: VecDeque<Counter> = VecDeque::new();
		queue.push_back(self.clone());

		let mut states: HashMap<Counter, Option<(Counter, String)>> = HashMap::new();
		states.insert(self.clone(), None);

		while !queue.is_empty() {
			let cur_state: Counter = queue.pop_front().unwrap();

			if cur_state.win() {
				return cur_state.traverse_moves_dict(states);
			}

			let legal_moves = cur_state.legal_moves();
			for legal_move in legal_moves {
				let mut new_state: Counter = cur_state.clone();
				new_state.pour(legal_move);

				if !states.contains_key(&new_state) {
					states.insert(
						new_state.clone(),
						Some((cur_state.clone(), new_state.serialize_move(legal_move))),
					);
					queue.push_back(new_state);
				}
			}
		}

		vec![]
	}

	fn traverse_moves_dict(
		&self,
		states: HashMap<Counter, Option<(Counter, String)>>,
	) -> Vec<String> {
		let mut move_list: Vec<String> = vec![];
		let mut cur_counter: &Counter = self;

		while let Some(Some((parent, move_str))) = states.get(cur_counter) {
			move_list.push(format!("{move_str}"));
			cur_counter = parent;
		}

		move_list.reverse();
		move_list
	}

	fn pour(&mut self, legal_move: (usize, usize)) -> u8 {
		self.last_move = legal_move;
		let (from, to) = legal_move;
		if from < to {
			let (left, right) = self.recipients.split_at_mut(to);
			left[from].pour(&mut right[0])
		} else {
			let (left, right) = self.recipients.split_at_mut(from);
			right[0].pour(&mut left[to])
		}
	}

	fn serialize_move(&self, legal_move: (usize, usize)) -> String {
		format!(
			"{} → {}",
			self.recipients[legal_move.0].get_name(),
			self.recipients[legal_move.1].get_name()
		)
	}
}
