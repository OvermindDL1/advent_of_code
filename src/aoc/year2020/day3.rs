use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day3 {
	/// The input file to use with the parseable data blank line delimited
	#[clap(default_value = "inputs/2020/day3.input")]
	pub input_file: PathBuf,
}

impl Day3 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let map = map_trimmed_nonempty_lines_of_file(&self.input_file, |line| {
			Ok(line
				.chars()
				.map(|c| match c {
					'.' => false,
					'#' => true,
					_ => panic!("Unexpected character {}", c),
				})
				.collect::<Vec<_>>())
		})?;

		println!(
			"Step 1: {}",
			map.iter()
				.enumerate()
				.filter(|tree| tree.1[(tree.0 * 3) % tree.1.len()])
				.count()
		);

		let mut answer = 1;
		for (right, down) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
			answer *= map
				.iter()
				.enumerate()
				.filter(|t| (t.0 % down) == 0)
				.filter(|tree| tree.1[((tree.0 / down) * right) % tree.1.len()])
				.count();
		}
		println!("Step 2: {}", answer);

		Ok(())
	}
}
