use crate::lattine_e_barattoli::Cell::{BLOCKED, CAN, EMPTY, JAR};
use std::cmp::PartialEq;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

const COUNTER_ITER: RangeInclusive<i32> = 0..=28;
const GRAB_RELEASE: u32 = 0b101;

pub mod lattine_e_barattoli_solver;

pub fn solve_1() {
	solve(0b1110111011100101);
}

fn solve(counter: Counter) {
	let counter = counter + 0b01000000000000000000000000000000;
	let num_can_jar = count_can_jar(&counter);
	match bfs(counter, num_can_jar) {
		None => {
			print!("No solution")
		}
		Some(solution) => {
			for step in solution {
				println!("{}", step)
			}
		}
	}
}

fn count_can_jar(counter: &Counter) -> u8 {
	let mut c: u8 = 0;

	for i in COUNTER_ITER.step_by(2) {
		let cell: Cell = Cell::from_counter(counter, i as u8);
		match cell {
			JAR | CAN => c += 1,
			_ => {}
		}
	}

	c
}

type Counter = u32;

#[derive(PartialEq, Clone, Debug)]
enum Cell {
	BLOCKED,
	EMPTY,
	JAR,
	CAN,
}

impl Cell {
	fn from_counter(counter: &Counter, index: u8) -> Self {
		match (counter >> index) & 0b11 {
			0 => BLOCKED,
			1 => EMPTY,
			2 => CAN,
			_ => JAR,
		}
	}

	fn to_int(&self) -> u32 {
		match self {
			BLOCKED => 0,
			EMPTY => 1,
			CAN => 2,
			JAR => 3,
		}
	}
}

type MoveMade = [Grab; 2];

struct VisitedRecord {
	parent: Counter,
	move_made: MoveMade,
}

#[derive(Clone, Debug)]
struct Grab {
	left: Cell,
	right: Cell,
	index: u8,
}

impl Grab {
	fn from_counter(counter: &Counter, index: u8) -> Self {
		Self {
			left: Cell::from_counter(counter, index + 2),
			right: Cell::from_counter(counter, index),
			index,
		}
	}

	fn is_valid_grab(&self) -> bool {
		!(self.left == BLOCKED
			|| self.right == BLOCKED
			|| self.left == EMPTY
			|| self.right == EMPTY)
	}

	fn is_valid_release(&self) -> bool {
		self.left == EMPTY && self.right == EMPTY
	}

	fn to_int(&self) -> u32 {
		(self.left.to_int() << 2) + self.right.to_int()
	}
}

fn bfs(initial_counter: Counter, num_can_jar: u8) -> Option<Vec<String>> {
	let mut frontier: VecDeque<Counter> = VecDeque::new();
	let mut came_from: HashMap<Counter, Option<VisitedRecord>> = HashMap::new();

	frontier.push_back(initial_counter);
	came_from.insert(initial_counter, None);

	while !frontier.is_empty() {
		let current_counter: Counter = frontier.pop_front().unwrap();

		if win(&current_counter, num_can_jar) {
			return Some(reconstruct_path(current_counter, came_from));
		}

		for (possible_counter, possible_move) in possible_moves(&current_counter) {
			match came_from.entry(possible_counter) {
				Entry::Occupied(_) => {}
				Entry::Vacant(entry) => {
					entry.insert(Some(VisitedRecord {
						parent: current_counter,
						move_made: possible_move,
					}));
					frontier.push_back(possible_counter);
				}
			}
		}
	}

	None
}

fn win(counter: &Counter, num_can_jar: u8) -> bool {
	let mut count_mutation: u8 = 0;
	let mut count_can_jar: u8 = 0;
	let first_cell: Cell = Cell::from_counter(counter, 0);
	let mut_to_win: u8 = if first_cell == EMPTY { 2 } else { 1 };
	let mut last_cell: Cell = first_cell;

	for i in COUNTER_ITER.step_by(2) {
		let cell: Cell = Cell::from_counter(counter, i as u8);
		match cell {
			JAR | CAN | EMPTY => {
				match last_cell {
					JAR | CAN | EMPTY => {
						if last_cell != cell {
							count_mutation += 1;
							if count_mutation > mut_to_win {
								return false;
							}
						}
					}
					_ => {}
				}
				if cell != EMPTY {
					count_can_jar += 1;

					if count_can_jar == num_can_jar {
						return true;
					}
				}
			}
			BLOCKED => {
				break;
			}
		}
		last_cell = cell;
	}
	true
}

fn possible_moves(counter: &Counter) -> Vec<(Counter, MoveMade)> {
	let mut moves: Vec<(Counter, MoveMade)> = vec![];

	for i in COUNTER_ITER.step_by(2) {
		let grab: Grab = Grab::from_counter(&counter, i as u8);
		if !grab.is_valid_grab() {
			continue;
		}

		for k in COUNTER_ITER.step_by(2) {
			if k == i {
				continue;
			}

			let release: Grab = Grab::from_counter(&counter, k as u8);

			if !release.is_valid_release() {
				continue;
			}

			let grab_int = grab.to_int();
			let mask = !((0b01111 << i) | (0b01111 << k));
			let mut new_counter = counter & mask;
			new_counter |= grab_int << k;
			new_counter |= GRAB_RELEASE << i;

			moves.push((new_counter, [grab.clone(), release.clone()]))
		}
	}

	moves
}

fn reconstruct_path(
	win_counter: Counter,
	came_from: HashMap<Counter, Option<VisitedRecord>>,
) -> Vec<String> {
	let mut path: Vec<String> = vec![];
	let mut current_counter = win_counter;

	while let Some(visited_record) = came_from.get(&current_counter) {
		match visited_record {
			None => {
				break;
			}
			Some(record) => {
				path.push(decipher_move(&record.move_made));
				current_counter = record.parent;
			}
		}
	}

	path.reverse();
	path
}

fn decipher_move(move_made: &MoveMade) -> String {
	format!(
		"{} {}",
		move_made[0].index / 2 + 1,
		move_made[1].index / 2 + 1
	)
}
