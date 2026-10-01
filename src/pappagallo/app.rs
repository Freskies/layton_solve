use std::sync::Arc;
use crate::pappagallo::configuration::Configuration;
use crate::pappagallo::point::Point;
use crate::pappagallo::segment::Segment;
use crate::pappagallo::solver::{self, Rope, Solution};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
	Points,
	Obstacles,
	RequiredRopes,
}

pub struct PappagalloApp {
	image_bytes: Arc<[u8]>,
	points: Vec<Point>,
	obstacles: Vec<Segment>,

	mode: Mode,
	pending_start: Option<Point>,

	required_ropes: Vec<Rope>,
	pending_anchor: Option<usize>,
	rope_count: usize,

	solutions: Vec<Solution>,
	solution_index: usize,
	status: String,
}

impl PappagalloApp {
	pub(crate) fn new(path: &str) -> Self {
		let bytes = std::fs::read(path)
			.expect("Impossibile leggere {path}");

		Self {
			image_bytes: bytes.into(),
			points: Vec::new(),
			obstacles: Vec::new(),
			mode: Mode::Points,
			pending_start: None,
			required_ropes: Vec::new(),
			pending_anchor: None,
			rope_count: 5,
			solutions: Vec::new(),
			solution_index: 0,
			status: String::new(),
		}
	}

	fn show_toolbar(&mut self, ui: &mut egui::Ui) {
		ui.heading("Layton Solver");

		self.show_edit_controls(ui);
		self.show_solver_controls(ui);

		let instructions = match self.mode {
			Mode::Points => "Clicca per aggiungere un punto (A-Z).",
			Mode::Obstacles => {
				"Clicca due volte per disegnare un ostacolo. Esc annulla."
			}
			Mode::RequiredRopes => {
				"Clicca due ancoraggi per fissare una corda. Ripeti per rimuoverla."
			}
		};

		ui.label(instructions);

		if !self.status.is_empty() {
			ui.label(&self.status);
		}

		ui.separator();
	}

	fn show_edit_controls(&mut self, ui: &mut egui::Ui) {
		let previous_mode = self.mode;

		ui.horizontal(|ui| {
			ui.selectable_value(&mut self.mode, Mode::Points, "Punti");
			ui.selectable_value(&mut self.mode, Mode::Obstacles, "Ostacoli");
			ui.selectable_value(
				&mut self.mode,
				Mode::RequiredRopes,
				"Corde obbligatorie",
			);
		});

		if self.mode != previous_mode {
			self.cancel_pending();
		}

		ui.horizontal(|ui| {
			if ui.button("Annulla ultimo").clicked() {
				self.undo();
			}

			if ui.button("Esporta configurazione").clicked() {
				self.export_configuration();
			}

			ui.label(format!(
				"{} punti · {} ostacoli · {} corde fissate",
				self.points.len(),
				self.obstacles.len(),
				self.required_ropes.len(),
			));
		});
	}

	fn show_solver_controls(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(format!("Corde totali: {}", self.rope_count));

			if ui.button("−").clicked() && self.rope_count > 0 {
				self.rope_count -= 1;
				self.invalidate_results();
			}

			if ui.button("+").clicked() && self.rope_count < 13 {
				self.rope_count += 1;
				self.invalidate_results();
			}

			if ui.button("Calcola").clicked() {
				self.calculate();
			}

			if ui.button("Libera tutte le corde").clicked() {
				self.required_ropes.clear();
				self.pending_anchor = None;
				self.invalidate_results();
			}
		});

		if self.solutions.is_empty() {
			return;
		}

		ui.horizontal(|ui| {
			if ui.button("Precedente").clicked() && self.solution_index > 0 {
				self.solution_index -= 1;
			}

			ui.label(format!(
				"{} / {}",
				self.solution_index + 1,
				self.solutions.len(),
			));

			if ui.button("Successiva").clicked()
				&& self.solution_index + 1 < self.solutions.len()
			{
				self.solution_index += 1;
			}
		});

		let text = self.solutions[self.solution_index]
			.iter()
			.map(|&(a, b)| {
				format!(
					"{}{}",
					(b'A' + a as u8) as char,
					(b'A' + b as u8) as char,
				)
			})
			.collect::<Vec<_>>()
			.join(" ");

		ui.label(text);
	}

	fn configuration(&self) -> Configuration {
		Configuration {
			points: self.points.clone(),
			obstacles: self.obstacles.clone(),
		}
	}

	fn calculate(&mut self) {
		self.cancel_pending();
		self.invalidate_results();

		match solver::generate(
			&self.configuration(),
			self.rope_count,
			&self.required_ropes,
		) {
			Ok(solutions) => {
				self.status = format!(
					"Configurazioni trovate: {}",
					solutions.len(),
				);
				self.solutions = solutions;
			}
			Err(error) => {
				self.status = format!("Errore: {error}");
			}
		}
	}

	fn invalidate_results(&mut self) {
		self.solutions.clear();
		self.solution_index = 0;
		self.status.clear();
	}

	fn cancel_pending(&mut self) {
		self.pending_start = None;
		self.pending_anchor = None;
	}

	fn undo(&mut self) {
		let changed = match self.mode {
			Mode::Points => {
				let removed = self.points.pop().is_some();

				if removed {
					let count = self.points.len();

					self.required_ropes.retain(|&(a, b)| {
						a < count && b < count
					});

					self.pending_anchor = None;
				}

				removed
			}
			Mode::Obstacles => {
				if self.pending_start.take().is_some() {
					false
				} else {
					self.obstacles.pop().is_some()
				}
			}
			Mode::RequiredRopes => {
				if self.pending_anchor.take().is_some() {
					false
				} else {
					self.required_ropes.pop().is_some()
				}
			}
		};

		if changed {
			self.invalidate_results();
		}
	}

	fn handle_keyboard(&mut self, ui: &egui::Ui) {
		if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
			self.cancel_pending();
		}
	}

	fn show_canvas(&mut self, ui: &mut egui::Ui) {
		let response = ui.add(
			egui::Image::from_bytes(
				"bytes://level.png",
				self.image_bytes.clone(),
			)
				.fit_to_exact_size(ui.available_size())
				.maintain_aspect_ratio(true)
				.sense(egui::Sense::click()),
		);

		let rect = response.rect;

		if rect.width() <= 0.0 || rect.height() <= 0.0 {
			return;
		}

		if response.clicked() {
			if let Some(position) = response.interact_pointer_pos() {
				if rect.contains(position) {
					if self.mode == Mode::RequiredRopes {
						if let Some(index) = self.anchor_at(position, rect) {
							self.handle_anchor_click(index);
						}
					} else {
						self.handle_canvas_click(
							Point::from_screen(position, rect),
						);
					}
				}
			}
		}

		let painter = ui.painter_at(rect);

		self.draw_obstacles(&painter, rect);
		self.draw_ropes(&painter, rect);
		self.draw_points(&painter, rect);

		let mouse = response
			.hover_pos()
			.filter(|position| rect.contains(*position));

		self.draw_pending_segment(&painter, rect, mouse);
		self.draw_pending_rope(&painter, rect, mouse);
	}

	fn anchor_at(
		&self,
		position: egui::Pos2,
		rect: egui::Rect,
	) -> Option<usize> {
		self.points
			.iter()
			.enumerate()
			.map(|(index, point)| {
				let distance = point.to_screen(rect).distance(position);
				(index, distance)
			})
			.filter(|&(_, distance)| distance <= 12.0)
			.min_by(|a, b| a.1.total_cmp(&b.1))
			.map(|(index, _)| index)
	}

	fn handle_anchor_click(&mut self, index: usize) {
		let Some(start) = self.pending_anchor.take() else {
			self.pending_anchor = Some(index);
			return;
		};

		// Cliccare di nuovo sul primo ancoraggio annulla la selezione.
		if start == index {
			return;
		}

		let rope = (start.min(index), start.max(index));

		if let Some(position) = self.required_ropes
			.iter()
			.position(|&existing| existing == rope)
		{
			self.required_ropes.remove(position);
		} else {
			self.required_ropes.push(rope);
		}

		self.invalidate_results();
	}

	fn handle_canvas_click(&mut self, point: Point) {
		match self.mode {
			Mode::Points => {
				if self.points.len() < 26 {
					self.points.push(point);
					self.invalidate_results();
				}
			}
			Mode::Obstacles => {
				if let Some(start) = self.pending_start {
					let dx = point.x - start.x;
					let dy = point.y - start.y;

					if dx * dx + dy * dy > 1e-10 {
						self.obstacles.push(Segment {
							start,
							end: point,
						});

						self.pending_start = None;
						self.invalidate_results();
					}
				} else {
					self.pending_start = Some(point);
				}
			}
			Mode::RequiredRopes => {
				// Gestita da handle_anchor_click.
			}
		}
	}

	fn draw_points(&self, painter: &egui::Painter, rect: egui::Rect) {
		for (index, point) in self.points.iter().enumerate() {
			let position = point.to_screen(rect);
			let letter = (b'A' + index as u8) as char;
			painter.circle_filled(position, 5.0, egui::Color32::LIGHT_GREEN);
			Self::draw_label(painter, position, letter);
		}
	}

	fn draw_label(painter: &egui::Painter, position: egui::Pos2, letter: char) {
		let label_position = position + egui::vec2(9.0, -9.0);
		let font = egui::FontId::proportional(22.0);
		let text = letter.to_string();

		painter.text(
			label_position + egui::vec2(1.0, 1.0),
			egui::Align2::LEFT_BOTTOM,
			&text,
			font.clone(),
			egui::Color32::BLACK,
		);

		painter.text(
			label_position,
			egui::Align2::LEFT_BOTTOM,
			&text,
			font,
			egui::Color32::WHITE,
		);
	}

	fn draw_obstacles(&self, painter: &egui::Painter, rect: egui::Rect) {
		let color = egui::Color32::LIGHT_RED;

		for segment in &self.obstacles {
			let start = segment.start.to_screen(rect);
			let end = segment.end.to_screen(rect);

			painter.line_segment([start, end], egui::Stroke::new(3.0, color));

			painter.circle_filled(start, 4.0, color);
			painter.circle_filled(end, 4.0, color);
		}
	}

	fn draw_pending_segment(
		&self,
		painter: &egui::Painter,
		rect: egui::Rect,
		mouse: Option<egui::Pos2>,
	) {
		let Some(start) = self.pending_start else {
			return;
		};

		let start = start.to_screen(rect);
		let color = egui::Color32::YELLOW;

		painter.circle_filled(start, 5.0, color);

		if let Some(mouse) = mouse {
			painter.line_segment([start, mouse], egui::Stroke::new(2.0, color));
		}
	}

	fn draw_rope(
		&self,
		painter: &egui::Painter,
		rect: egui::Rect,
		rope: Rope,
		color: egui::Color32,
	) {
		let start = self.points[rope.0].to_screen(rect);
		let end = self.points[rope.1].to_screen(rect);

		painter.line_segment(
			[start, end],
			egui::Stroke::new(3.0, color),
		);
	}

	fn draw_ropes(&self, painter: &egui::Painter, rect: egui::Rect) {
		// Corde aggiunte dal solver.
		if let Some(solution) = self.solutions.get(self.solution_index) {
			for &rope in solution {
				if !self.required_ropes.contains(&rope) {
					self.draw_rope(
						painter,
						rect,
						rope,
						egui::Color32::LIGHT_BLUE,
					);
				}
			}
		}

		// Corde fissate manualmente, visibili anche prima del calcolo.
		for &rope in &self.required_ropes {
			self.draw_rope(
				painter,
				rect,
				rope,
				egui::Color32::LIGHT_GREEN,
			);
		}
	}

	fn draw_pending_rope(
		&self,
		painter: &egui::Painter,
		rect: egui::Rect,
		mouse: Option<egui::Pos2>,
	) {
		let Some(index) = self.pending_anchor else {
			return;
		};

		let start = self.points[index].to_screen(rect);

		painter.circle_stroke(
			start,
			9.0,
			egui::Stroke::new(2.0, egui::Color32::YELLOW),
		);

		if let Some(mouse) = mouse {
			// L'anteprima si aggancia al punto quando gli passi vicino.
			let end = self.anchor_at(mouse, rect)
				.map(|index| self.points[index].to_screen(rect))
				.unwrap_or(mouse);

			painter.line_segment(
				[start, end],
				egui::Stroke::new(2.0, egui::Color32::YELLOW),
			);
		}
	}

	fn export_configuration(&self) {
		let configuration = Configuration {
			points: self.points.clone(),
			obstacles: self.obstacles.clone(),
		};

		match serde_json::to_string_pretty(&configuration) {
			Ok(json) => println!("{json}"),
			Err(error) => eprintln!("Errore di esportazione: {error}"),
		}
	}
}

impl eframe::App for PappagalloApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ui, |ui| {
			self.handle_keyboard(ui);
			self.show_toolbar(ui);
			self.show_canvas(ui);
		});
	}
}