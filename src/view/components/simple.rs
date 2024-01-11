/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::widget::{checkbox, container, pick_list, text};
use iced_aw::{grid_row, GridRow, NumberInput};

use crate::view::calculator::{Component, Message};
use crate::view::ROW_HEIGHT;
use crate::wafer::{Diameter, Panel, Shape, ShapeOption, Wafer, YieldModel};

pub fn critical_area(wafer: &Wafer, simple: bool) -> GridRow<'static, Message> {
	let label = container(text("Critical Area (mm²)")).height(ROW_HEIGHT).center_y();
	let input = container(
		NumberInput::new(
			wafer.critical_area,
			wafer.die.area(),
			Message::number_input(Component::CriticalArea),
		)
		.min(if simple { wafer.die.area() } else { 0.0 })
		.step(0.5),
	)
	.height(ROW_HEIGHT)
	.center_y();
	let check = checkbox("Simple", simple, Message::checkbox(Component::CriticalArea));

	grid_row![label, input, check]
}

pub fn shape(shape: Shape) -> GridRow<'static, Message> {
	let label = container(text("Shape")).height(ROW_HEIGHT).center_y();
	let options = container(pick_list(ShapeOption::ALL, Some(shape.into()), Message::ShapeOption))
		.height(ROW_HEIGHT)
		.center_y();
	let picker = match shape {
		Shape::Wafer(diameter) => container(pick_list(Diameter::ALL, Some(diameter), |d| {
			Message::Shape(Shape::Wafer(d))
		})),
		Shape::Panel(panel) => container(pick_list(Panel::ALL, Some(panel), |p| Message::Shape(Shape::Panel(p)))),
	};
	let picker = picker.height(ROW_HEIGHT).center_y();

	grid_row![label, options, picker]
}

pub fn defect_rate(defect_rate: f32) -> GridRow<'static, Message> {
	let label = container(text("Defect Rate (#/cm²)")).height(ROW_HEIGHT).center_y();
	let input = container(
		NumberInput::new(defect_rate, 10000.0, Message::number_input(Component::DefectRate))
			.min(0.0)
			.step(0.05),
	)
	.height(ROW_HEIGHT)
	.center_y();

	grid_row![label, input]
}

pub fn edge_loss(edge_loss: f32) -> GridRow<'static, Message> {
	let label = container(text("Edge Loss (mm)")).height(ROW_HEIGHT).center_y();
	let input = container(
		NumberInput::new(edge_loss, 25.0, Message::number_input(Component::EdgeLoss))
			.min(0.0)
			.step(0.2),
	)
	.height(ROW_HEIGHT)
	.center_y();

	grid_row![label, input]
}

pub fn die_centering(centered: bool) -> GridRow<'static, Message> {
	let label = container(text("Centering")).height(ROW_HEIGHT).center_y();
	let checkbox = container(checkbox("", centered, Message::Center));

	grid_row![label, checkbox]
}

pub fn yield_model(yield_model: YieldModel) -> GridRow<'static, Message> {
	let label = container(text("Yield Model")).height(ROW_HEIGHT).center_y();
	let picker = container(pick_list(YieldModel::ALL, Some(yield_model), Message::YieldModel))
		.height(ROW_HEIGHT)
		.center_y();

	grid_row![label, picker]
}
