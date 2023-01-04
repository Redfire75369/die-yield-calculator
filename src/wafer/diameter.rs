/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum Diameter {
	TWO = 51,
	THREE = 76,
	FOUR = 100,
	SIX = 150,
	EIGHT = 200,
	TWELVE = 300,
	EIGHTEEN = 450,
}

impl Diameter {
	pub const ALL: &'static [Diameter] = &[
		Diameter::TWO,
		Diameter::THREE,
		Diameter::FOUR,
		Diameter::SIX,
		Diameter::EIGHTEEN,
		Diameter::TWELVE,
		Diameter::EIGHTEEN,
	];
}

impl Display for Diameter {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Diameter::TWO => f.write_str("51 mm (2 in)"),
			Diameter::THREE => f.write_str("76 mm (3 in)"),
			Diameter::FOUR => f.write_str("100 mm (4 in)"),
			Diameter::SIX => f.write_str("150 mm (6 in)"),
			Diameter::EIGHT => f.write_str("200 mm (8 in)"),
			Diameter::TWELVE => f.write_str("300 mm (12 in)"),
			Diameter::EIGHTEEN => f.write_str("450 mm (18 in)"),
		}
	}
}
