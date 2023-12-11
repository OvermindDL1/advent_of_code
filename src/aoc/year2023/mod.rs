pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
pub use day5::Day5;
pub use day6::Day6;
pub use day7::Day7;
pub use day8::Day8;
pub use day9::Day9;

use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2023 {
	/// Run all the Advent of Code 2023 days
	RunAll,
	/// Advent of Code 2023, Day 1: Trebuchet?!
	#[clap(name = "1")]
	Day1(Day1),
	/// Advent of Code 2023, Day 2: Cube Conundrum
	#[clap(name = "2")]
	Day2(Day2),
	/// Advent of Code 2023, Day 3: Gear Ratios
	#[clap(name = "3")]
	Day3(Day3),
	/// Advent of Code 2023, Day 4: Scratchcards
	#[clap(name = "4")]
	Day4(Day4),
	/// Advent of Code 2023, Day 5: If You Give A Seed A Fertilizer
	#[clap(name = "5")]
	Day5(Day5),
	/// Advent of Code 2023, Day 6: Wait For It
	#[clap(name = "6")]
	Day6(Day6),
	/// Advent of Code 2023, Day 7: Camel Cards
	#[clap(name = "7")]
	Day7(Day7),
	/// Advent of Code 2023, Day 8: Haunted Wasteland
	#[clap(name = "8")]
	Day8(Day8),
	/// Advent of Code 2023, Day 9: Mirage Maintenance
	#[clap(name = "9")]
	Day9(Day9),
}

impl Year2023 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(
			Year2023,
			self,
			app,
			[Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9]
		)
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(
			Year2023,
			app,
			[Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9]
		)
	}
}
