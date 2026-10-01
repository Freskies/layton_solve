use serde::{Deserialize, Serialize};
use crate::pappagallo::point::Point;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Segment {
	pub(crate) start: Point,
	pub(crate) end: Point,
}