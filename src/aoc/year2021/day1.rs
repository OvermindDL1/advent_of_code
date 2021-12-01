use crate::aoc::helpers::*;
use clap::Parser;
use itertools::Itertools;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file of "depths"
	#[clap(default_value = "inputs/2021/day1.input")]
	pub input_file: PathBuf,
}

impl Day1 {
	pub fn run(&self) -> anyhow::Result<()> {
		let nums =
			map_trimmed_nonempty_lines_of_file(
				&self.input_file,
				|line| Ok(line.parse::<usize>()?),
			)?;
		println!(
			"Step 1: {}",
			nums.iter()
				.tuple_windows()
				.map(|(a, b)| a < b)
				.filter(|&x| x)
				.count()
		);
		println!(
			"Step 2: {}",
			nums.iter()
				.tuple_windows()
				.map(|(a, b, c)| a + b + c)
				.tuple_windows()
				.map(|(a, b)| a < b)
				.filter(|&x| x)
				.count()
		);

		Ok(())
	}
}
