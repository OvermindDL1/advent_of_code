use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day4 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 4))]
	pub input: DataFrom,
}

impl Day4 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(usize, usize)> {
		Ok((0, 0))
	}
}
