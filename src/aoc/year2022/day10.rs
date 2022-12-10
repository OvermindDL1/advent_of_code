use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day10 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 10})]
	pub input: DataFrom,
}

impl Day10 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let score0 = 0;
		let score1 = 0;

		println!("Step 1: {}", score0);
		println!("Step 2: {}", score1);
		Ok(())
	}
}
