/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::widget::{checkbox, column, container, text};
use iced_aw::{grid_row, GridRow, NumberInput};

use crate::view::calculator::{Component, Message};
use crate::view::ROW_HEIGHT;
use crate::wafer::{MAXIMUM_SCRIBE_WIDTH, Wafer};

pub fn scribe_lines(wafer: &Wafer, equal_scribe: bool) -> GridRow<'static, Message> {
	let horizontal_label = container(text("Horizontal")).height(ROW_HEIGHT).center_y();
	let vertical_label = container(text("Vertical")).height(ROW_HEIGHT).center_y();
	let labels = column![horizontal_label, vertical_label];

	let horizontal_input = container(
		NumberInput::new(
			wafer.scribe_lanes.0,
			MAXIMUM_SCRIBE_WIDTH,
			Message::number_input(Component::ScribeHorizontal),
		)
		.step(0.2),
	)
	.center_y();

	let vertical_input = container(
		NumberInput::new(
			wafer.scribe_lanes.1,
			if equal_scribe {
				wafer.scribe_lanes.1
			} else {
				MAXIMUM_SCRIBE_WIDTH
			},
			Message::number_input(Component::ScribeVertical),
		)
		.min(if equal_scribe { wafer.scribe_lanes.1 } else { 0.0 })
		.step(0.2),
	)
	.center_y();

	let inputs = column![horizontal_input, vertical_input];

	grid_row![
		container(text("Scribe Lines (mm)")).center_y(),
		labels,
		inputs,
		checkbox("", equal_scribe, Message::checkbox(Component::ScribeHorizontal)),
	]
}
