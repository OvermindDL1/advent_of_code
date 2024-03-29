use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file of "calories"
	#[clap(default_value_t = DataFrom::internal(2022, 1))]
	pub input: DataFrom,
}

impl Day1 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(usize, usize)> {
		let mut highest = [0; 4];
		process_trimmed_lines_of_file(&self.input, |line| {
			if line.is_empty() {
				highest.sort_unstable();
				highest[0] = 0;
				return Ok(());
			}
			highest[0] += line.parse::<usize>()?;
			Ok(())
		})?;
		if highest[0] != 0 {
			highest.sort_unstable();
			highest[0] = 0;
		}
		Ok((highest[3], highest[1..=3].iter().sum::<usize>()))
	}
}
