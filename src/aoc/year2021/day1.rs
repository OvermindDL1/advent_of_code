use anyhow::Context as _;
use clap::Parser;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file of "depths"
	pub input_file: PathBuf,
}

impl Day1 {
	pub fn run(&self) -> anyhow::Result<()> {
		let mut nums = Vec::with_capacity(8192);
		{
			let mut line = String::with_capacity(16);
			let mut data = BufReader::new(File::open(&self.input_file)?);
			while let Ok(len) = data.read_line(&mut line) {
				if len == 0 {
					break;
				}
				let trimmed = line.trim();
				if !trimmed.is_empty() {
					nums.push(
						trimmed
							.parse::<usize>()
							.with_context(|| format!("Invalid line: {:?}", line))?,
					);
				}
				line.clear();
			}
		}
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
