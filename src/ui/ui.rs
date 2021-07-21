/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Sandbox, Element, Column, Align, Text, Row, Length, Container, Canvas, Checkbox, PickList, pick_list};
use crate::wafer::Wafer;
use crate::ui::input::{NumberInput};
use crate::ui::wafer::WaferDisplay;
use std::fmt::{Display, Formatter, Debug, Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Diameter(u16);

impl Diameter {
	const ALL: [Diameter; 7] = [
		Diameter(51),
		Diameter(76),
		Diameter(100),
		Diameter(150),
		Diameter(200),
		Diameter(300),
		Diameter(450),
	];
}

impl Display for Diameter {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.0 {
			51 => f.write_str("51 mm (2 in)"),
			76 => f.write_str("76 mm (3 in)"),
			100 => f.write_str("100 mm (4 in)"),
			150 => f.write_str("150 mm (6 in)"),
			200 => f.write_str("200 mm (8 in)"),
			300 => f.write_str("300 mm (12 in)"),
			450 => f.write_str("450 mm (18 in)"),
			_ => Err(Error),
		}
	}
}

pub struct Calculator {
	wafer: Wafer,
	diameter: Diameter,

	die_square: bool,
	scribe_equal: bool,

	die_width: NumberInput,
	die_height: NumberInput,
	diameter_list: pick_list::State<Diameter>,
	defect_rate: NumberInput,
	scribe_horizontal: NumberInput,
	scribe_vertical: NumberInput,
	translate_horizontal: NumberInput,
	translate_vertical: NumberInput,
	centered: bool,

	wafer_display: WaferDisplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Component {
	DieWidth,
	DieHeight,
	Diameter,
	DefectRate,
	ScribeHorizontal,
	ScribeVertical,
	TranslateHorizontal,
	TranslateVertical,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
	Edit(Component, String, bool),
	Increment(Component, bool),
	Decrement(Component, bool),
	DimensionsEqual(Component, bool),
	Diameter(Diameter),
	Center(bool),
}

impl Sandbox for Calculator {
	type Message = Message;

	fn new() -> Calculator {
		let wafer = Wafer::default();
		Calculator {
			wafer,
			diameter: Diameter::ALL[2],

			die_square: false,
			scribe_equal: false,

			die_width: NumberInput::new(wafer.die.width, 0.0001, 50.0, 0.2, Component::DieWidth),
			die_height: NumberInput::new(wafer.die.height, 0.0001, 50.0, 0.2, Component::DieHeight),
			diameter_list: pick_list::State::default(),
			defect_rate: NumberInput::new(wafer.defect_rate, 0.0001, 1000.0, 0.02, Component::DefectRate),
			scribe_horizontal: NumberInput::new(wafer.scribe_lanes.0, 0.0001, 50.0, 0.05, Component::ScribeHorizontal),
			scribe_vertical: NumberInput::new(wafer.scribe_lanes.1, 0.0001, 50.0, 0.05, Component::ScribeVertical),
			translate_horizontal: NumberInput::new(wafer.translation.0, -50.0, 50.0, 0.05, Component::TranslateHorizontal),
			translate_vertical: NumberInput::new(wafer.translation.1, -50.0, 50.0, 0.05, Component::TranslateVertical),
			centered: false,

			wafer_display: WaferDisplay::new(&wafer),
		}
	}

	fn title(&self) -> String {
		String::from("Die Yield Calculator")
	}

	fn update(&mut self, message: Message) {
		match message {
			Message::Edit(c, s, b) => match c {
				Component::DieWidth => {
					if self.die_square {
						self.die_height.update(Message::Edit(Component::DieHeight, s.clone(), false))
					}
					self.die_width.update(Message::Edit(c, s, false))
				}
				Component::DieHeight => self.die_height.update(Message::Edit(c, s, b)),
				Component::DefectRate => self.defect_rate.update(Message::Edit(c, s, false)),
				Component::ScribeHorizontal => {
					if self.scribe_equal {
						self.scribe_vertical.update(Message::Edit(Component::ScribeVertical, s.clone(), false))
					}
					self.scribe_horizontal.update(Message::Edit(c, s, false))
				}
				Component::ScribeVertical => self.scribe_vertical.update(Message::Edit(c, s, b)),
				Component::TranslateHorizontal => self.translate_horizontal.update(Message::Edit(c, s, b)),
				Component::TranslateVertical => self.translate_vertical.update(Message::Edit(c, s, b)),
				_ => {}
			},
			Message::Increment(c, b) => match c {
				Component::DieWidth => {
					if self.die_square {
						self.die_height.update(Message::Increment(Component::DieHeight, false))
					}
					self.die_width.update(Message::Increment(c, b))
				}
				Component::DieHeight => self.die_height.update(Message::Increment(c, b)),
				Component::DefectRate => self.defect_rate.update(Message::Increment(c, b)),
				Component::ScribeHorizontal => {
					if self.scribe_equal {
						self.scribe_vertical.update(Message::Increment(Component::ScribeVertical, false))
					}
					self.scribe_horizontal.update(Message::Increment(c, b))
				}
				Component::ScribeVertical => self.scribe_vertical.update(Message::Increment(c, b)),
				Component::TranslateHorizontal => self.translate_horizontal.update(Message::Increment(c, b)),
				Component::TranslateVertical => self.translate_vertical.update(Message::Increment(c, b)),
				_ => {}
			},
			Message::Decrement(c, b) => match c {
				Component::DieWidth => {
					if self.die_square {
						self.die_height.update(Message::Decrement(Component::DieHeight, false))
					}
					self.die_width.update(Message::Decrement(c, b));
				}
				Component::DieHeight => self.die_height.update(Message::Decrement(c, b)),
				Component::DefectRate => self.defect_rate.update(Message::Decrement(c, b)),
				Component::ScribeHorizontal => {
					if self.scribe_equal {
						self.scribe_vertical.update(Message::Decrement(Component::ScribeVertical, false))
					}
					self.scribe_horizontal.update(Message::Decrement(c, b))
				}
				Component::ScribeVertical => self.scribe_vertical.update(Message::Decrement(c, b)),
				Component::TranslateHorizontal => self.translate_horizontal.update(Message::Decrement(c, b)),
				Component::TranslateVertical => self.translate_vertical.update(Message::Decrement(c, b)),
				_ => {}
			},
			Message::DimensionsEqual(c, b) => match c {
				Component::DieWidth => {
					self.die_height.update(Message::DimensionsEqual(c, b));
					self.die_height
						.update(Message::Edit(Component::DieHeight, self.wafer.die.width.to_string(), false));
					self.die_square = b;
				}
				Component::ScribeHorizontal => {
					self.scribe_vertical.update(Message::DimensionsEqual(c, b));
					self.scribe_vertical
						.update(Message::Edit(Component::ScribeVertical, self.wafer.scribe_lanes.0.to_string(), false));
					self.scribe_equal = b;
				}
				_ => {}
			},
			Message::Diameter(d) => {
				self.wafer.diameter = d.0 as f32;
				self.diameter = d;
			}
			Message::Center(b) => {
				self.wafer.centered = b;
				self.centered = b;
			}
		}

		self.wafer.die.width = self.die_width.get();
		self.wafer.die.height = self.die_height.get();
		self.wafer.defect_rate = self.defect_rate.get();
		self.wafer.scribe_lanes.0 = self.scribe_horizontal.get();
		self.wafer.scribe_lanes.1 = self.scribe_vertical.get();
		self.wafer.translation.0 = self.translate_horizontal.get();
		self.wafer.translation.1 = self.translate_vertical.get();

		self.wafer_display.update(&self.wafer);
	}

	fn view(&mut self) -> Element<Message> {
		let diameters = &Diameter::ALL[..];

		Container::new(
			Row::with_children(vec![
				Column::with_children(vec![
					Row::with_children(vec![
						Column::with_children(vec![
							self.die_width
								.view(Row::with_children(vec![Text::new("Die Width (mm):").into()]))
								.height(Length::Shrink)
								.width(Length::Fill)
								.align_items(Align::Center)
								.spacing(4)
								.into(),
							self.die_height
								.view(Row::with_children(vec![Text::new("Die Height (mm):").into()]))
								.height(Length::Shrink)
								.width(Length::Fill)
								.align_items(Align::Center)
								.spacing(4)
								.into(),
						])
						.width(Length::FillPortion(9))
						.spacing(2)
						.into(),
						Checkbox::new(self.die_square, "", move |b| Message::DimensionsEqual(Component::DieWidth, b))
							.width(Length::FillPortion(1))
							.into(),
					])
					.height(Length::Shrink)
					.width(Length::Fill)
					.align_items(Align::Center)
					.spacing(8)
					.into(),
					Row::with_children(vec![
						Text::new("Wafer Diameter:").into(),
						PickList::new(&mut self.diameter_list, diameters, Some(self.diameter.clone()), Message::Diameter).into(),
					])
					.height(Length::Shrink)
					.width(Length::Fill)
					.align_items(Align::Center)
					.spacing(8)
					.into(),
					self.defect_rate
						.view(Row::with_children(vec![Text::new("Defect Rate (#/cm²:").into()]))
						.height(Length::Shrink)
						.width(Length::Fill)
						.align_items(Align::Center)
						.spacing(8)
						.into(),
					Row::with_children(vec![
						Text::new("Scribe Lanes (mm):").into(),
						Column::with_children(vec![
							self.scribe_horizontal
								.view(Row::with_children(vec![Text::new("Horizontal:").into()]))
								.height(Length::Shrink)
								.width(Length::Fill)
								.align_items(Align::Center)
								.spacing(4)
								.into(),
							self.scribe_vertical
								.view(Row::with_children(vec![Text::new("Vertical:").into()]))
								.height(Length::Shrink)
								.width(Length::Fill)
								.align_items(Align::Center)
								.spacing(4)
								.into(),
						])
						.width(Length::FillPortion(9))
						.spacing(2)
						.into(),
						Checkbox::new(self.scribe_equal, "", move |b| Message::DimensionsEqual(Component::ScribeHorizontal, b))
							.width(Length::FillPortion(1))
							.into(),
					])
					.height(Length::Shrink)
					.width(Length::Fill)
					.align_items(Align::Center)
					.spacing(4)
					.into(),
					Row::with_children(vec![
						Text::new("Translation (mm):").into(),
						Column::with_children(vec![
							self.translate_horizontal
								.view(Row::with_children(vec![Text::new("Horizontal:").into()]))
								.height(Length::Shrink)
								.width(Length::Fill)
								.align_items(Align::Center)
								.spacing(4)
								.into(),
							self.translate_vertical
								.view(Row::with_children(vec![Text::new("Vertical:").into()]))
								.height(Length::Shrink)
								.width(Length::Fill)
								.align_items(Align::Center)
								.spacing(4)
								.into(),
						])
						.width(Length::FillPortion(9))
						.spacing(2)
						.into(),
					])
					.height(Length::Shrink)
					.width(Length::Fill)
					.align_items(Align::Center)
					.spacing(4)
					.into(),
					Row::with_children(vec![
						Text::new("Die Centering:").into(),
						Checkbox::new(self.centered, "", Message::Center).into(),
					])
					.height(Length::Shrink)
					.width(Length::Fill)
					.align_items(Align::Center)
					.spacing(4)
					.into(),
				])
				.padding(8)
				.spacing(8)
				.align_items(Align::Start)
				.width(Length::FillPortion(2))
				.into(),
				Column::with_children(vec![Container::new(
					Canvas::new(&mut self.wafer_display).width(Length::Fill).height(Length::Fill),
				)
				.width(Length::Fill)
				.height(Length::Fill)
				.padding(20)
				.center_x()
				.center_y()
				.into()])
				.padding(8)
				.align_items(Align::Center)
				.width(Length::FillPortion(3))
				.into(),
			])
			.padding(8)
			.height(Length::Shrink)
			.align_items(Align::Center),
		)
		.height(Length::Shrink)
		.width(Length::Fill)
		.center_y()
		.into()
	}
}
