/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Alignment, Length};
use iced::widget::{checkbox, column, container, Row, row, text};
use iced_aw::NumberInput;
use crate::die::Die;

use crate::view::calculator::{Component, Message};
use crate::view::ROW_HEIGHT;
use crate::wafer::Wafer;

pub fn die_size(wafer: &Wafer, reticle_limit: bool) -> Row<Message> {
	let width_label = container(text("Die Width (mm)")).height(ROW_HEIGHT).center_y();
	let height_label = container(text("Die Height (mm)")).height(ROW_HEIGHT).center_y();
	let labels = column![width_label, height_label];

	let width_input = container(
		NumberInput::new(wafer.die.width(), 0.0, Message::number_input(Component::DieWidth))
			.bounds(wafer.die.width_bounds(reticle_limit, wafer.shape))
			.step(0.2),
	)
	.height(ROW_HEIGHT)
	.center_y();

	let height_input = container(
		NumberInput::new(wafer.die.height(), 0.0, Message::number_input(Component::DieHeight))
			.bounds(wafer.die.height_bounds(reticle_limit, wafer.shape))
			.step(0.2),
	)
	.height(ROW_HEIGHT)
	.center_y();

	let inputs = column![width_input, height_input];

	let square = checkbox("Square", matches!(wafer.die, Die::Square(_)), Message::checkbox(Component::DieWidth));
	let reticle = checkbox("Reticle Limit", reticle_limit, Message::checkbox(Component::Reticle));

	row![
		labels.width(Length::FillPortion(4)),
		inputs.width(Length::FillPortion(2)),
		square.width(Length::FillPortion(2)),
		reticle.width(Length::FillPortion(2)),
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}
