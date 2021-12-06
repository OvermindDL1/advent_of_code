pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2021 {
	/// Run all the Advent of Code 2021 days
	RunAll,
	/// Advent of Code 2021, Day 1 - Sonar Sweep
	#[clap(name = "1")]
	Day1(day1::Day1),
	/// Advent of Code 2021, Day 2 - Dive!
	#[clap(name = "2")]
	Day2(day2::Day2),
	/// Advent of Code 2021, Day 3 - Binary Diagnostic
	#[clap(name = "3")]
	Day3(day3::Day3),
	/// Advent of Code 2021, Day 4 - Giant Squid
	#[clap(name = "4")]
	Day4(day4::Day4),
}

impl Year2021 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2021, self, app, [Day1, Day2, Day3, Day4])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2021, app, [Day1, Day2, Day3, Day4])
	}
}
