use std::collections::{HashMap, VecDeque};

// 3bit per rana
// 111 -> vuoto
// mappa: 7pali -> 21bit
// posso usare un u32 per l'intera mappa

pub fn solve_rane_saltellanti() {
	let colors: Vec<&str> = vec!["Gialla", "Rossa", "Verde", "Azzurra"];
	let start: u32 = 0b_001_010_111_011_100;
	let target: u32 = 0b_011_100_111_001_010;
	solve(start, target, colors);
}

pub fn solve_salti_sotto_la_luna() {
	let colors: Vec<&str> = vec!["Azzurra", "Nera", "Verde", "Rossa", "Bianca", "Gialla"];
	let start: u32 = 0b_001_010_011_111_100_101_110;
	let target: u32 = 0b_110_011_001_111_101_010_100;
	solve(start, target, colors);
}

fn solve(start: u32, target: u32, colors: Vec<&str>) {
	let solution = bfs(start, target);
	solution
		.iter()
		.for_each(|step| println!("{}", decode_move(step.0, step.1, &colors)))
}

fn bfs(root: u32, target: u32) -> Vec<(u32, u32)> {
	let mut visited: HashMap<u32, u32> = HashMap::new(); // K: stato, V: padre
	visited.insert(root, 0u32);
	let mut queue: VecDeque<u32> = VecDeque::new();
	queue.push_back(root);

	while let Some(state) = queue.pop_front() {
		if state == target {
			return traverse_solution(target, visited);
		}

		for next_state in get_possible_moves(state) {
			if !visited.contains_key(&next_state) {
				visited.insert(next_state, state);
				queue.push_back(next_state);
			}
		}
	}

	panic!("Solution not found")
}

fn traverse_solution(target: u32, visited: HashMap<u32, u32>) -> Vec<(u32, u32)> {
	let mut solution: Vec<(u32, u32)> = vec![];
	let mut current: u32 = target;

	while let Some(father) = visited.get(&current) {
		if *father == 0 { break }
		solution.push((*father, current));
		current = *father;
	}

	solution.reverse();
	solution
}

fn get_possible_moves(state: u32) -> Vec<u32> {
	let mut new_states: Vec<u32> = vec![];
	let mut frogs_to_move = get_frogs_can_move(state);

	while frogs_to_move != 0 {
		let frog_to_move: u32 = (frogs_to_move & 0b111) as u32;
		new_states.push(jump(frog_to_move, state));
		frogs_to_move >>= 3;
	}

	new_states
}

// return max 12 bits: every 3bit non 0 are a frog that can move
fn get_frogs_can_move(mut state: u32) -> u16 {
	let mut frogs_to_check: u16 = 0u16;
	let mut after_minus_one: u8 = 0u8;

	while state != 0 {
		let current: u16 = (state & 0b111) as u16;
		if current != 0b111 {
			frogs_to_check = (frogs_to_check << 3) + current;
		}

		if after_minus_one > 0 || current == 0b111 {
			after_minus_one += 1;

			if after_minus_one == 3 {
				break;
			}
		}

		state >>= 3;
	}

	let after_zero = after_minus_one - 1;
	frogs_to_check & (0b_111_111_111_111 >> ((2 - after_zero) * 3))
}

fn jump(frog: u32, mut state: u32) -> u32 {
	let mut new_state: u32 = 0u32;
	let mut pos: u32 = 0;
	while state != 0 {
		let current_frog: u32 = state & 0b111;
		new_state += if current_frog == 0b111 {
			frog
		} else if current_frog == frog {
			0b111
		} else {
			current_frog
		} << (3 * pos);
		pos += 1;
		state >>= 3;
	}
	new_state
}

fn decode_move(mut start: u32, end: u32, colors: &Vec<&str>) -> String {
	let mut mask: u32 = 0u32;
	let mut pos: u32 = 0;
	while start != 0 {
		let current_frog = start & 0b111;
		if current_frog == 0b111 {
			mask += 0b111 << (3 * pos);
			break;
		}
		mask += 0b000 << (3 * pos);
		pos += 1;
		start >>= 3;
	}

	let frog: u32 = (end & mask) >> (pos * 3);
	colors.get((frog - 1) as usize).unwrap().to_string()
}
