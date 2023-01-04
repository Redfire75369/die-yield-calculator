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

pub fn die_size(wafer: &Wafer, die_square: bool) -> Row<Message> {
	let labels = column![
		container(text("Die Width (mm)")).height(ROW_HEIGHT).center_y(),
		container(text("Die Height (mm)")).height(ROW_HEIGHT).center_y(),
	];

	let inputs = column![
		container(
			NumberInput::new(wafer.die.width, 33.0, |f| Message::NumberInputChange(Component::DieWidth, f))
				.min(0.001)
				.step(0.2)
		)
		.height(ROW_HEIGHT)
		.center_y(),
		container(
			NumberInput::new(wafer.die.height, 33.0, |f| Message::NumberInputChange(Component::DieWidth, f))
				.min(0.001)
				.step(0.2)
		)
		.height(ROW_HEIGHT)
		.center_y(),
	];

	row![
		labels.width(Length::FillPortion(4)),
		inputs.width(Length::FillPortion(5)),
		checkbox("", die_square, |b| Message::DimensionsEqual(Component::DieWidth, b)).width(Length::FillPortion(1))
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}