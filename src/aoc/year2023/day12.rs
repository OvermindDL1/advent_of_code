use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day12 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 12))]
	pub input: DataFrom,
}

impl Day12 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		Ok((-1, -1))
	}
}

#[cfg(test)]
mod tests {
	crate::run_basic_tests!(
		super::Day12,
		example: (
			"
	",
			(0, 0),
		),
	);
}
