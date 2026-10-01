use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {
	pub fn from_screen(position: egui::Pos2, rect: egui::Rect) -> Self {
		Self {
			x: (position.x - rect.left()) / rect.width(),
			y: (position.y - rect.top()) / rect.height(),
		}
	}

	pub fn to_screen(self, rect: egui::Rect) -> egui::Pos2 {
		egui::pos2(
			rect.left() + self.x * rect.width(),
			rect.top() + self.y * rect.height(),
		)
	}
}