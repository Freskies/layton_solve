use crate::al_parcheggio::car::{Car, CarDirection};

pub struct Board {
	board: [[Option<Car>; 6]; 6],
}

impl Board {
	pub fn init() -> Self {
		let b = Board {
			board: [[None; 6]; 6],
		};

		b
	}

	pub fn print(&self) {
		for row in &self.board {
			row.into_iter().for_each(|cell| {
				if let Some(car) = cell {
					print!("{}", match car.direction {
						CarDirection::Up => "↑",
						CarDirection::Right => "→",
						CarDirection::Down => "↓",
						CarDirection::Left => "←"
					})
				} else {
					print!("· ");
				}
			});
			println!()
		}
		println!()
	}
}
