use crate::frittelle_impilate::frittelle_impilate_solver::FrittelleImpilateSolver;

pub mod frittelle_impilate_solver;

pub fn solve_1() {
	solve(3);
}

pub fn solve_2() { solve(4); }

pub fn solve_3() { solve(5); }


fn solve(num_frittelle: u8) {
	let mut solver = FrittelleImpilateSolver::init(num_frittelle);
	solver.bfs();
	if let Some(path) = solver.victory_path {
		for act in path {
			println!("{act}")
		}
	} else {
		println!("Nessuna sol trovata");
	}
}

