/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Diameter {
	Two,
	Three,
	Four,
	Five,
	Six,
	Eight,
	#[default]
	Twelve,
	Eighteen,
}

impl Diameter {
	pub const ALL: &'static [Diameter] = &[
		Diameter::Two,
		Diameter::Three,
		Diameter::Four,
		Diameter::Five,
		Diameter::Six,
		Diameter::Eight,
		Diameter::Twelve,
		Diameter::Eighteen,
	];

	pub fn diameter(self) -> f32 {
		match self {
			Diameter::Two => 51.0,
			Diameter::Three => 76.0,
			Diameter::Four => 100.0,
			Diameter::Five => 125.0,
			Diameter::Six => 150.0,
			Diameter::Eight => 200.0,
			Diameter::Twelve => 300.0,
			Diameter::Eighteen => 450.0,
		}
	}
}

impl Display for Diameter {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Diameter::Two => f.write_str("51 mm (2 in)"),
			Diameter::Three => f.write_str("76 mm (3 in)"),
			Diameter::Four => f.write_str("100 mm (4 in)"),
			Diameter::Five => f.write_str("100 mm (5 in)"),
			Diameter::Six => f.write_str("150 mm (6 in)"),
			Diameter::Eight => f.write_str("200 mm (8 in)"),
			Diameter::Twelve => f.write_str("300 mm (12 in)"),
			Diameter::Eighteen => f.write_str("450 mm (18 in)"),
		}
	}
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Panel {
	#[default]
	TwelveByTwelve,
	TwelveByEighteen,
	EighteenByEighteen,
	EighteenByTwentyFour,
	TwentyOneByTwentyOne,
	TwentyFourByTwentyFour,
}

impl Panel {
	pub const ALL: &'static [Panel] = &[
		Panel::TwelveByTwelve,
		Panel::TwelveByEighteen,
		Panel::EighteenByEighteen,
		Panel::EighteenByTwentyFour,
		Panel::TwentyOneByTwentyOne,
		Panel::TwentyFourByTwentyFour,
	];

	pub fn dimensions(self) -> (f32, f32) {
		match self {
			Panel::TwelveByTwelve => (300.0, 300.0),
			Panel::TwelveByEighteen => (305.0, 457.0),
			Panel::EighteenByEighteen => (457.0, 457.0),
			Panel::EighteenByTwentyFour => (457.0, 600.0),
			Panel::TwentyOneByTwentyOne => (510.0, 515.0),
			Panel::TwentyFourByTwentyFour => (600.0, 600.0),
		}
	}
}

impl Display for Panel {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Panel::TwelveByTwelve => f.write_str("300 mm (12 in)"),
			Panel::TwelveByEighteen => f.write_str("305 × 457 mm² (12 × 18 in²)"),
			Panel::EighteenByEighteen => f.write_str("457 mm² (18 in)"),
			Panel::EighteenByTwentyFour => f.write_str("457 × 600 mm² (18 × 24 in²)"),
			Panel::TwentyOneByTwentyOne => f.write_str("510 × 515 mm² (21 in)"),
			Panel::TwentyFourByTwentyFour => f.write_str("600 mm (24 in)"),
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Shape {
	Wafer(Diameter),
	Panel(Panel),
}

impl Shape {
	pub fn max_width(self) -> f32 {
		match self {
			Shape::Wafer(diameter) => diameter.diameter(),
			Shape::Panel(panel) => panel.dimensions().0,
		}
	}

	pub fn max_height(self) -> f32 {
		match self {
			Shape::Wafer(diameter) => diameter.diameter(),
			Shape::Panel(panel) => panel.dimensions().1,
		}
	}
}

impl Default for Shape {
	fn default() -> Shape {
		Shape::Wafer(Diameter::default())
	}
}

impl From<Diameter> for Shape {
	fn from(diameter: Diameter) -> Self {
		Shape::Wafer(diameter)
	}
}

impl From<Panel> for Shape {
	fn from(panel: Panel) -> Shape {
		Shape::Panel(panel)
	}
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ShapeOption {
	#[default]
	Wafer,
	Panel,
}

impl ShapeOption {
	pub const ALL: &'static [ShapeOption] = &[ShapeOption::Wafer, ShapeOption::Panel];
}

impl From<Shape> for ShapeOption {
	fn from(shape: Shape) -> ShapeOption {
		match shape {
			Shape::Wafer(_) => ShapeOption::Wafer,
			Shape::Panel(_) => ShapeOption::Panel,
		}
	}
}

impl Display for ShapeOption {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			ShapeOption::Wafer => f.write_str("Wafer"),
			ShapeOption::Panel => f.write_str("Panel"),
		}
	}
}
