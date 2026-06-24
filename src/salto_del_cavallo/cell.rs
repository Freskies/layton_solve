#[derive(Copy, Clone)]
pub enum Cell {
	CHECKED,
	UNCHECKED,
}

impl Cell {
	pub fn to_string(&self) -> &str {
		match self {
			Cell::CHECKED => "O",
			Cell::UNCHECKED => "X",
		}
	}
}
