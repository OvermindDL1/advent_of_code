pub mod day1;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2021 {
	/// Advent of Code 2021, Day 1 - Sonar Sweep
	#[clap(name = "1")]
	Day1(day1::Day1),
}

impl Year2021 {
	pub fn run(&self) -> anyhow::Result<()> {
		crate::run_days!(self, [Day1])
	}
}
