use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use itertools::Itertools;

use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Day5 {
	/// The input file to use with the parseable seat data
	#[clap(default_value_t = DataFrom::internal(2020, 5))]
	pub input: DataFrom,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Seat(u16);

impl FromStr for Seat {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() != 10 {
			anyhow::bail!("Invalid seat length: {}", s.len());
		}
		let mut seat = 0u16;
		for (i, b) in s.as_bytes()[..7].iter().copied().enumerate() {
			match b {
				b'F' => (),
				b'B' => seat |= 1 << (9 - i),
				_ => anyhow::bail!("Invalid row letter: {}", b as char),
			}
		}
		for (i, b) in &mut s.as_bytes()[7..].iter().copied().enumerate() {
			match b {
				b'L' => (),
				b'R' => seat |= 1 << (2 - i),
				_ => anyhow::bail!("Invalid seat letter: {}", b as char),
			}
		}

		Ok(Seat(seat))
	}
}

impl Day5 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u16, u16)> {
		let mut seats: Vec<Seat> = map_trimmed_nonempty_lines_of_file(&self.input, str::parse)?;
		seats.sort_unstable();
		let score1 = seats.last().context("no seats")?.0;
		let score2 = seats
			.iter()
			.map(|s| s.0)
			.tuple_windows()
			.find_map(|(a, b)| if b - a > 1 { Some(a + 1) } else { None })
			.context("did not find a missing seat")?;

		Ok((score1, score2))
	}
}
