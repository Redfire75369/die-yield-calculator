/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::{Application, Settings};

use crate::view::Calculator;

pub mod die;
pub mod util;
pub mod view;
pub mod wafer;

fn main() -> iced::Result {
	Calculator::run(Settings {
		antialiasing: true,
		..Settings::default()
	})
}
