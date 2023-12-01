use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day6 {
	/// The input file of "signal data"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 6})]
	pub input: DataFrom,
}

impl Day6 {
	fn find_start_of_window(input: &str, window_size: usize) -> anyhow::Result<usize> {
		Ok(window_size
			+ input
				.as_bytes()
				.windows(window_size)
				.position(|vs| {
					vs.iter()
						.enumerate()
						.all(|(i, v)| !vs[(i + 1)..].contains(v))
				})
				.context("no window start found")?)
	}

	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let input = self.input.as_cow_str()?;
		let input = input.as_ref().trim();

		let score1 = Self::find_start_of_window(input, 4)?;
		let score2 = Self::find_start_of_window(input, 14)?;

		println!("Step 1: {score1}");
		println!("Step 2: {score2}");
		Ok(())
	}
}
