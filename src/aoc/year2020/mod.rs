pub mod day1;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2020 {
	/// Advent of Code 2020, Day 1
	#[clap(name = "1")]
	Day1(day1::Day1),
}

impl Year2020 {
	pub fn run(&self) -> anyhow::Result<()> {
		crate::run_days!(self, [Day1,])
	}
}
