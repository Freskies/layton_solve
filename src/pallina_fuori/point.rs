#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Point(pub usize, pub usize);

impl Point {
	pub fn add(self, point_addendum: (isize, isize)) -> Option<Point> {
		if self.0 as isize + point_addendum.0 > 0 && self.1 as isize + point_addendum.1 > 0 {
			let x = (self.0 as isize + point_addendum.0) as usize;
			let y = (self.1 as isize + point_addendum.1) as usize;
			return Some(Point(x, y));
		}
		None
	}
}
