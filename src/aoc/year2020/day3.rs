use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::bail;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day3 {
	#[clap(default_value_t = DataFrom::Internal {year: 2020, day: 3})]
	pub input: DataFrom,
}

impl Day3 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let map = map_trimmed_nonempty_lines_of_file(&self.input, |line| {
			line.chars()
				.map(|c| {
					Ok(match c {
						'.' => false,
						'#' => true,
						_ => bail!("Unexpected character {c}"),
					})
				})
				.collect::<anyhow::Result<Vec<_>>>()
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
		println!("Step 2: {answer}");

		Ok(())
	}
}
