use crate::salto_del_cavallo::point::Point;

#[derive(Debug)]
pub struct HorseMove {
	pub(crate) point: Point,
	pub(crate) name: &'static str,
}

impl HorseMove {
	pub fn init() -> Self {
		Self {
			point: Point::init(),
			name: "START",
		}
	}

	pub fn print(&self) {
		print!("{}", self.name)
	}
}

pub const HORSE_MOVES: [HorseMove; 8] = [
	HorseMove {
		point: Point { x: 2, y: -1 },
		name: "DESTRA-SU",
	},
	HorseMove {
		point: Point { x: 2, y: 1 },
		name: "DESTRA-GIU",
	},
	HorseMove {
		point: Point { x: -1, y: 2 },
		name: "GIU-SINISTRA",
	},
	HorseMove {
		point: Point { x: 1, y: 2 },
		name: "GIU-DESTRA",
	},
	HorseMove {
		point: Point { x: -2, y: -1 },
		name: "SINISTRA-SU",
	},
	HorseMove {
		point: Point { x: -2, y: 1 },
		name: "SINISTRA-GIU",
	},
	HorseMove {
		point: Point { x: -1, y: -2 },
		name: "SU-SINISTRA",
	},
	HorseMove {
		point: Point { x: 1, y: -2 },
		name: "SU-DESTRA",
	},
];