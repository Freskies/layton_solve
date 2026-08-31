pub fn solve_1() {
	solve()
}

pub fn solve_2() {
	solve()
}

fn solve() {}

const INVALID_POINTS: [Point; 16] = [
	// bottom left
	Point { x: 0, y: 0 },
	Point { x: 0, y: 1 },
	Point { x: 1, y: 0 },
	Point { x: 1, y: 1 },
	// bottom right
	Point { x: 5, y: 0 },
	Point { x: 5, y: 1 },
	Point { x: 6, y: 0 },
	Point { x: 6, y: 1 },
	// top left
	Point { x: 0, y: 5 },
	Point { x: 0, y: 6 },
	Point { x: 1, y: 5 },
	Point { x: 1, y: 6 },
	// top right
	Point { x: 5, y: 5 },
	Point { x: 5, y: 6 },
	Point { x: 6, y: 5 },
	Point { x: 6, y: 6 },
];

const DIRECTIONS: [(Point, Point); 4] = [
	(Point { x: 0, y: 2 }, Point { x: 0, y: 1 }),
	(Point { x: 2, y: 0 }, Point { x: 1, y: 0 }),
	(Point { x: 0, y: -2 }, Point { x: 0, y: -1 }),
	(Point { x: -2, y: 0 }, Point { x: -1, y: 0 }),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
	x: i8,
	y: i8,
}

struct BallMove {
	start: Point,
	mid: Point,
	end: Point,
}

impl Point {
	fn is_valid(&self) -> bool {
		(0..6).contains(&self.x) && (0..6).contains(&self.y) && !INVALID_POINTS.contains(&self)
	}

	fn add(&self, other: &Point) -> Point {
		Point {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}

	fn possible_moves(&self, points: Vec<Point>) -> Vec<BallMove> {
		let mut possible_moves = Vec::new();
		for direction in DIRECTIONS {
			let ball_move: BallMove = BallMove {
				start: Point::from(*self),
				mid: self.add(&direction.1),
				end: self.add(&direction.0),
			};
			if ball_move.start.is_valid()
				&& !points.contains(&ball_move.end)
				&& points.contains(&ball_move.mid)
			{
				possible_moves.push(ball_move);
			}
		}
		possible_moves
	}
}
