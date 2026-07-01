use std::cmp::min;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Recipient {
	pub drink: u8,
	pub space: u8,
}

impl Recipient {
	pub fn can_pour(&self) -> bool {
		self.drink != 0
	}

	pub fn can_hold(&self) -> bool {
		self.hold_space() != 0
	}

	pub fn hold_space(&self) -> u8 {
		self.space - self.drink
	}

	pub fn pour(&mut self, r: &mut Recipient) -> u8 {
		let drink_to_pour = min(self.drink, r.hold_space());
		self.drink -= drink_to_pour;
		r.drink += drink_to_pour;
		drink_to_pour
	}

	pub fn get_name(&self) -> String {
		format!("{}L", self.space)
	}
}
