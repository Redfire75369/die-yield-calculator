/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Color, Length, Rectangle, Size, Theme, Vector};
use iced::widget::Canvas;
use iced::widget::canvas::{Cache, Cursor, Geometry, Path, Program, Stroke};

use crate::die::DieType;
use crate::util::random;
use crate::view::calculator::Message;
use crate::wafer::{Wafer, YieldModel};

#[derive(Default)]
pub struct WaferViewState {
	cache: Cache,
}

impl WaferViewState {
	pub fn request_redraw(&mut self) {
		self.cache.clear()
	}

	pub fn view<'a>(&'a self, wafer: &'a Wafer) -> Canvas<Message, Theme, WaferView<'a>> {
		Canvas::new(WaferView { state: self, wafer }).width(Length::Fill).height(Length::Fill)
	}
}

pub struct WaferView<'a> {
	state: &'a WaferViewState,
	wafer: &'a Wafer,
}

impl<'a> Program<Message> for WaferView<'a> {
	type State = ();

	fn draw(&self, _state: &(), _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
		let wafer = self.state.cache.draw(bounds.size(), |frame| {
			let center = frame.center();
			let dimension = frame.width().min(frame.height()) * 0.8;
			let scale = dimension / self.wafer.diameter;
			let top_left = center - Vector::new(dimension / 2.0, dimension / 2.0);

			frame.stroke(
				&Path::rectangle(
					top_left - Vector::new(dimension * 0.05, dimension * 0.1),
					Size::new(dimension * 1.1, dimension * 1.18),
				),
				Stroke::default().with_color(Color::from_rgb8(120, 120, 120)),
			);
			frame.stroke(
				&Path::rectangle(
					top_left - Vector::new(dimension * 0.025, dimension * 0.025),
					Size::new(dimension * 1.05, dimension * 1.05),
				),
				Stroke::default().with_color(Color::from_rgb8(170, 170, 170)),
			);
			frame.stroke(
				&Path::circle(center, dimension / 2.0),
				Stroke::default().with_color(Color::from_rgb8(200, 0, 0)),
			);
			frame.stroke(
				&Path::circle(center, (dimension / 2.0) * (self.wafer.inner_diameter() / self.wafer.diameter)),
				Stroke::default().with_color(Color::from_rgb8(0, 200, 0)),
			);

			let mut die_types = (0, 0, 0, 0);

			let die_size = Size::new(self.wafer.die.width * scale, self.wafer.die.height * scale);
			let die_grid = self.wafer.get_dies();
			for die_column in &die_grid {
				for (die_type, die_coord) in die_column {
					let tl = top_left + Vector::new(die_coord.x, die_coord.y) * scale;

					match die_type {
						DieType::Complete => {
							frame.fill_rectangle(tl, die_size, Color::from_rgba8(60, 180, 60, 0.8));
							die_types.0 += 1;
						}
						DieType::Partial => {
							frame.fill_rectangle(tl, die_size, Color::from_rgba8(200, 200, 0, 0.8));
							die_types.1 += 1;
						}
						DieType::Wasted => {
							frame.fill_rectangle(tl, die_size, Color::from_rgba8(180, 60, 60, 0.8));
							die_types.2 += 1;
						}
						DieType::None => die_types.3 += 1,
					}
				}
			}

			let die_yield = match self.wafer.yield_model {
				YieldModel::Poisson => self.wafer.yield_poisson(),
				YieldModel::Murphy => self.wafer.yield_murphy(),
			};
			let bad_dies = ((die_types.0 as f32) * (1.0 - die_yield)) as u32;

			let mut i = 0;

			while i < bad_dies {
				let x = random(0, (die_grid.len() - 1) as u16) as usize;
				let y = random(0, (die_grid[0].len() - 1) as u16) as usize;

				let (die_type, die_coord) = die_grid[x][y];
				if die_type == DieType::Complete {
					let tl = top_left + Vector::new(die_coord.x, die_coord.y) * scale;
					let center = Rectangle::new(tl, die_size).center();
					frame.fill_rectangle(tl, die_size, Color::from_rgb8(70, 70, 70));
					frame.fill(
						&Path::circle(center, self.wafer.die.width.min(self.wafer.die.height) * scale / 5.0),
						Color::from_rgb8(180, 180, 180),
					);
					i += 1;
				}
			}
		});

		vec![wafer]
	}
}
