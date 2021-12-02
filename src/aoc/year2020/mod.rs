pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2020 {
	/// Run all the Advent of Code 2020 days
	RunAll,
	/// Advent of Code 2020, Day 1 - Report Repair
	#[clap(name = "1")]
	Day1(day1::Day1),
	/// Advent of Code 2020, Day 2 - Password Philosophy
	#[clap(name = "2")]
	Day2(day2::Day2),
	/// Advent of Code 2020, Day 3 - Toboggan Trajectory
	#[clap(name = "3")]
	Day3(day3::Day3),
	/// Advent of Code 2020, Day 4 - Passport Processing
	#[clap(name = "4")]
	Day4(day4::Day4),
	/// Advent of Code 2020, Day 5 - Binary Boarding
	#[clap(name = "5")]
	Day5(day5::Day5),
	/// Advent of Code 2020, Day 6 - Custom Customs
	#[clap(name = "6")]
	Day6(day6::Day6),
	/// Advent of Code 2020, Day 7 - Handy Haversacks
	#[clap(name = "7")]
	Day7(day7::Day7),
}

impl Year2020 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(
			Year2020,
			self,
			app,
			[Day1, Day2, Day3, Day4, Day5, Day6, Day7]
		)
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2020, app, [Day1, Day2, Day3, Day4, Day5, Day6, Day7])
	}
}
