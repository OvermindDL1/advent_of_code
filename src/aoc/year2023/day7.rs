use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day7 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 7))]
	pub input: DataFrom,
}

impl Day7 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		Ok((-1, -1))
	}
}
