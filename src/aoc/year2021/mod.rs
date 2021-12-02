pub mod day1;
pub mod day2;

use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2021 {
	/// Advent of Code 2021, Day 1 - Sonar Sweep
	#[clap(name = "1")]
	Day1(day1::Day1),
	/// Advent of Code 2021, Day 2 - Dive!
	#[clap(name = "2")]
	Day2(day2::Day2),
}

impl Year2021 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2021, self, app, [Day1, Day2])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2021, app, [Day1, Day2])
	}
}
