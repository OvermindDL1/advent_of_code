use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::borrow::Cow;

#[derive(Debug, Parser)]
pub struct Day6 {
	/// The input file of
	#[clap(default_value_t = DataFrom::Internal {year: 2021, day: 6})]
	pub input: DataFrom,
}

#[derive(Debug, Default)]
struct LifeStateCounts([usize; 9]);

impl LifeStateCounts {
	fn tick(&mut self) {
		let rollover = self.0[0];
		self.0[0] = self.0[1];
		self.0[1] = self.0[2];
		self.0[2] = self.0[3];
		self.0[3] = self.0[4];
		self.0[4] = self.0[5];
		self.0[5] = self.0[6];
		self.0[6] = self.0[7] + rollover;
		self.0[7] = self.0[8];
		self.0[8] = rollover;
	}

	fn sum(&self) -> usize {
		self.0.iter().sum()
	}
}

impl Day6 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut counts = LifeStateCounts::default();
		process_trimmed_nonempty_lines_of_file(&self.input, |line| {
			for num in line.split(',') {
				let num: usize = num
					.parse()
					.with_context(|| format!("invalid number: {}", num))?;
				if num >= counts.0.len() {
					anyhow::bail!(
						"number of lifetime is too large (max {}): {}",
						counts.0.len(),
						num
					);
				}
				counts.0[num] += 1;
			}
			Ok(())
		})?;

		(0..80).for_each(|_| counts.tick());
		println!("Step 1: {:?}", counts.sum());
		(80..256).for_each(|_| counts.tick());
		println!("Step 2: {:?}", counts.sum());

		Ok(())
	}
}
