pub mod app;
pub mod configuration;
pub mod point;
pub mod segment;
pub mod solver;

use crate::pappagallo::app::PappagalloApp;
use eframe::egui;

pub fn solve_1() { solve("asset/pappagallo1.png"); }
pub fn solve_2() { solve("asset/pappagallo2.png"); }
pub fn solve_3() { solve("asset/pappagallo3.png"); }
pub fn solve_4() { solve("asset/pappagallo4.png"); }
pub fn solve_5() { solve("asset/pappagallo5.png"); }
pub fn solve_6() { solve("asset/pappagallo6.png"); }
pub fn solve_7() { solve("asset/pappagallo7.png"); }
pub fn solve_8() { solve("asset/pappagallo8.png"); }
pub fn solve_9() { solve("asset/pappagallo9.png"); }
pub fn solve_10() { solve("asset/pappagallo10.png"); }
pub fn solve_11() { solve("asset/pappagallo11.png"); }
pub fn solve_12() { solve("asset/pappagallo12.png"); }

fn solve(path: &str) {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 850.0]),
		..Default::default()
	};

	eframe::run_native(
		"Pappagallo di merda",
		options,
		Box::new(|cc| {
			egui_extras::install_image_loaders(&cc.egui_ctx);
			Ok(Box::new(PappagalloApp::new(path)))
		}),
	).expect("Why don't you party?");
}
