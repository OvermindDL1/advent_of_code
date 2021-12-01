use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context as _;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day2 {
	/// The input file to use with the parseable type per line
	#[clap(default_value = "inputs/2020/day2.input")]
	pub input_file: PathBuf,
}

impl Day2 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut valid_count_1 = 0;
		let mut valid_count_2 = 0;
		process_trimmed_nonempty_lines_of_file(&self.input_file, |line| {
			match Self::is_valid(line)? {
				(true, true) => {
					valid_count_1 += 1;
					valid_count_2 += 1;
				}
				(true, false) => valid_count_1 += 1,
				(false, true) => valid_count_2 += 1,
				(false, false) => (),
			}
			Ok(())
		})?;

		println!("Step 1: {}", valid_count_1);
		println!("Step 2: {}", valid_count_2);

		Ok(())
	}

	fn is_valid(line: &str) -> anyhow::Result<(bool, bool)> {
		let mut valid_1 = true;
		let mut valid_2 = true;
		let (low, rest) = line.split_once('-').context("missing `-`")?;
		let (high, rest) = rest.split_once(' ').context("missing ` `")?;
		let (c, rest) = rest.split_once(':').context("missing `:`")?;
		let low = low.parse::<usize>()?;
		let high = high.parse::<usize>()?;
		let mut cc = c.chars();
		let c = cc.next().context("missing check character")?;
		if cc.next().is_some() {
			anyhow::bail!("too many characters");
		}
		let password = rest.trim();
		let mut count = 0;
		for _ in password.chars().filter(|cc| cc == &c) {
			count += 1;
			if count > high {
				valid_1 = false;
				break;
			}
		}
		if count < low {
			valid_1 = false;
		}
		let low = low - 1;
		let high = high - 1;
		let mut chars = password.chars().enumerate();
		while let Some((i, cc)) = chars.next() {
			if i == low {
				valid_2 = cc == c;
				break;
			}
		}
		while let Some((i, cc)) = chars.next() {
			if i == high {
				return Ok((valid_1, valid_2 != (cc == c)));
			}
		}
		anyhow::bail!("ran out of input as the high value went past the password length")
	}
}
