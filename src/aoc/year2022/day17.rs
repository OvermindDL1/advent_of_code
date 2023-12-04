use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day17 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2022, 17))]
	pub input: DataFrom,
}

impl Day17 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(usize, usize)> {
		Ok((0, 0))
	}
}
