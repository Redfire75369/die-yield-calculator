/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fmt::{Display, Formatter};

use crate::wafer::Wafer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YieldModel {
	Poisson,
	Murphy,
	Rectangular,
	Moore,
	Seeds,
}

impl YieldModel {
	pub const ALL: &'static [YieldModel] = &[
		YieldModel::Poisson,
		YieldModel::Murphy,
		YieldModel::Rectangular,
		YieldModel::Moore,
		YieldModel::Seeds,
	];

	pub fn wafer_yield(self, wafer: &Wafer) -> f32 {
		let defects = wafer.critical_area * wafer.defect_rate / 100.0;
		match self {
			YieldModel::Poisson => (-defects).exp(),
			YieldModel::Murphy => ((1.0 - (-defects).exp()) / (defects)).powi(2),
			YieldModel::Rectangular => (1.0 - (-2.0 * defects).exp()) / (2.0 * defects),
			YieldModel::Moore => (-defects.sqrt()).exp(),
			YieldModel::Seeds => 1.0 / (1.0 + defects),
		}
	}
}

impl Display for YieldModel {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			YieldModel::Poisson => f.write_str("Poisson Model"),
			YieldModel::Murphy => f.write_str("Murphy's Model"),
			YieldModel::Rectangular => f.write_str("Rectangular Model"),
			YieldModel::Moore => f.write_str("Moore's Model"),
			YieldModel::Seeds => f.write_str("Seeds Model"),
		}
	}
}
