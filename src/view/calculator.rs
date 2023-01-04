/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Alignment, Color, Element, Length, Sandbox, Theme};
use iced::theme::Palette;
use iced::widget::{column, container, row};

use crate::view::components::{defect_rate, diameter, die_centering, die_size, die_yield_info, edge_loss, scribe_lines, translation, yield_model};
use crate::view::wafer::WaferViewState;
use crate::wafer::{Diameter, Wafer, YieldModel};

pub struct Calculator {
	wafer: Wafer,
	diameter: Diameter,

	die_square: bool,
	scribe_equal: bool,

	wafer_view: WaferViewState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Component {
	DieWidth,
	DieHeight,
	Diameter,
	DefectRate,
	EdgeLoss,
	ScribeHorizontal,
	ScribeVertical,
	TranslateHorizontal,
	TranslateVertical,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
	Center(bool),
	Diameter(Diameter),
	DimensionsEqual(Component, bool),
	NumberInputChange(Component, f32),
	YieldModel(YieldModel),
	None,
}

impl Sandbox for Calculator {
	type Message = Message;

	fn new() -> Self {
		let wafer = Wafer::default();
		Calculator {
			wafer,
			diameter: Diameter::FOUR,

			die_square: false,
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
			Message::Diameter(d) => {
				self.wafer.diameter = d as u16 as f32;
				self.diameter = d;
			}
			Message::DimensionsEqual(c, b) => match c {
				Component::DieWidth => {
					self.wafer.die.height = self.wafer.die.width;
					self.die_square = b;
				}
				Component::ScribeHorizontal => {
					self.wafer.scribe_lanes.1 = self.wafer.scribe_lanes.0;
					self.scribe_equal = b;
				}
				_ => {}
			},
			Message::NumberInputChange(c, f) => match c {
				Component::DieWidth => {
					self.wafer.die.width = f;
					if self.die_square {
						self.wafer.die.height = f;
					}
				}
				Component::DieHeight => self.wafer.die.height = f,
				Component::DefectRate => self.wafer.defect_rate = f,
				Component::EdgeLoss => self.wafer.edge_loss = f,
				Component::ScribeHorizontal => {
					self.wafer.scribe_lanes.0 = f;
					if self.scribe_equal {
						self.wafer.scribe_lanes.1 = f;
					}
				}
				Component::ScribeVertical => self.wafer.scribe_lanes.1 = f,
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
		let diameter_input = diameter(self.diameter);
		let defect_rate_input = defect_rate(self.wafer.defect_rate);
		let edge_loss_input = edge_loss(self.wafer.edge_loss);
		let scribe_lanes_inputs = scribe_lines(&self.wafer, self.scribe_equal);
		let translation_inputs = translation(&self.wafer);
		let centering_input = die_centering(self.wafer.centered);
		let yield_model_input = yield_model(self.wafer.yield_model);

		let options = column![
			die_size_inputs,
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

		let wafer_yield_info = die_yield_info(&self.wafer);

		let wafer_view_column = column![wafer_view.height(Length::FillPortion(4)), wafer_yield_info.height(Length::FillPortion(1))]
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
