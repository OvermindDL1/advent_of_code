use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day13 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 13))]
	pub input: DataFrom,
}

impl Day13 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(i64, i64)> {
		Ok((-1, -1))
	}
}

#[cfg(test)]
mod tests {
	crate::run_basic_tests!(
		super::Day13,
		example: (
			"
	",
			(0, 0),
		),
	);
}
