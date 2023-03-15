/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Alignment, Length};
use iced::widget::{checkbox, column, container, Row, row, text};
use iced_aw::NumberInput;

use crate::die::max_other_dimension;
use crate::view::calculator::{Component, Message};
use crate::view::ROW_HEIGHT;
use crate::wafer::{MINIMUM_DIE_DIMENSION, Wafer};

pub fn die_size(wafer: &Wafer, die_square: bool) -> Row<Message> {
	let width_label = container(text("Die Width (mm)")).height(ROW_HEIGHT).center_y();
	let height_label = container(text("Die Height (mm)")).height(ROW_HEIGHT).center_y();
	let labels = column![width_label, height_label];

	let width_input = container(
		NumberInput::new(
			wafer.die.width,
			max_other_dimension(wafer.die.height),
			Message::number_input(Component::DieWidth),
		)
		.min(MINIMUM_DIE_DIMENSION)
		.step(0.2),
	)
	.height(ROW_HEIGHT)
	.center_y();
	let height_input = container(
		NumberInput::new(
			wafer.die.height,
			if die_square { wafer.die.height } else { max_other_dimension(wafer.die.width) },
			Message::number_input(Component::DieHeight),
		)
		.min(if die_square { wafer.die.height } else { MINIMUM_DIE_DIMENSION })
		.step(0.2),
	)
	.height(ROW_HEIGHT)
	.center_y();

	let inputs = column![width_input, height_input];

	row![
		labels.width(Length::FillPortion(4)),
		inputs.width(Length::FillPortion(5)),
		checkbox("", die_square, Message::checkbox(Component::DieWidth)).width(Length::FillPortion(1))
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}
