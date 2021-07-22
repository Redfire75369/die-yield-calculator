/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Align, button, Button, Column, Length, Row, Text, text_input, TextInput, Color, Background};

use crate::ui::ui::{Component, Message};
use crate::ui::styles;

pub struct Disabled;

impl text_input::StyleSheet for Disabled {
	fn active(&self) -> text_input::Style {
		text_input::Style {
			background: Background::Color(Color::from_rgb8(210, 210, 210)),
			..styles::text_input::Default::active()
		}
	}

	fn focused(&self) -> text_input::Style {
		self.active()
	}

	fn placeholder_color(&self) -> Color {
		styles::text_input::Default::placeholder_color()
	}

	fn value_color(&self) -> Color {
		styles::text_input::Default::value_color()
	}

	fn selection_color(&self) -> Color {
		styles::text_input::Default::selection_color()
	}
}

#[derive(Debug, Clone)]
pub struct NumberInput {
	value: f32,
	min: f32,
	max: f32,

	change: f32,
	component: Component,
	disabled: bool,

	input: text_input::State,
	increment: button::State,
	decrement: button::State,
}

impl NumberInput {
	pub fn new(value: f32, min: f32, max: f32, change: f32, component: Component) -> NumberInput {
		NumberInput {
			value,
			min,
			max,

			change,
			component,
			disabled: false,

			input: text_input::State::default(),
			increment: button::State::default(),
			decrement: button::State::default(),
		}
	}

	pub fn update(&mut self, message: Message) {
		match message {
			Message::Edit(c, s, disabled) => {
				if c == self.component && !disabled {
					if let Some(v) = parse_f32(&s, self.min < 0.0) {
						self.value = v.max(self.min).min(self.max);
					}
				}
			}
			Message::Increment(c, disabled) => {
				if c == self.component && !disabled {
					self.value = (((self.value + self.change) * 1e4).round() / 1e4).max(self.min).min(self.max)
				}
			}
			Message::Decrement(c, disabled) => {
				if c == self.component && !disabled {
					self.value = (((self.value - self.change) * 1e4).round() / 1e4).max(self.min).min(self.max)
				}
			}
			Message::DimensionsEqual(_, disabled) => {
				self.disabled = disabled;
			}
			_ => {}
		}
	}

	pub fn view<'a>(&'a mut self, row: Row<'a, Message>) -> Row<'a, Message> {
		// TODO: Darken text inputs when disabled
		row.push({
			let t = TextInput::new(&mut self.input, "", &format!("{:.4}", self.value), {
				let c = self.component;
				let b = self.disabled;
				move |s| Message::Edit(c, s, b)
			}).padding(16);
			if self.disabled {
				t.style(Disabled)
			} else {
				t
			}
		})
		.push(
			Column::with_children(vec![
				Button::new(&mut self.increment, Text::new("▲"))
					.on_press(Message::Increment(self.component, self.disabled))
					.into(),
				Button::new(&mut self.decrement, Text::new("▼"))
					.on_press(Message::Decrement(self.component, self.disabled))
					.into(),
			])
			.height(Length::Shrink)
			.align_items(Align::Center),
		)
	}

	pub fn get(&self) -> f32 {
		self.value
	}
}

fn parse_f32(str: &String, negative: bool) -> Option<f32> {
	match str.parse::<f32>() {
		Ok(n) => {
			if n.is_nan() || (!negative && n < 0.0) {
				None
			} else {
				Some(n)
			}
		}
		Err(_) => None,
	}
}
