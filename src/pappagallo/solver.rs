use crate::pappagallo::configuration::Configuration;
use crate::pappagallo::point::Point;
use crate::pappagallo::segment::Segment;
use std::{error::Error, fs};

pub type Rope = (usize, usize);
pub type Solution = Vec<Rope>;

pub fn solve_file(path: &str, rope_count: usize) -> Result<(), Box<dyn Error>> {
	let json = fs::read_to_string(path)?;
	let configuration: Configuration = serde_json::from_str(&json)?;

	validate(&configuration)?;

	let solutions = generate(&configuration, rope_count, &[])?;

	println!("Configurazioni trovate: {}", solutions.len());

	for (index, solution) in solutions.iter().enumerate() {
		let text = solution
			.iter()
			.map(|&(a, b)| format!("{}{}", letter(a), letter(b)))
			.collect::<Vec<_>>()
			.join(" ");

		println!("{:04}: {text}", index + 1);
	}

	Ok(())
}

fn letter(index: usize) -> char {
	(b'A' + index as u8) as char
}

fn validate(configuration: &Configuration) -> Result<(), Box<dyn Error>> {
	if configuration.points.len() > 26 {
		return Err("Sono supportati al massimo 26 punti (A-Z)".into());
	}

	let all_points = configuration.points.iter().copied().chain(
		configuration
			.obstacles
			.iter()
			.flat_map(|segment| [segment.start, segment.end]),
	);

	for point in all_points {
		if !point.x.is_finite()
			|| !point.y.is_finite()
			|| !(0.0..=1.0).contains(&point.x)
			|| !(0.0..=1.0).contains(&point.y)
		{
			return Err("Le coordinate devono essere comprese tra 0 e 1".into());
		}
	}

	for (index, a) in configuration.points.iter().enumerate() {
		for b in &configuration.points[index + 1..] {
			if a.x == b.x && a.y == b.y {
				return Err("Due ancoraggi hanno la stessa posizione".into());
			}
		}
	}

	Ok(())
}

pub fn generate(
	configuration: &Configuration,
	rope_count: usize,
	required: &[Rope],
) -> Result<Vec<Solution>, Box<dyn Error>> {
	validate(configuration)?;

	if required.len() > rope_count {
		return Err("Le corde obbligatorie superano il numero di corde richiesto".into());
	}

	let candidates = legal_ropes(configuration);
	let mut used = vec![false; configuration.points.len()];
	let mut current: Solution = Vec::new();

	for &(a, b) in required {
		if a >= used.len() || b >= used.len() || a == b {
			return Err("Una corda obbligatoria ha estremi non validi".into());
		}

		let rope = (a.min(b), a.max(b));

		if !candidates.contains(&rope) {
			return Err(format!(
				"La corda {}{} attraversa un ostacolo",
				letter(rope.0),
				letter(rope.1),
			)
			.into());
		}

		if used[a] || used[b] {
			return Err("Due corde obbligatorie utilizzano lo stesso ancoraggio".into());
		}

		let segment = segment_for(configuration, rope);

		if current
			.iter()
			.any(|&existing| intersects(segment, segment_for(configuration, existing)))
		{
			return Err("Due corde obbligatorie si intersecano".into());
		}

		used[a] = true;
		used[b] = true;
		current.push(rope);
	}

	if rope_count > configuration.points.len() / 2 {
		return Ok(Vec::new());
	}

	let mut solutions = Vec::new();

	search(
		configuration,
		&candidates,
		0,
		rope_count,
		&mut used,
		&mut current,
		&mut solutions,
	);

	// Ordine stabile delle corde anche quando alcune erano obbligatorie.
	for solution in &mut solutions {
		solution.sort_unstable();
	}

	Ok(solutions)
}

// Elenca le singole corde che non attraversano ostacoli.
fn legal_ropes(configuration: &Configuration) -> Vec<Rope> {
	let mut candidates = Vec::new();

	for a in 0..configuration.points.len() {
		for b in a + 1..configuration.points.len() {
			let rope = segment_for(configuration, (a, b));

			let blocked = configuration
				.obstacles
				.iter()
				.any(|&obstacle| intersects(rope, obstacle));

			if !blocked {
				candidates.push((a, b));
			}
		}
	}

	candidates
}

fn segment_for(configuration: &Configuration, rope: Rope) -> Segment {
	Segment {
		start: configuration.points[rope.0],
		end: configuration.points[rope.1],
	}
}

fn search(
	configuration: &Configuration,
	candidates: &[Rope],
	start_index: usize,
	target: usize,
	used: &mut [bool],
	current: &mut Solution,
	solutions: &mut Vec<Solution>,
) {
	if current.len() == target {
		solutions.push(current.clone());
		return;
	}

	let missing = target - current.len();
	let free_points = used.iter().filter(|&&value| !value).count();

	// Interrompe rami che non possono raggiungere il numero richiesto.
	if free_points / 2 < missing || candidates.len() - start_index < missing {
		return;
	}

	for index in start_index..candidates.len() {
		let (a, b) = candidates[index];

		if used[a] || used[b] {
			continue;
		}

		let proposed = segment_for(configuration, (a, b));

		let crosses_existing = current
			.iter()
			.any(|&rope| intersects(proposed, segment_for(configuration, rope)));

		if crosses_existing {
			continue;
		}

		used[a] = true;
		used[b] = true;
		current.push((a, b));

		search(
			configuration,
			candidates,
			index + 1,
			target,
			used,
			current,
			solutions,
		);

		// Ripristina lo stato prima di provare un'altra corda.
		current.pop();
		used[a] = false;
		used[b] = false;
	}
}

// Tolleranza numerica, non spessore degli ostacoli.
const EPSILON: f64 = 1e-9;

// Segno del prodotto vettoriale AB × AC.
fn orientation(a: Point, b: Point, c: Point) -> i8 {
	let ab_x = b.x as f64 - a.x as f64;
	let ab_y = b.y as f64 - a.y as f64;
	let ac_x = c.x as f64 - a.x as f64;
	let ac_y = c.y as f64 - a.y as f64;

	let cross = ab_x * ac_y - ab_y * ac_x;

	if cross > EPSILON {
		1
	} else if cross < -EPSILON {
		-1
	} else {
		0
	}
}

// Usata quando il punto è già risultato collineare al segmento.
fn inside_bounds(point: Point, segment: Segment) -> bool {
	let x = point.x as f64;
	let y = point.y as f64;

	x >= segment.start.x.min(segment.end.x) as f64 - EPSILON
		&& x <= segment.start.x.max(segment.end.x) as f64 + EPSILON
		&& y >= segment.start.y.min(segment.end.y) as f64 - EPSILON
		&& y <= segment.start.y.max(segment.end.y) as f64 + EPSILON
}

fn intersects(first: Segment, second: Segment) -> bool {
	let a = first.start;
	let b = first.end;
	let c = second.start;
	let d = second.end;

	let o1 = orientation(a, b, c);
	let o2 = orientation(a, b, d);
	let o3 = orientation(c, d, a);
	let o4 = orientation(c, d, b);

	// Incrocio interno tra segmenti.
	if o1 * o2 < 0 && o3 * o4 < 0 {
		return true;
	}

	// Contatti agli estremi e sovrapposizioni collineari.
	(o1 == 0 && inside_bounds(c, first))
		|| (o2 == 0 && inside_bounds(d, first))
		|| (o3 == 0 && inside_bounds(a, second))
		|| (o4 == 0 && inside_bounds(b, second))
}
