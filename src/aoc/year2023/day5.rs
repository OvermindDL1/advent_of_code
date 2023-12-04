use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day5 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::Internal {year: 2023, day: 5})]
	pub input: DataFrom,
}

impl Day5 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		println!("Step 1: {}", -1);
		println!("Step 2: {}", -1);

		Ok(())
	}
}
