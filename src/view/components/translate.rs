/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Alignment, Length};
use iced::widget::{column, container, Row, row, text};
use iced_aw::NumberInput;

use crate::view::calculator::{Component, Message};
use crate::view::ROW_HEIGHT;
use crate::wafer::Wafer;

pub fn translation(wafer: &Wafer) -> Row<Message> {
	let labels = column![
		container(text("Horizontal")).height(ROW_HEIGHT).center_y(),
		container(text("Vertical")).height(ROW_HEIGHT).center_y(),
	];

	let inputs = column![
		container(
			NumberInput::new(wafer.translation.0, wafer.die.width, |f| Message::NumberInput(
				Component::TranslateHorizontal,
				f
			))
			.min(0.0)
			.step(0.2)
		)
		.height(ROW_HEIGHT)
		.center_y(),
		container(
			NumberInput::new(wafer.translation.1, wafer.die.height, |f| Message::NumberInput(Component::TranslateVertical, f))
				.min(0.0)
				.step(0.2)
		)
		.height(ROW_HEIGHT)
		.center_y(),
	];

	row![
		container(text("Translation (mm)"))
			.height(ROW_HEIGHT)
			.center_y()
			.width(Length::FillPortion(4)),
		labels.width(Length::FillPortion(2)),
		inputs.width(Length::FillPortion(4)),
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}
