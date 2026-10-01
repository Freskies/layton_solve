use serde::{Deserialize, Serialize};
use crate::pappagallo::point::Point;
use crate::pappagallo::segment::Segment;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
	pub(crate) points: Vec<Point>,
	pub(crate) obstacles: Vec<Segment>,
}