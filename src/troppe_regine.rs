pub fn solve() {
	//first_solve();
	all_solve();
}

#[allow(unused)]
fn all_solve() {
	let mut state: [u8; 8] = [0; 8];

	for n in 0..(0b111111111111111111111111) {
		let mut k = n.clone();
		for i in 0..8 {
			state[i] = (k & 0b111) as u8 + 1;
			k = k >> 3;
		}
		if win(&state) {
			println!("{:?}", state);
		}
	}
}

#[allow(unused)]
fn first_solve() {
	let mut state: [u8; 8] = [0; 8];
	let mut n: u32 = 0;

	while !win(&state) {
		n += 1;
		let mut k = n.clone();
		for i in 0..8 {
			state[i] = (k & 0b111) as u8;
			k = k >> 3;
		}
	}

	println!("{:?}", state);
}

fn win(state: &[u8; 8]) -> bool {
	for row in 0..8 {
		for prev_row in 0..row {
			let col = state[row] as i8;
			let prev_col = state[prev_row] as i8;

			if col == prev_col {
				return false;
			}

			let row_diff = (row - prev_row) as i8;
			let col_diff = (col - prev_col).abs();

			if row_diff == col_diff {
				return false;
			}
		}
	}

	true
}
