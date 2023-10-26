/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub use shape::*;
pub use yield_model::YieldModel;

use crate::die::{Die, DieType};
use crate::util::{Coordinate, Rectangle};

mod shape;
mod yield_model;

pub const MAXIMUM_SCRIBE_WIDTH: f32 = 10.0;
pub const MINIMUM_DIE_DIMENSION: f32 = 0.01;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wafer {
	pub critical_area: f32,
	pub shape: Shape,
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
		Die::Rectangle {
			width: self.die.width() + self.scribe_lanes.0,
			height: self.die.height() + self.scribe_lanes.1,
		}
	}

	pub fn clamp_critical_area(&mut self) {
		self.critical_area = self.critical_area.min(self.die.area());
	}

	fn die_type(&self, die_coord: &Coordinate) -> DieType {
		let die = Rectangle::new(die_coord, self.die.width(), self.die.height());

		let (within, within_inner) = match self.shape {
			Shape::Wafer(diameter) => {
				let radius = diameter.diameter() / 2.0;
				let center = Coordinate { x: radius, y: radius };
				(die.within_radius(&center, radius), die.within_radius(&center, radius - self.edge_loss))
			}
			Shape::Panel(panel) => {
				let (width, height) = panel.dimensions();
				let edge_loss = self.edge_loss;
				let outer = Rectangle::new(&Coordinate { x: 0.0, y: 0.0 }, width, height);
				let inner = Rectangle::new(
					&Coordinate { x: edge_loss, y: edge_loss },
					width - 2.0 * edge_loss,
					height - 2.0 * edge_loss,
				);
				(die.within_rectangle(&outer), die.within_rectangle(&inner))
			}
		};

		if within_inner.0 {
			DieType::Complete
		} else if within.0 {
			if within_inner.1 {
				DieType::Partial
			} else {
				DieType::Wasted
			}
		} else {
			DieType::None
		}
	}

	pub fn get_dies(&self) -> Vec<Vec<(DieType, Coordinate)>> {
		let horizontal = (self.shape.max_width() / self.reticle().width()).floor() as u32;
		let vertical = (self.shape.max_height() / self.reticle().height()).floor() as u32;

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
							-0.5 * self.die.width()
						};
						let y_offset = if self.centered == vertical_even {
							0.5 * self.scribe_lanes.1
						} else {
							-0.5 * self.die.height()
						};

						let coord = Coordinate {
							x: self.shape.max_width() / 2.0
								+ ((x as f32) - 0.5 * (horizontal as f32)).floor() * reticle.width()
								+ x_offset + self.translation.0,
							y: self.shape.max_height() / 2.0
								+ ((y as f32) - 0.5 * (vertical as f32)).floor() * reticle.height()
								+ y_offset + self.translation.1,
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
			shape: Shape::default(),
			edge_loss: 3.0,
			defect_rate: 0.1,

			scribe_lanes: (0.25, 0.25),
			translation: (0.0, 0.0),
			centered: false,

			die: Die::default(),
			yield_model: YieldModel::default(),
		}
	}
}
