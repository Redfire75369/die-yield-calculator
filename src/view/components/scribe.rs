/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Alignment, Length};
use iced::widget::{checkbox, column, container, Row, row, text};
use iced_aw::NumberInput;

use crate::view::calculator::{Component, Message};
use crate::view::ROW_HEIGHT;
use crate::wafer::Wafer;

pub fn scribe_lines(wafer: &Wafer, equal_scribe: bool) -> Row<Message> {
	let labels = column![
		container(text("Horizontal")).height(ROW_HEIGHT).center_y(),
		container(text("Vertical")).height(ROW_HEIGHT).center_y(),
	];

	let inputs = column![
		container(
			NumberInput::new(wafer.scribe_lanes.0, 33.0, |f| Message::NumberInputChange(Component::ScribeHorizontal, f))
				.min(0.001)
				.step(0.2)
		)
		.height(ROW_HEIGHT)
		.center_y(),
		container(
			NumberInput::new(wafer.scribe_lanes.1, 33.0, |f| Message::NumberInputChange(Component::ScribeVertical, f))
				.min(0.001)
				.step(0.2)
		)
		.height(ROW_HEIGHT)
		.center_y(),
	];

	row![
		container(text("Scribe Lines (mm)"))
			.height(ROW_HEIGHT)
			.center_y()
			.width(Length::FillPortion(4)),
		labels.width(Length::FillPortion(2)),
		inputs.width(Length::FillPortion(3)),
		checkbox("", equal_scribe, |b| Message::DimensionsEqual(Component::ScribeHorizontal, b)).width(Length::FillPortion(1)),
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}
