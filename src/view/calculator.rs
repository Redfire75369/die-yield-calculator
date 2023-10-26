/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Alignment, Color, Element, Length, Sandbox, Theme};
use iced::theme::Palette;
use iced::widget::{column, container, row};
use iced_aw::grid;

use crate::util::min_if;
use crate::view::components::{critical_area, defect_rate, shape, die_centering, die_size, edge_loss, scribe_lines, translation, yield_model};
use crate::view::wafer::WaferViewState;
use crate::wafer::{Diameter, MAXIMUM_SCRIBE_WIDTH, Panel, Shape, ShapeOption, Wafer, YieldModel};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Component {
	DieWidth,
	DieHeight,
	Reticle,
	CriticalArea,
	Shape,
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
	ShapeOption(ShapeOption),
	Shape(Shape),
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

	reticle_limit: bool,
	simple_critical_area: bool,
	scribe_equal: bool,

	wafer_view: WaferViewState,
}

impl Sandbox for Calculator {
	type Message = Message;

	fn new() -> Calculator {
		Calculator {
			wafer: Wafer::default(),

			reticle_limit: true,
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
				Component::DieWidth if b => self.wafer.die = self.wafer.die.square(self.reticle_limit),
				Component::DieWidth => self.wafer.die = self.wafer.die.rectangle(),
				Component::Reticle if b => {
					self.reticle_limit = true;
					self.wafer.die = self.wafer.die.clamp_reticle();
				}
				Component::Reticle => self.reticle_limit = false,
				Component::CriticalArea => {
					self.simple_critical_area = b;
				}
				Component::ScribeHorizontal => {
					self.scribe_equal = b;
					self.wafer.scribe_lanes.1 = self.wafer.scribe_lanes.0;
				}
				_ => {}
			},
			Message::Shape(shape) => {
				self.wafer.shape = shape;
			}
			Message::ShapeOption(opt) => {
				self.wafer.shape = match opt {
					ShapeOption::Wafer => Shape::Wafer(Diameter::default()),
					ShapeOption::Panel => Shape::Panel(Panel::default()),
				};
			}
			Message::NumberInput(c, mut f) => match c {
				Component::DieWidth => self.wafer.die = self.wafer.die.new_width(f),
				Component::DieHeight => self.wafer.die = self.wafer.die.new_height(f),
				Component::CriticalArea => {
					self.wafer.critical_area = min_if(!self.simple_critical_area, f, self.wafer.die.area());
				}
				Component::DefectRate => self.wafer.defect_rate = f,
				Component::EdgeLoss => self.wafer.edge_loss = f,
				Component::ScribeHorizontal => {
					f = f.min(MAXIMUM_SCRIBE_WIDTH);
					self.wafer.scribe_lanes.0 = f;
					if self.scribe_equal {
						self.wafer.scribe_lanes.1 = f;
					}
				}
				Component::ScribeVertical => {
					f = f.min(MAXIMUM_SCRIBE_WIDTH);
					self.wafer.scribe_lanes.1 = f;
				}
				Component::TranslateHorizontal => self.wafer.translation.0 = f,
				Component::TranslateVertical => self.wafer.translation.1 = f,
				_ => {}
			},
			Message::YieldModel(m) => self.wafer.yield_model = m,
			Message::None => {}
		}

		if self.simple_critical_area {
			self.wafer.critical_area = self.wafer.die.area();
		} else {
			self.wafer.clamp_critical_area();
		}

		self.wafer_view.request_redraw();
	}

	fn view(&self) -> Element<'_, Message> {
		let die_size_inputs = die_size(&self.wafer, self.reticle_limit);
		let critical_area_inputs = critical_area(&self.wafer, self.simple_critical_area);
		let shape_input = shape(self.wafer.shape);
		let defect_rate_input = defect_rate(self.wafer.defect_rate);
		let edge_loss_input = edge_loss(self.wafer.edge_loss);
		let scribe_lanes_inputs = scribe_lines(&self.wafer, self.scribe_equal);
		let translation_inputs = translation(&self.wafer);
		let centering_input = die_centering(self.wafer.centered);
		let yield_model_input = yield_model(self.wafer.yield_model);

		let options = grid![
			die_size_inputs,
			critical_area_inputs,
			shape_input,
			defect_rate_input,
			edge_loss_input,
			scribe_lanes_inputs,
			translation_inputs,
			centering_input,
			yield_model_input,
		]
		.spacing(4.0)
		.width(Length::FillPortion(3));

		let wafer_view = container(self.wafer_view.view(&self.wafer))
			.width(Length::Fill)
			.padding(4)
			.center_x()
			.center_y();

		let wafer_view_column = column![wafer_view]
			.height(Length::Shrink)
			.width(Length::FillPortion(2))
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
