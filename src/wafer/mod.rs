/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::f32::consts::PI;

pub use diameter::Diameter;
pub use yield_model::YieldModel;

use crate::die::{Die, DieType};
use crate::util::{Coordinate, Rectangle};

mod diameter;
mod yield_model;

type Grid<T> = Vec<Vec<T>>;

pub const MINIMUM_SCRIBE_WIDTH: f32 = 0.001;
pub const MAXIMUM_SCRIBE_WIDTH: f32 = 10.0;

pub const MINIMUM_DIE_DIMENSION: f32 = 0.01;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wafer {
	pub critical_area: f32,
	pub diameter: f32,
	pub edge_loss: f32,
	pub defect_rate: f32,

	pub scribe_lanes: (f32, f32),
	pub translation: (f32, f32),
	pub centered: bool,

	pub die: Die,
	pub yield_model: YieldModel,
}

impl Wafer {
	fn reticle(&self) -> Die {
		Die {
			width: self.die.width + self.scribe_lanes.0,
			height: self.die.height + self.scribe_lanes.1,
		}
	}

	pub fn inner_diameter(&self) -> f32 {
		self.diameter - 2.0 * self.edge_loss
	}

	pub fn fix_critical_area(&mut self) {
		self.critical_area = self.critical_area.min(self.die.area());
	}

	#[allow(dead_code)]
	fn max_dies(&self) -> u32 {
		((PI * (self.inner_diameter() / 2.0).powi(2)) / self.reticle().area() - (PI * self.inner_diameter()) / (2.0 * self.reticle().area()).sqrt())
			as u32
	}

	fn die_type(&self, die_coord: &Coordinate) -> DieType {
		let center = Coordinate {
			x: self.diameter / 2.0,
			y: self.diameter / 2.0,
		};
		let die = Rectangle::new(die_coord, self.die.width, self.die.height);

		let within = die.within_radius(&center, self.diameter / 2.0);
		let within_inner = die.within_radius(&center, self.inner_diameter() / 2.0);

		if within_inner.0 && within_inner.1 && within_inner.2 && within_inner.3 {
			DieType::Complete
		} else if within.0 && within.1 && within.2 && within.3 {
			if within_inner.0 || within_inner.1 || within_inner.2 || within_inner.3 {
				DieType::Partial
			} else {
				DieType::Wasted
			}
		} else {
			DieType::None
		}
	}

	pub fn get_dies(&self) -> Grid<(DieType, Coordinate)> {
		let horizontal: u32 = (self.diameter / self.reticle().width).floor() as u32;
		let vertical: u32 = (self.diameter / self.reticle().height).floor() as u32;

		let horizontal_even = horizontal % 2 == 0;
		let vertical_even = vertical % 2 == 0;
		let reticle = self.reticle();

		(0..=horizontal)
			.map(|x| {
				(0..=vertical)
					.map(|y| {
						let x_offset = if self.centered == horizontal_even {
							0.5 * self.scribe_lanes.0
						} else {
							-0.5 * self.die.width
						};
						let y_offset = if self.centered == vertical_even {
							0.5 * self.scribe_lanes.1
						} else {
							-0.5 * self.die.height
						};

						let coord = Coordinate {
							x: self.diameter / 2.0 + ((x as f32) - 0.5 * (horizontal as f32)).floor() * reticle.width + x_offset + self.translation.0,
							y: self.diameter / 2.0 + ((y as f32) - 0.5 * (vertical as f32)).floor() * reticle.height + y_offset + self.translation.1,
						};
						(self.die_type(&coord), coord)
					})
					.collect()
			})
			.collect()
	}
}

impl Default for Wafer {
	fn default() -> Wafer {
		Wafer {
			critical_area: Die::default().area(),
			diameter: 300.0,
			edge_loss: 3.0,
			defect_rate: 0.10,
			scribe_lanes: (0.2, 0.2),
			translation: (0.0, 0.0),
			centered: false,
			die: Die::default(),
			yield_model: YieldModel::Murphy,
		}
	}
}
