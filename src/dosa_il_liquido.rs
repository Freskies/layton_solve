use crate::dosa_il_liquido::counter::Counter;
use crate::dosa_il_liquido::recipient::Recipient;

pub mod counter;
pub mod recipient;

pub fn solve_succo() {
	solve(Counter::init(vec![
		Recipient { drink: 8, space: 8 },
		Recipient { drink: 0, space: 5 },
		Recipient { drink: 0, space: 3 },
	]));
}

pub fn solve_latte() {
	solve(Counter::init(vec![
		Recipient { drink: 10, space: 10 },
		Recipient { drink: 0, space: 7 },
		Recipient { drink: 0, space: 3 },
	]));
}

pub fn solve_acqua() {
	solve(Counter::init(vec![
		Recipient { drink: 16, space: 16 },
		Recipient { drink: 0, space: 9 },
		Recipient { drink: 0, space: 7 },
	]));
}

fn solve(mut counter: Counter) {
	let solution: Vec<String> = counter.solve();
	for step in solution {
		println!("{}", step);
	}
}
