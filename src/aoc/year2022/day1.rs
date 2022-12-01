use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file of "calories"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 1})]
	pub input: DataFrom,
}

impl Day1 {
	pub fn sort_up(d: &mut [usize; 4]) {
		if d[0] > d[1] {
			d.swap(0, 1);
		}
		if d[1] > d[2] {
			d.swap(1, 2);
		}
		if d[2] > d[3] {
			d.swap(2, 3);
		}
	}

	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut highest = [0; 4];
		process_trimmed_lines_of_file(&self.input, |line| {
			if line.is_empty() {
				Self::sort_up(&mut highest);
				highest[0] = 0;
				return Ok(());
			}
			highest[0] += line.parse::<usize>()?;
			Ok(())
		})?;
		if highest[0] != 0 {
			Self::sort_up(&mut highest);
			highest[0] = 0;
		}
		println!("Step 1: {}", highest[3]);
		println!("Step 2: {}", highest[1..=3].iter().sum::<usize>());
		Ok(())
	}
}
