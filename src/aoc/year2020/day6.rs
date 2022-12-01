use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;


#[derive(Debug, Parser)]
pub struct Day6 {
	/// The input file to use with the parseable data blank line delimited
	#[clap(default_value_t = DataFrom::Internal {year: 2020, day: 6})]
	pub input: DataFrom,
}

impl Day6 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut current = (0u32, u32::MAX);
		let mut groups = Vec::with_capacity(512);
		process_trimmed_lines_of_file(&self.input, |line| {
			if line.is_empty() {
				groups.push(current);
				current = (0, u32::MAX);
			} else {
				let mut person = 0;
				for b in line.bytes() {
					person |= 1 << (b - b'a');
				}
				current.0 |= person;
				current.1 &= person;
			}
			Ok(())
		})?;
		groups.push(current);
		println!(
			"Step 1: {}",
			groups
				.iter()
				.copied()
				.map(|n| n.0)
				.map(u32::count_ones)
				.sum::<u32>()
		);
		println!(
			"Step 2: {}",
			groups
				.iter()
				.copied()
				.map(|n| n.0 & n.1)
				.map(u32::count_ones)
				.sum::<u32>()
		);

		Ok(())
	}
}
