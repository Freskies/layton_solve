use std::collections::HashSet;

pub fn solve() {
	for i in 33333..44444 {
		let num = i - 33333;
		let num_str = num.to_string();

		if num_str.len() != 4 {
			continue;
		}

		let i_str = i.to_string();
		let mut num_set: HashSet<char> = HashSet::new();
		num_set.extend(num_str.chars());
		num_set.extend(i_str.chars());

		if num_set.contains(&'0') {
			continue;
		}

		if num_set.len() == 9 {
			println!("{i} {num}")
		}
	}
}
