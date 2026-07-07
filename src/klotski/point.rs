use crate::klotski::piece_move::LegalMove;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Point(pub usize, pub usize);

impl Point {
	pub fn to_index(&self, width: usize) -> usize {
		(self.1 * width) + self.0
	}

	pub fn from_index(index: usize, width: usize) -> Point {
		Point(index % width, index / width)
	}

	pub fn add(self, legal_move: &LegalMove, width: usize, height: usize) -> Option<Point> {
		let point_addendum = Self::direction_to_point_addendum(legal_move);
		let new_x = self.0 as isize + point_addendum.0;
		let new_y = self.1 as isize + point_addendum.1;

		if new_x >= 0 && new_y >= 0 && (new_x as usize) < width && (new_y as usize) < height {
			return Some(Point(new_x as usize, new_y as usize));
		}
		None
	}

	const fn direction_to_point_addendum(legal_move: &LegalMove) -> (isize, isize) {
		match legal_move {
			LegalMove::UP => (0, -1),
			LegalMove::RIGHT => (1, 0),
			LegalMove::DOWN => (0, 1),
			LegalMove::LEFT => (-1, 0),
		}
	}
}
