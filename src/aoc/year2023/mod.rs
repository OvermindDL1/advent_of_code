pub mod day1;

use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2023 {
	/// Run all the Advent of Code 2023 days
	RunAll,
	/// Advent of Code 2023, Day 1: Trebuchet?!
	#[clap(name = "1")]
	Day1(day1::Day1),
}

impl Year2023 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2023, self, app, [Day1])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2023, app, [Day1])
	}
}
