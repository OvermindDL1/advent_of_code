use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day2 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::Internal {year: 2023, day: 2})]
	pub input: DataFrom,
}

impl Day2 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		println!("Step 1: {}", -1);
		println!("Step 2: {}", -1);

		Ok(())
	}
}
