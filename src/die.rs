/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

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
	pub fn average_dimension(&self) -> f32 {
		f32::sqrt(self.area())
	}

	pub fn area(&self) -> f32 {
		self.width * self.height
	}
}

impl Default for Die {
	fn default() -> Die {
		Die { width: 10.0, height: 10.0 }
	}
}
