use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context as _};
use clap::Parser;
use std::ops::RangeInclusive;

#[derive(Debug, Parser)]
pub struct Day6 {
	/// The input file of "race data"
	#[clap(default_value_t = DataFrom::internal(2023, 6))]
	pub input: DataFrom,
}

impl Day6 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		let input = self.input.as_cow_str()?;
		let (time_line, dist_line) = input
			.split_once('\n')
			.context("input must have two lines")?;
		let times_iter = time_line
			.trim()
			.strip_prefix("Time:")
			.with_context(|| format!("time line must start with \"Time: \": {time_line}"))?
			.trim()
			.split_ascii_whitespace()
			.map(str::parse);
		let distances_iter = dist_line
			.trim()
			.strip_prefix("Distance:")
			.with_context(|| format!("distance line must start with \"Distance: \": {dist_line}"))?
			.trim()
			.split_ascii_whitespace()
			.map(str::parse);
		let races = times_iter
			.zip(distances_iter)
			.map(|(time, record_distance)| {
				Ok(Race {
					time: time?,
					record_distance: record_distance?,
				})
			})
			.collect::<anyhow::Result<Vec<_>>>()?;

		let mut score1 = 1;
		for race in &races {
			let winning_range = race.get_winning_range()?;
			score1 *= winning_range.count() as u64;
		}

		let full_time = time_line
			.bytes()
			.filter(u8::is_ascii_digit)
			.fold(0, |acc, b| acc * 10 + (b - b'0') as u64);
		let full_distance = dist_line
			.bytes()
			.filter(u8::is_ascii_digit)
			.fold(0, |acc, b| acc * 10 + (b - b'0') as u64);
		let full_race = Race {
			time: full_time,
			record_distance: full_distance,
		};

		let score2 = full_race.get_winning_range()?.count() as u64;

		Ok((score1, score2))
	}
}

#[derive(Debug)]
struct Race {
	time: u64,
	record_distance: u64,
}

impl Race {
	fn is_record_when(&self, held: u64) -> bool {
		let move_time = self.time - held;
		let move_distance = move_time * held;
		move_distance > self.record_distance
	}

	// Unoptimized, literally checking along the number range, there's probably math for this
	// but this already runs in a few nanoseconds even in debug build so I'm not gonna bother
	fn get_winning_range(&self) -> anyhow::Result<RangeInclusive<u64>> {
		if self.time < u16::MAX as _ {
			let low = (0..self.time)
				.find(|&held| self.is_record_when(held))
				.with_context(|| format!("found no low winning range for {self:?}"))?;
			let high = (0..self.time)
				.rfind(|&held| self.is_record_when(held))
				.with_context(|| {
					format!("found no high winning range for {self:?} (low = {low})",)
				})?;
			return Ok(low..=high);
		}

		let mut low = 0;
		let mut high = self.time;
		let mut mid = low + (high - low) / 2;
		if !self.is_record_when(mid) {
			bail!("mid point is not in the winning range for {self:?} (mid = {mid})");
		}

		// get low bound
		let low = if self.is_record_when(low) {
			low
		} else {
			loop {
				if mid == low {
					break mid + 1;
				} else if self.is_record_when(mid) {
					if mid == low + 1 {
						break mid;
					}
					mid -= (mid - low) / 2;
				} else {
					low = mid;
					mid += (high - mid) / 2;
				}
			}
		};

		// get high bound
		let high = if self.is_record_when(high) {
			high
		} else {
			loop {
				if mid == high {
					break mid - 1;
				} else if self.is_record_when(mid) {
					if mid == high - 1 {
						break mid;
					}
					mid += (high - mid) / 2;
				} else {
					high = mid;
					mid -= (mid - low) / 2;
				}
			}
		};

		Ok(low..=high)
	}
}
