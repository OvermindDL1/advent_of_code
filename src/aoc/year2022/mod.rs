pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2022 {
	/// Run all the Advent of Code 2022 days
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
	/// Advent of Code 2022, Day 7 - No Space Left On Device
	#[clap(name = "7")]
	Day7(day7::Day7),
	/// Advent of Code 2022, Day 8 - Treetop Tree House
	#[clap(name = "8")]
	Day8(day8::Day8),
	/// Advent of Code 2022, Day 9 - Rope Bridge
	#[clap(name = "9")]
	Day9(day9::Day9),
	/// Advent of Code 2022, Day 10 - Cathode-Ray Tube
	#[clap(name = "10")]
	Day10(day10::Day10),
	/// Advent of Code 2022, Day 11 - Monkey in the Middle
	#[clap(name = "11")]
	Day11(day11::Day11),
	/// Advent of Code 2022, Day 12 - Hill Climbing Algorithm
	#[clap(name = "12")]
	Day12(day12::Day12),
	/// Advent of Code 2022, Day 13 - Distress Signal
	#[clap(name = "13")]
	Day13(day13::Day13),
	/// Advent of Code 2022, Day 14 - Regolith Reservoir
	#[clap(name = "14")]
	Day14(day14::Day14),
	/// Advent of Code 2022, Day 15 - Beacon Exclusion Zone
	#[clap(name = "15")]
	Day15(day15::Day15),
	/// Advent of Code 2022, Day 16 - Proboscidea Volcanium
	#[clap(name = "16")]
	Day16(day16::Day16),
}

impl Year2022 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(
			Year2022,
			self,
			app,
			[
				Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9, Day10, Day11, Day12, Day13,
				Day14, Day15, Day16
			]
		)
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(
			Year2022,
			app,
			[
				Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9, Day10, Day11, Day12, Day13,
				Day14, Day15, Day16
			]
		)
	}
}
