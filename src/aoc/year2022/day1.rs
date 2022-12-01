use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file of "calories"
	#[clap(default_value = "inputs/2022/day1.input")]
	pub input_file: PathBuf,
}

impl Day1 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut highest = [0; 4];
		process_trimmed_lines_of_file(&self.input_file, |line| {
			if line.is_empty() {
				highest.sort();
				highest[0] = 0;
				return Ok(());
			}
			highest[0] += line.parse::<usize>()?;
			Ok(())
		})?;
		if highest[0] != 0 {
			highest.sort();
			highest[0] = 0;
		}
		println!("Step 1: {}", highest[3]);
		println!("Step 2: {}", highest[1..=3].iter().sum::<usize>());
		Ok(())
	}
}
