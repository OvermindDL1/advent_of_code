use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day13 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 13})]
	pub input: DataFrom,
}

impl Day13 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		println!("Step 1: {}", -1);
		println!("Step 2: {}", -1);

		Ok(())
	}
}
