use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;
use itertools::Itertools;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file of "depths"
	#[clap(default_value_t = DataFrom::Internal {year: 2021, day: 1})]
	pub input: DataFrom,
}

impl Day1 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let nums =
			map_trimmed_nonempty_lines_of_file(&self.input, |line| Ok(line.parse::<usize>()?))?;
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
