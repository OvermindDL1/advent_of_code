pub mod day1;

use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2022 {
	/// Run all the Advent of Code 2021 days
	RunAll,
	/// Advent of Code 2022, Day 1 - Calorie Counting
	#[clap(name = "1")]
	Day1(day1::Day1),
}

impl Year2022 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2022, self, app, [Day1])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2022, app, [Day1])
	}
}
