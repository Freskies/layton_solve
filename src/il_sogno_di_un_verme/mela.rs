#[derive(Copy, Clone)]
pub struct Mela {
	pub pos_to_win: usize,
}

impl Mela {
	pub fn init(pos_to_win: usize) -> Self {
		Mela { pos_to_win }
	}

	pub fn serialize(&self) -> String {
		format!("{}", self.pos_to_win)
	}
}
