use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day5 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 5))]
	pub input: DataFrom,
}

impl Day5 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(i32, i32)> {
		Ok((-1, -1))
	}
}
