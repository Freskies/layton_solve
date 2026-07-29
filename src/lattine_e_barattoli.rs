use crate::lattine_e_barattoli::Cell::{BLOCKED, CAN, EMPTY, JAR};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

pub mod lattine_e_barattoli_solver;

pub fn solve_1() {}

type Counter = u32; // 0b[xx]* -> 2bit per cell

enum Cell {
	EMPTY,
	JAR,
	CAN,
	BLOCKED,
}

impl Cell {
	fn from_counter(counter: Counter, index: u8) -> Self {
		let cell = (counter >> index) & 0b11;
		if cell == 0 {
			EMPTY
		} else if cell == 1 {
			JAR
		} else if cell == 2 {
			CAN
		} else {
			BLOCKED
		}
	}
}

type MoveMade = [Cell; 2];

struct VisitedRecord {
	parent: Counter,
	move_made: MoveMade,
}

fn solve(counter: Counter) {}

fn bfs(initial_counter: Counter) -> Option<Vec<String>> {
	let mut frontier: VecDeque<Counter> = VecDeque::new();
	let mut came_from: HashMap<Counter, Option<VisitedRecord>> = HashMap::new();

	frontier.push_back(initial_counter);
	came_from.insert(initial_counter, None);

	while !frontier.is_empty() {
		let current_counter: Counter = frontier.pop_front().unwrap();

		if win() {
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

fn possible_moves(counter: &Counter) -> Vec<(Counter, MoveMade)> {
	vec![]
}

fn win() -> bool {
	false
}

fn reconstruct_path(
	win_counter: Counter,
	came_from: HashMap<Counter, Option<VisitedRecord>>,
) -> Vec<String> {
	vec![]
}
