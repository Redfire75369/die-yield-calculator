/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Alignment, Length};
use iced::widget::{checkbox, column, container, horizontal_space, pick_list, row, Row, text};
use iced_aw::NumberInput;

use crate::view::calculator::{Component, Message};
use crate::view::ROW_HEIGHT;
use crate::wafer::{Diameter, Wafer, YieldModel};

pub fn diameter(diameter: Diameter) -> Row<'static, Message> {
	let label = container(text("Wafer Diameter")).height(ROW_HEIGHT).center_y();
	let picker = container(pick_list(Diameter::ALL, Some(diameter), Message::Diameter))
		.height(ROW_HEIGHT)
		.center_y();

	row![
		label.width(Length::FillPortion(4)),
		picker.width(Length::FillPortion(5)),
		horizontal_space(Length::FillPortion(1))
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}

pub fn defect_rate(defect_rate: f32) -> Row<'static, Message> {
	let label = container(text("Defect Rate (#/cm²)")).height(ROW_HEIGHT).center_y();
	let input = container(
		NumberInput::new(defect_rate, 10000.0, |f| Message::NumberInputChange(Component::DefectRate, f))
			.min(0.0)
			.step(0.05),
	)
	.height(ROW_HEIGHT)
	.center_y();

	row![
		label.width(Length::FillPortion(4)),
		input.width(Length::FillPortion(5)),
		horizontal_space(Length::FillPortion(1))
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}

pub fn edge_loss(edge_loss: f32) -> Row<'static, Message> {
	let label = container(text("Edge Loss (mm)")).height(ROW_HEIGHT).center_y();
	let input = container(
		NumberInput::new(edge_loss, 25.0, |f| Message::NumberInputChange(Component::EdgeLoss, f))
			.min(0.0)
			.step(0.2),
	)
	.height(ROW_HEIGHT)
	.center_y();

	row![
		label.width(Length::FillPortion(4)),
		input.width(Length::FillPortion(5)),
		horizontal_space(Length::FillPortion(1))
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}

pub fn die_centering(centered: bool) -> Row<'static, Message> {
	let label = container(text("Centering")).height(ROW_HEIGHT).center_y();
	let checkbox = container(checkbox("", centered, Message::Center));

	row![
		label.width(Length::FillPortion(4)),
		checkbox.width(Length::FillPortion(5)),
		horizontal_space(Length::FillPortion(1))
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}

pub fn yield_model(yield_model: YieldModel) -> Row<'static, Message> {
	let label = container(text("Yield Model")).height(ROW_HEIGHT).center_y();
	let picker = container(pick_list(YieldModel::ALL, Some(yield_model), Message::YieldModel))
		.height(ROW_HEIGHT)
		.center_y();

	row![
		label.width(Length::FillPortion(4)),
		picker.width(Length::FillPortion(5)),
		horizontal_space(Length::FillPortion(1))
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}

pub fn die_yield_info(wafer: &Wafer) -> Row<Message> {
	let die_yield = wafer.yield_model.wafer_yield(wafer);
	let die_types = wafer.get_die_counts();

	let bad_dies = ((die_types.0 as f32) * (1.0 - die_yield)) as u32;
	let good_dies = die_types.0 - bad_dies;

	let extreme = column![
		container(text(format!("Good Dies {}", good_dies))).height(ROW_HEIGHT).center_y(),
		container(text(format!("Wasted Dies {}", die_types.2))).height(ROW_HEIGHT).center_y(),
	];

	let middle = column![
		container(text(format!("Defective Dies {}", bad_dies))).height(ROW_HEIGHT).center_y(),
		container(text(format!("Partial Dies {}", die_types.1))).height(ROW_HEIGHT).center_y(),
	];

	let other = column![
		container(text(format!("Maximum Dies {}", die_types.0))).height(ROW_HEIGHT).center_y(),
		container(text(format!("Fab Yield {:.2}%", die_yield * 100.0)))
			.height(ROW_HEIGHT)
			.center_y(),
	];

	row![
		extreme.width(Length::FillPortion(3)),
		middle.width(Length::FillPortion(3)),
		other.width(Length::FillPortion(3)),
	]
	.height(Length::Shrink)
	.width(Length::Fill)
	.align_items(Alignment::Center)
}
