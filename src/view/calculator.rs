/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Alignment, Color, Element, Length, Sandbox, Theme};
use iced::theme::Palette;
use iced::widget::{column, container, row};

use crate::die::{RETICLE_LONG, RETICLE_SHORT};
use crate::view::components::{
	critical_area, defect_rate, diameter, die_centering, die_size, edge_loss, scribe_lines, translation, yield_model,
};
use crate::view::wafer::WaferViewState;
use crate::wafer::{Diameter, MAXIMUM_SCRIBE_WIDTH, MINIMUM_DIE_DIMENSION, MINIMUM_SCRIBE_WIDTH, Wafer, YieldModel};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Component {
	DieWidth,
	DieHeight,
	CriticalArea,
	Diameter,
	DefectRate,
	EdgeLoss,
	ScribeHorizontal,
	ScribeVertical,
	TranslateHorizontal,
	TranslateVertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Message {
	Center(bool),
	Diameter(Diameter),
	Checkbox(Component, bool),
	NumberInput(Component, f32),
	YieldModel(YieldModel),
	None,
}

impl Message {
	pub fn checkbox(component: Component) -> impl Fn(bool) -> Message + Copy {
		move |boolean| Message::Checkbox(component, boolean)
	}

	pub fn number_input(component: Component) -> impl Fn(f32) -> Message + Copy {
		move |float| Message::NumberInput(component, float)
	}
}

pub struct Calculator {
	wafer: Wafer,
	diameter: Diameter,

	die_square: bool,
	simple_critical_area: bool,
	scribe_equal: bool,

	wafer_view: WaferViewState,
}

impl Sandbox for Calculator {
	type Message = Message;

	fn new() -> Self {
		let wafer = Wafer::default();
		Calculator {
			wafer,
			diameter: Diameter::TWELVE,

			die_square: false,
			simple_critical_area: true,
			scribe_equal: false,

			wafer_view: WaferViewState::default(),
		}
	}

	fn title(&self) -> String {
		String::from("Die Yield Calculator")
	}

	fn update(&mut self, message: Message) {
		match message {
			Message::Center(b) => self.wafer.centered = b,
			Message::Checkbox(c, b) => match c {
				Component::DieWidth => {
					self.die_square = b;
					self.wafer.die.height = self.wafer.die.width.min(RETICLE_SHORT);
				}
				Component::CriticalArea => {
					self.simple_critical_area = b;
					self.wafer.critical_area = self.wafer.die.area();
				}
				Component::ScribeHorizontal => {
					self.scribe_equal = b;
					self.wafer.scribe_lanes.1 = self.wafer.scribe_lanes.0;
				}
				_ => {}
			},
			Message::Diameter(d) => {
				self.wafer.diameter = d as u16 as f32;
				self.diameter = d;
			}
			Message::NumberInput(c, mut f) => match c {
				Component::DieWidth => {
					f = f.max(MINIMUM_DIE_DIMENSION);
					if self.die_square || self.wafer.die.height >= RETICLE_LONG {
						f = f.min(RETICLE_SHORT);
					}
					if self.die_square {
						self.wafer.die.height = f;
					}
					self.wafer.die.width = f;

					if self.simple_critical_area {
						self.wafer.critical_area = self.wafer.die.area();
					} else {
						self.wafer.fix_critical_area();
					}
				}
				Component::DieHeight => {
					if !self.die_square {
						f = f.max(MINIMUM_DIE_DIMENSION);
						if self.wafer.die.width >= RETICLE_LONG {
							f = f.min(RETICLE_SHORT);
						}
						self.wafer.die.height = f;

						if self.simple_critical_area {
							self.wafer.critical_area = self.wafer.die.area();
						} else {
							self.wafer.fix_critical_area();
						}
					}
				}
				Component::CriticalArea => {
					if !self.simple_critical_area {
						self.wafer.critical_area = f.min(self.wafer.die.area());
					}
				}
				Component::DefectRate => self.wafer.defect_rate = f,
				Component::EdgeLoss => self.wafer.edge_loss = f,
				Component::ScribeHorizontal => {
					f = f.max(MINIMUM_SCRIBE_WIDTH).min(MAXIMUM_SCRIBE_WIDTH);
					self.wafer.scribe_lanes.0 = f;
					if self.scribe_equal {
						self.wafer.scribe_lanes.1 = f;
					}
				}
				Component::ScribeVertical => {
					f = f.max(MINIMUM_SCRIBE_WIDTH).min(MAXIMUM_SCRIBE_WIDTH);
					self.wafer.scribe_lanes.1 = f;
				},
				Component::TranslateHorizontal => self.wafer.translation.0 = f,
				Component::TranslateVertical => self.wafer.translation.1 = f,
				_ => {}
			},
			Message::YieldModel(m) => self.wafer.yield_model = m,
			Message::None => {}
		}

		self.wafer_view.request_redraw();
	}

	fn view(&self) -> Element<'_, Message> {
		let die_size_inputs = die_size(&self.wafer, self.die_square);
		let critical_area_inputs = critical_area(&self.wafer, self.simple_critical_area);
		let diameter_input = diameter(self.diameter);
		let defect_rate_input = defect_rate(self.wafer.defect_rate);
		let edge_loss_input = edge_loss(self.wafer.edge_loss);
		let scribe_lanes_inputs = scribe_lines(&self.wafer, self.scribe_equal);
		let translation_inputs = translation(&self.wafer);
		let centering_input = die_centering(self.wafer.centered);
		let yield_model_input = yield_model(self.wafer.yield_model);

		let options = column![
			die_size_inputs,
			critical_area_inputs,
			diameter_input,
			defect_rate_input,
			edge_loss_input,
			scribe_lanes_inputs,
			translation_inputs,
			centering_input,
			yield_model_input,
		]
		.spacing(16)
		.padding(8)
		.width(Length::FillPortion(3))
		.align_items(Alignment::Start);

		let wafer_view = container(self.wafer_view.view(&self.wafer))
			.width(Length::Fill)
			.padding(4)
			.center_x()
			.center_y();

		let wafer_view_column = column![wafer_view.height(Length::FillPortion(4))]
			.height(Length::Shrink)
			.width(Length::FillPortion(3))
			.align_items(Alignment::Center);

		let content = row![options, wafer_view_column]
			.spacing(0)
			.padding(12)
			.height(Length::Shrink)
			.align_items(Alignment::Center);

		container(content)
			.height(Length::Shrink)
			.width(Length::Shrink)
			.center_x()
			.center_y()
			.into()
	}

	fn theme(&self) -> Theme {
		let mut palette = Palette::LIGHT;
		palette.primary = Color::from_rgb8(120, 120, 120);
		Theme::custom(palette)
	}
}
