use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day3 {
	/// The input file of "inventory"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 3})]
	pub input: DataFrom,
}

impl Day3 {
	fn priority_of(v: u8) -> anyhow::Result<u8> {
		Ok(match v {
			b'a'..=b'z' => v - b'a' + 1,
			b'A'..=b'Z' => v - b'A' + 27,
			_ => bail!("invalid value"),
		})
	}

	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let input = self.input.as_cow_u8();
		let lines: Vec<_> = input
			.as_ref()
			.split(|c| *c == b'\n')
			.filter(|line| !line.is_empty())
			.collect();

		let mut score1 = 0;
		for line in lines.iter() {
			let (c0, c1) = line.split_at(line.len() >> 1);
			let found = c0
				.iter()
				.find(|i| c1.contains(i))
				.cloned()
				.context("no duplicate found")?;
			let priority = Self::priority_of(found)? as usize;
			score1 += priority;
		}

		let mut score2 = 0;
		for lines in lines.chunks_exact(3) {
			let [l0, l1, l2] = lines else {
				bail!("invalid row count for badges")
			};
			let badge = l0
				.iter()
				.cloned()
				.find(|c| l1.contains(c) && l2.contains(c))
				.context("no badge found across all 3")?;
			let priority = Self::priority_of(badge)? as usize;
			score2 += priority;
		}

		println!("Step 1: {}", score1);
		println!("Step 2: {}", score2);
		Ok(())
	}
}
