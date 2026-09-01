use crate::klotski::symmetry_meta::SymmetryMeta;
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Board {
	pub data: [u8; 72],
	pub len: usize,
}

impl Board {
	pub fn len(&self) -> usize {
		self.len
	}

	pub fn iter(&self) -> impl Iterator<Item = &u8> {
		self.data[..self.len].iter()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut u8> {
		self.data[..self.len].iter_mut()
	}

	pub fn normalize(&mut self, meta: &SymmetryMeta) {
		let mut map = [0u8; 128];
		let mut next_idx = [0usize; 16];

		for i in 0..self.len {
			let c = self.data[i];
			let g = meta.piece_group[c as usize];

			if g > 0 {
				if map[c as usize] == 0 {
					map[c as usize] = meta.canonical_chars[g as usize][next_idx[g as usize]];
					next_idx[g as usize] += 1;
				}
				self.data[i] = map[c as usize];
			}
		}
	}
}

impl Index<usize> for Board {
	type Output = u8;
	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl IndexMut<usize> for Board {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.data[index]
	}
}
