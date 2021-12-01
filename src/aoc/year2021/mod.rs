pub mod day1;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2021 {
	#[clap(name = "1")]
	Day1(day1::Day1),
}

impl Year2021 {
	pub fn run(&self) -> anyhow::Result<()> {
		todo!("preparing for Advent of Code 2020 year 2021!")
	}
}
