/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub mod text_input {
	use iced::{text_input, Color, Background};

	pub struct Default;

	impl Default {
		pub fn active() -> text_input::Style {
			text_input::Style {
				background: Background::Color(Color::WHITE),
				border_radius: 5.0,
				border_width: 1.0,
				border_color: Color::from_rgb(0.7, 0.7, 0.7),
			}
		}

		pub fn focused() -> text_input::Style {
			text_input::Style {
				background: Background::Color(Color::WHITE),
				border_radius: 5.0,
				border_width: 1.0,
				border_color: Color::from_rgb(0.5, 0.5, 0.5),
			}
		}

		pub fn placeholder_color() -> Color {
			Color::from_rgb(0.7, 0.7, 0.7)
		}

		pub fn value_color() -> Color {
			Color::from_rgb(0.3, 0.3, 0.3)
		}

		pub fn selection_color() -> Color {
			Color::from_rgb(0.8, 0.8, 1.0)
		}
	}
}
