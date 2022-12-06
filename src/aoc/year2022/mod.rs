pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2022 {
	/// Run all the Advent of Code 2021 days
	RunAll,
	/// Advent of Code 2022, Day 1 - Calorie Counting
	#[clap(name = "1")]
	Day1(day1::Day1),
	/// Advent of Code 2022, Day 2 - Rock Paper Scissors
	#[clap(name = "2")]
	Day2(day2::Day2),
	/// Advent of Code 2022, Day 3 - Rucksack Reorganization
	#[clap(name = "3")]
	Day3(day3::Day3),
	/// Advent of Code 2022, Day 4 - Camp Cleanup
	#[clap(name = "4")]
	Day4(day4::Day4),
	/// Advent of Code 2022, Day 5 - Supply Stacks
	#[clap(name = "5")]
	Day5(day5::Day5),
	/// Advent of Code 2022, Day 6 - Tuning Trouble
	#[clap(name = "6")]
	Day6(day6::Day6),
}

impl Year2022 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2022, self, app, [Day1, Day2, Day3, Day4, Day5, Day6])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2022, app, [Day1, Day2, Day3, Day4, Day5, Day6])
	}
}
