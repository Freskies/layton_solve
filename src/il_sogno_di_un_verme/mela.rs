#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Mela {
	pub pos_to_win: usize,
}

impl Mela {
	pub fn init(pos_to_win: usize) -> Self {
		Mela { pos_to_win }
	}
}
