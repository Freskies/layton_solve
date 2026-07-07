use crate::klotski::board::Board;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SymmetryMeta {
	pub piece_group: [u8; 128],
	pub canonical_chars: [[u8; 16]; 16],
}

impl SymmetryMeta {
	pub fn build(initial_board: &Board, width: usize) -> Self {
		let mut min_x = [255usize; 128];
		let mut max_x = [0usize; 128];
		let mut min_y = [255usize; 128];
		let mut max_y = [0usize; 128];
		let mut seen = [false; 128];

		for i in 0..initial_board.len {
			let c = initial_board[i];
			if (b'0'..=b'9').contains(&c) {
				continue;
			}
			let idx = c as usize;
			seen[idx] = true;
			let x = i % width;
			let y = i / width;

			if x < min_x[idx] {
				min_x[idx] = x;
			}
			if x > max_x[idx] {
				max_x[idx] = x;
			}
			if y < min_y[idx] {
				min_y[idx] = y;
			}
			if y > max_y[idx] {
				max_y[idx] = y;
			}
		}

		let mut groups: HashMap<(usize, usize), Vec<u8>> = HashMap::new();
		for c in 0..128 {
			if seen[c] {
				let w = max_x[c] - min_x[c] + 1;
				let h = max_y[c] - min_y[c] + 1;
				groups.entry((w, h)).or_default().push(c as u8);
			}
		}

		let mut meta = SymmetryMeta {
			piece_group: [0; 128],
			canonical_chars: [[0; 16]; 16],
		};

		let mut group_id = 1;
		for (_, mut chars) in groups {
			chars.sort(); // Ordine alfabetico per coerenza
			for (i, &c) in chars.iter().enumerate() {
				meta.piece_group[c as usize] = group_id as u8;
				meta.canonical_chars[group_id][i] = c;
			}
			group_id += 1;
		}

		meta
	}
}
