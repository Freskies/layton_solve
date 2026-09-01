pub fn solve_1() {
	solve(vec![
		Point { x: 2, y: 4 },
		Point { x: 3, y: 2 },
		Point { x: 3, y: 3 },
		Point { x: 3, y: 4 },
		Point { x: 3, y: 5 },
		Point { x: 4, y: 4 },
	])
}

pub fn solve_2() {
	solve(vec![
		Point { x: 1, y: 3 },
		Point { x: 2, y: 3 },
		Point { x: 3, y: 1 },
		Point { x: 3, y: 2 },
		Point { x: 3, y: 3 },
		Point { x: 3, y: 4 },
		Point { x: 3, y: 5 },
		Point { x: 4, y: 3 },
		Point { x: 5, y: 3 },
	])
}

pub fn solve_3() {
	solve(vec![])
}

pub fn solve_4() {
	solve(vec![
		Point { x: 2, y: 0 },
		Point { x: 2, y: 1 },
		Point { x: 2, y: 2 },
		Point { x: 2, y: 3 },
		Point { x: 3, y: 0 },
		Point { x: 3, y: 1 },
		Point { x: 3, y: 2 },
		Point { x: 4, y: 0 },
		Point { x: 4, y: 1 },
		Point { x: 4, y: 2 },
		Point { x: 4, y: 3 },
	])
}

fn solve(balls: Vec<Point>) {
	let mut solution: Vec<BallMove> = vec![];
	if dfs(balls, &mut solution) {
		decode_solution(solution)
			.iter()
			.for_each(|step| println!("{step}"))
	} else {
		println!("No solutions found.")
	}
}

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

const ARROWS: [char; 9] = ['↙', '↓', '↘', '←', '•', '→', '↖', '↑', '↗'];

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

	fn possible_moves(&self, points: &Vec<Point>) -> Vec<BallMove> {
		let mut possible_moves = Vec::new();
		for direction in DIRECTIONS {
			let ball_move: BallMove = BallMove {
				start: Point::from(*self),
				mid: self.add(&direction.1),
				end: self.add(&direction.0),
			};
			if ball_move.end.is_valid()
				&& !points.contains(&ball_move.end)
				&& points.contains(&ball_move.mid)
			{
				possible_moves.push(ball_move);
			}
		}
		possible_moves
	}

	fn decode(&self) -> String {
		let mut str: String = "".to_string();
		let mut x = self.x;
		let mut y = self.y;

		str.push(if self.x <= 1 {
			x += 2;
			'←'
		} else if self.x >= 5 {
			x -= 2;
			'→'
		} else if self.y <= 1 {
			y += 2;
			'↓'
		} else if self.y >= 5 {
			y -= 2;
			'↑'
		} else {
			'•'
		});

		x -= 2;
		y -= 2;

		str.push(ARROWS[(y * 3 + x) as usize]);

		str
	}
}

impl BallMove {
	fn decode_direction(&self) -> char {
		if self.start.x > self.end.x {
			'←'
		} else if self.start.x < self.end.x {
			'→'
		} else if self.start.y > self.end.y {
			'↓'
		} else {
			'↑'
		}
	}
}

fn do_move(balls: &mut Vec<Point>, ball_move: &BallMove) {
	balls.remove(balls.iter().position(|&p| p == ball_move.start).unwrap());
	balls.remove(balls.iter().position(|&p| p == ball_move.mid).unwrap());
	balls.push(ball_move.end);
}

fn undo_move(balls: &mut Vec<Point>, ball_move: &BallMove) {
	balls.remove(balls.iter().position(|&p| p == ball_move.end).unwrap());
	balls.push(ball_move.start);
	balls.push(ball_move.mid);
}

fn dfs(mut balls: Vec<Point>, solution: &mut Vec<BallMove>) -> bool {
	if balls.len() == 1 {
		return true;
	}

	for ball in balls.clone() {
		for ball_move in ball.possible_moves(&balls.clone()) {
			do_move(&mut balls, &ball_move);
			if dfs(balls.clone(), solution) {
				solution.push(ball_move);
				return true;
			}
			undo_move(&mut balls, &ball_move)
		}
	}

	false
}

fn decode_solution(mut solution: Vec<BallMove>) -> Vec<String> {
	solution.reverse();
	solution
		.iter()
		.map(|step| format!("{} {}", step.start.decode(), step.decode_direction()))
		.collect()
}
