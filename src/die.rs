/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::util::min_if;
use crate::wafer::{MINIMUM_DIE_DIMENSION, Shape};

pub const RETICLE_LONG: f32 = 33.0;
pub const RETICLE_SHORT: f32 = 26.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DieType {
	Complete,
	Partial,
	Wasted,
	None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Die {
	Rectangle { width: f32, height: f32 },
	Square(f32),
}

impl Die {
	pub fn width(self) -> f32 {
		match self {
			Die::Rectangle { width, .. } => width,
			Die::Square(width) => width,
		}
	}

	pub fn height(self) -> f32 {
		match self {
			Die::Rectangle { height, .. } => height,
			Die::Square(width) => width,
		}
	}

	pub fn area(self) -> f32 {
		self.width() * self.height()
	}

	pub fn width_bounds(self, reticle_limit: bool, shape: Shape) -> (f32, f32) {
		if !reticle_limit {
			return (MINIMUM_DIE_DIMENSION, shape.max_width());
		}

		let max = match self {
			Die::Rectangle { height, .. } => {
				if height > RETICLE_SHORT {
					RETICLE_SHORT
				} else {
					RETICLE_LONG
				}
			}
			Die::Square(_) => RETICLE_SHORT,
		};
		(MINIMUM_DIE_DIMENSION, max)
	}

	pub fn height_bounds(self, reticle_limit: bool, shape: Shape) -> (f32, f32) {
		let min = match self {
			Die::Rectangle { .. } => MINIMUM_DIE_DIMENSION,
			Die::Square(width) => width,
		};
		if !reticle_limit {
			return (min, shape.max_width());
		}

		let max = match self {
			Die::Rectangle { width, .. } => {
				if width > RETICLE_SHORT {
					RETICLE_SHORT
				} else {
					RETICLE_LONG
				}
			}
			Die::Square(width) => width,
		};
		(min, max)
	}

	pub fn new_width(self, width: f32) -> Die {
		let width = width.max(MINIMUM_DIE_DIMENSION);
		match self {
			Die::Rectangle { height, .. } => Die::Rectangle { width, height },
			Die::Square(_) => Die::Square(width),
		}
	}

	pub fn new_height(self, height: f32) -> Die {
		let height = height.max(MINIMUM_DIE_DIMENSION);
		match self {
			Die::Rectangle { width, .. } => Die::Rectangle { width, height },
			square => square,
		}
	}

	pub fn clamp_reticle(self) -> Die {
		match self {
			Die::Rectangle { width, height } => {
				if width > RETICLE_LONG {
					Die::Rectangle {
						width: RETICLE_LONG,
						height: height.min(RETICLE_SHORT),
					}
				} else if height > RETICLE_LONG {
					Die::Rectangle {
						width: width.min(RETICLE_SHORT),
						height: RETICLE_LONG,
					}
				} else if width > RETICLE_SHORT {
					Die::Rectangle {
						width,
						height: height.min(RETICLE_SHORT),
					}
				} else {
					Die::Rectangle { width, height }
				}
			}
			Die::Square(width) => Die::Square(width.min(RETICLE_SHORT)),
		}
	}

	pub fn rectangle(self) -> Die {
		match self {
			Die::Square(width) => Die::Rectangle { width, height: width },
			rectangle => rectangle,
		}
	}

	pub fn square(self, reticle_limit: bool) -> Die {
		match self {
			Die::Rectangle { width, .. } => {
				let width = min_if(reticle_limit, width, RETICLE_SHORT);
				Die::Square(width)
			}
			square => square,
		}
	}
}

impl Default for Die {
	fn default() -> Die {
		Die::Rectangle {
			width: 8.0,
			height: 8.0,
		}
	}
}
