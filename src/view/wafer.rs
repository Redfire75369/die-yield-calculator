/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashSet;

use iced::{Color, Length, Rectangle, Renderer, Size, Theme, Vector};
use iced::mouse::Cursor;
use iced::widget::Canvas;
use iced::widget::canvas::{Cache, Geometry, Path, Program, Stroke, Text};

use crate::die::DieType;
use crate::util::random;
use crate::view::calculator::Message;
use crate::wafer::{Shape, Wafer};

#[derive(Default)]
pub struct WaferViewState {
	cache: Cache,
}

impl WaferViewState {
	pub fn request_redraw(&mut self) {
		self.cache.clear()
	}

	pub fn view<'a>(&'a self, wafer: &'a Wafer) -> Canvas<WaferView<'a>, Message> {
		Canvas::new(WaferView { state: self, wafer }).width(Length::Fill).height(Length::Fill)
	}
}

pub struct WaferView<'a> {
	state: &'a WaferViewState,
	wafer: &'a Wafer,
}

impl<'a> Program<Message> for WaferView<'a> {
	type State = ();

	fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
		let wafer = self.state.cache.draw(renderer, bounds.size(), |frame| {
			let dimension = frame.width().min(frame.height()) * 0.8;
			let center = frame.center() - Vector::new(0.0, dimension * 0.05);
			let top_left = center - Vector::new(dimension / 2.0, dimension / 2.0);
			let mut frame_top_left = top_left;

			let outer_stroke = Stroke::default().with_color(Color::from_rgb8(0, 200, 0)).with_width(1.5);
			let inner_stroke = Stroke::default().with_color(Color::from_rgb8(200, 0, 0)).with_width(1.5);

			frame.stroke(
				&Path::rectangle(
					top_left - Vector::new(dimension * 0.05, dimension * 0.05),
					Size::new(dimension * 1.1, dimension * 1.225),
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

			let edge_loss = self.wafer.edge_loss;
			let scale = match self.wafer.shape {
				Shape::Wafer(diameter) => {
					let diameter = diameter.diameter();
					let inner_diameter = diameter - 2.0 * edge_loss;
					let scale = dimension / diameter;

					frame.stroke(&Path::circle(center, dimension / 2.0), outer_stroke);
					frame.stroke(&Path::circle(center, (dimension / 2.0) * (inner_diameter / diameter)), inner_stroke);

					scale
				}
				Shape::Panel(panel) => {
					let (width, height) = panel.dimensions();
					let (inner_width, inner_height) = (width - 2.0 * edge_loss, height - 2.0 * edge_loss);
					let scale = dimension / width.max(height);

					frame_top_left = center - Vector::new(width, height) * (scale / 2.0);

					frame.stroke(&Path::rectangle(frame_top_left, Size::new(width * scale, height * scale)), outer_stroke);
					frame.stroke(
						&Path::rectangle(
							frame_top_left + Vector::new(edge_loss, edge_loss) * scale,
							Size::new(inner_width * scale, inner_height * scale),
						),
						inner_stroke,
					);

					scale
				}
			};

			let mut die_types = (0, 0, 0); // Complete, Partial, Wasted

			let die_size = Size::new(self.wafer.die.width() * scale, self.wafer.die.height() * scale);
			let die_grid = self.wafer.get_dies();
			for die_column in &die_grid {
				for (die_type, die_coord) in die_column {
					let tl = frame_top_left + Vector::new(die_coord.x, die_coord.y) * scale;

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
						DieType::None => {}
					}
				}
			}

			let die_yield = self.wafer.yield_model.wafer_yield(&self.wafer);
			let bad_dies = ((die_types.0 as f32) * (1.0 - die_yield)).round() as usize;
			let mut bad = HashSet::with_capacity(bad_dies);

			while bad.len() < bad_dies {
				let x = random(0, die_grid.len() - 1);
				let y = random(0, die_grid[0].len() - 1);

				let (die_type, die_coord) = die_grid[x][y];
				if die_type == DieType::Complete && !bad.contains(&(x, y)) {
					let tl = frame_top_left + Vector::new(die_coord.x, die_coord.y) * scale;
					let center = Rectangle::new(tl, die_size).center();
					frame.fill_rectangle(tl, die_size, Color::from_rgb8(70, 70, 70));
					frame.fill(
						&Path::circle(center, self.wafer.die.width().min(self.wafer.die.height()) * scale / 5.0),
						Color::from_rgb8(180, 180, 180),
					);

					bad.insert((x, y));
				}
			}

			frame.fill_text(Text {
				content: format!("Good Dies {}", die_types.0 - bad_dies),
				position: top_left + Vector::new(dimension * 0.0125, dimension * 1.05),
				..Text::default()
			});
			frame.fill_text(Text {
				content: format!("Wasted Dies {}", die_types.2),
				position: top_left + Vector::new(dimension * 0.0125, dimension * 1.115),
				..Text::default()
			});

			frame.fill_text(Text {
				content: format!("Defective Dies {}", bad_dies),
				position: top_left + Vector::new(dimension * 0.3625, dimension * 1.05),
				..Text::default()
			});
			frame.fill_text(Text {
				content: format!("Partial Dies {}", die_types.1),
				position: top_left + Vector::new(dimension * 0.3625, dimension * 1.115),
				..Text::default()
			});

			frame.fill_text(Text {
				content: format!("Maximum Dies {}", die_types.0),
				position: top_left + Vector::new(dimension * 0.7125, dimension * 1.05),
				..Text::default()
			});
			frame.fill_text(Text {
				content: format!("Fab Yield {:.2}%", die_yield * 100.0),
				position: top_left + Vector::new(dimension * 0.7125, dimension * 1.115),
				..Text::default()
			});
		});

		vec![wafer]
	}
}
