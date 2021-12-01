use anyhow::Context as _;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day2 {
	/// The input file to use with the parseable type per line
	pub input_file: PathBuf,
}

impl Day2 {
	pub fn run(&self) -> anyhow::Result<()> {
		let mut valid_count_1 = 0;
		let mut valid_count_2 = 0;
		{
			let mut line = String::with_capacity(16);
			let mut data = BufReader::new(File::open(&self.input_file)?);
			while let Ok(len) = data.read_line(&mut line) {
				if len == 0 {
					break;
				}
				let trimmed = line.trim();
				if !trimmed.is_empty() {
					match Self::is_valid(trimmed)
						.with_context(|| format!("failed while processing: {}", trimmed))?
					{
						(true, true) => {
							valid_count_1 += 1;
							valid_count_2 += 1;
						}
						(true, false) => valid_count_1 += 1,
						(false, true) => valid_count_2 += 1,
						(false, false) => (),
					}
				}
				line.clear();
			}
			println!("Step 1: {}", valid_count_1);
			println!("Step 2: {}", valid_count_2);
		}

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
