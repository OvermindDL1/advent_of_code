use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::ops::RangeInclusive;

#[derive(Debug, Parser)]
pub struct Day4 {
	/// The input file of "assignments"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 4})]
	pub input: DataFrom,
}

impl Day4 {
	fn parse_part(part: &str) -> anyhow::Result<RangeInclusive<u8>> {
		let (start, end) = part.split_once('-').context("missing `-` in part")?;
		let start: u8 = start.parse()?;
		let end: u8 = end.parse()?;
		Ok(start..=end)
	}

	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut assignments = Vec::default();
		process_trimmed_nonempty_lines_of_file(&self.input, |line| {
			let (first, second) = line.split_once(',').context("not a pair")?;
			let first = Self::parse_part(first)?;
			let second = Self::parse_part(second)?;
			assignments.push([first, second]);
			Ok(())
		})?;

		let mut score1 = 0;
		for [left, right] in assignments.iter() {
			if left.start() >= right.start() && left.end() <= right.end()
				|| right.start() >= left.start() && right.end() <= left.end()
			{
				score1 += 1;
			}
		}

		let mut score2 = 0;
		for [left, right] in assignments.iter() {
			if left.contains(right.start())
				|| left.contains(right.end())
				|| right.contains(left.start())
				|| right.contains(left.end())
			{
				score2 += 1;
			}
		}

		println!("Step 1: {}", score1);
		println!("Step 2: {}", score2);
		Ok(())
	}
}