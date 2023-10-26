/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::Length;
use iced::widget::{column, container, text};
use iced_aw::{grid_row, GridRow, NumberInput};

use crate::view::calculator::{Component, Message};
use crate::view::ROW_HEIGHT;
use crate::wafer::Wafer;

pub fn translation(wafer: &Wafer) -> GridRow<'static, Message> {
	let horizontal_label = container(text("Horizontal")).height(ROW_HEIGHT).center_y();
	let vertical_label = container(text("Vertical")).height(ROW_HEIGHT).center_y();
	let labels = column![horizontal_label, vertical_label];

	let horizontal_input = container(
		NumberInput::new(
			wafer.translation.0,
			wafer.die.width(),
			Message::number_input(Component::TranslateHorizontal),
		)
		.min(0.0)
		.step(0.2),
	)
	.height(ROW_HEIGHT)
	.center_y();
	let vertical_input = container(
		NumberInput::new(
			wafer.translation.1,
			wafer.die.height(),
			Message::number_input(Component::TranslateVertical),
		)
		.min(0.0)
		.step(0.2),
	)
	.height(ROW_HEIGHT)
	.center_y();
	let inputs = column![horizontal_input, vertical_input];

	grid_row![
		container(text("Translation (mm)"))
			.height(ROW_HEIGHT)
			.center_y()
			.width(Length::FillPortion(4)),
		labels.width(Length::FillPortion(2)),
		inputs.width(Length::FillPortion(4)),
	]
}
