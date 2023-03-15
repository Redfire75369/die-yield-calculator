/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use iced::Length;

pub use calculator::Calculator;

mod calculator;
pub mod components;
mod wafer;

const ROW_HEIGHT: Length = Length::Fixed(42.0);
