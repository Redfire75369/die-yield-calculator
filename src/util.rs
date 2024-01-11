/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
	pub x: f32,
	pub y: f32,
}

impl Coordinate {
	pub fn distance(&self, coord: &Coordinate) -> f32 {
		(coord.x - self.x).powi(2) + (coord.y - self.y).powi(2)
	}

	pub fn within_radius(&self, center: &Coordinate, radius: f32) -> bool {
		center.distance(self) <= radius.powi(2)
	}

	pub fn within_rectangle(&self, rectangle: &Rectangle) -> bool {
		(rectangle.bl.x..=rectangle.br.x).contains(&self.x) && (rectangle.bl.y..=rectangle.tl.y).contains(&self.y)
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
	bl: Coordinate,
	br: Coordinate,
	tl: Coordinate,
	tr: Coordinate,
}

impl Rectangle {
	pub fn new(bl: Coordinate, width: f32, height: f32) -> Rectangle {
		Rectangle {
			bl,
			br: Coordinate {
				x: bl.x + width,
				y: bl.y,
			},
			tl: Coordinate {
				x: bl.x,
				y: bl.y + height,
			},
			tr: Coordinate {
				x: bl.x + width,
				y: bl.y + height,
			},
		}
	}

	pub fn within_radius(&self, center: &Coordinate, radius: f32) -> (bool, bool) {
		let bl = self.bl.within_radius(center, radius);
		let br = self.br.within_radius(center, radius);
		let tl = self.tl.within_radius(center, radius);
		let tr = self.tr.within_radius(center, radius);
		(bl && br && tl && tr, bl || br || tl || tr)
	}

	pub fn within_rectangle(&self, other: &Rectangle) -> (bool, bool) {
		let bl = self.bl.within_rectangle(other);
		let br = self.br.within_rectangle(other);
		let tl = self.tl.within_rectangle(other);
		let tr = self.tr.within_rectangle(other);
		(bl && br && tl && tr, bl || br || tl || tr)
	}
}

pub fn random(min: usize, max: usize) -> usize {
	rand::thread_rng().gen_range(min..=max)
}

pub fn min_if(cond: bool, a: f32, b: f32) -> f32 {
	if cond {
		a.min(b)
	} else {
		a
	}
}
