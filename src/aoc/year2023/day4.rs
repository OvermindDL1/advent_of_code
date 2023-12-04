use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::collections::VecDeque;

#[derive(Debug, Parser)]
pub struct Day4 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 4))]
	pub input: DataFrom,
}

impl Day4 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(i32, i32)> {
		let mut extras = VecDeque::with_capacity(128);
		let mut score1 = 0;
		let mut score2 = 0;
		process_trimmed_nonempty_lines_of_file(&self.input, |mut line| {
			line = line
				.strip_prefix("Card ")
				.context("missing `Card ` prefix")?;
			let (_card_number, line) = line.trim().split_once(':').context("missing `:`")?;
			// let card_number = card_number.parse::<u32>().context("failed to parse card number")?;
			let (winnings, testings) = line.split_once(" |").context("missing ` |`")?;
			let mut score = 0;
			let mut count = 0;
			for testing in testings.as_bytes().chunks_exact(3) {
				for winning in winnings.as_bytes().chunks_exact(3) {
					if testing == winning {
						count += 1;
						if score == 0 {
							score = 1;
						} else {
							score *= 2;
						}
					}
				}
			}
			score1 += score;
			let cur_count = 1 + extras.pop_front().unwrap_or_default();
			if extras.len() < count {
				extras.extend(std::iter::repeat(0).take(count - extras.len()));
			}
			extras
				.iter_mut()
				.take(count)
				.for_each(|extra| *extra += cur_count);
			score2 += cur_count;
			Ok(())
		})?;

		Ok((score1, score2))
	}
}
