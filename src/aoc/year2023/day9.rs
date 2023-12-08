use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day9 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 9))]
	pub input: DataFrom,
}

impl Day9 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		Ok((-1, -1))
	}
}

#[cfg(test)]
mod tests {
	crate::run_basic_tests!(
		super::Day8,
		example: (
			"
	",
			(0, 0),
		),
	);
}
