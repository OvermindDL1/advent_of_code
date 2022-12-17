use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day17 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 17})]
	pub input: DataFrom,
}

impl Day17 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		println!("Step 1: {}", -1);
		println!("Step 2: {}", -1);

		Ok(())
	}
}
