pub mod day1;
pub mod day2;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2020 {
	/// Advent of Code 2020, Day 1 - Report Repair
	#[clap(name = "1")]
	Day1(day1::Day1),
	/// Advent of Code 2020, Day 2 - Password Philosophy
	#[clap(name = "2")]
	Day2(day2::Day2),
}

impl Year2020 {
	pub fn run(&self) -> anyhow::Result<()> {
		crate::run_days!(self, [Day1, Day2])
	}
}
