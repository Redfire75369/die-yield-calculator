/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub const RETICLE_LONG: f32 = 33.0;
pub const RETICLE_SHORT: f32 = 26.0;

pub fn max_other_dimension(reticle_limit: bool, die_square: bool, diameter: f32, dimension: f32) -> f32 {
	if !reticle_limit {
		return diameter;
	}
	if die_square {
		return RETICLE_SHORT;
	}

	if dimension > RETICLE_SHORT {
		RETICLE_SHORT
	} else {
		RETICLE_LONG
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DieType {
	Complete = 0,
	Partial = 1,
	Wasted = 2,
	None = 3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Die {
	pub width: f32,
	pub height: f32,
}

impl Die {
	pub fn area(&self) -> f32 {
		self.width * self.height
	}
}

impl Default for Die {
	fn default() -> Die {
		Die { width: 8.0, height: 8.0 }
	}
}
