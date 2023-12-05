use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day6 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 6))]
	pub input: DataFrom,
}

impl Day6 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(i32, i32)> {
		Ok((-1, -1))
	}
}
