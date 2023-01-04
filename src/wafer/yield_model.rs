/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YieldModel {
	Poisson,
	Murphy,
}

impl YieldModel {
	pub const ALL: &'static [YieldModel] = &[YieldModel::Poisson, YieldModel::Murphy];
}

impl Display for YieldModel {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			YieldModel::Poisson => f.write_str("Poisson's Model"),
			YieldModel::Murphy => f.write_str("Murphy's Model"),
		}
	}
}
