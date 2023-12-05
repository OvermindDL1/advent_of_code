#![allow(clippy::range_minus_one, clippy::reversed_empty_ranges)]

use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;
use itertools::Itertools;
use std::mem::swap;
use std::ops::RangeInclusive;

#[derive(Debug, Parser)]
pub struct Day5 {
	/// The input file of "almanac"
	#[clap(default_value_t = DataFrom::internal(2023, 5))]
	pub input: DataFrom,
}

impl Day5 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		let input = self.input.as_cow_str()?;
		let input = input.trim();
		let mut seeds = Vec::with_capacity(32);
		let mut seed_ranges = Vec::with_capacity(16);
		let mut mappings = Vec::with_capacity(8);

		let mut sections = input.split("\n\n");

		// Seeds
		{
			let mut seeds_line = sections
				.next()
				.with_context(|| format!("input appears empty: {input}"))?;
			seeds_line = seeds_line
				.strip_prefix("seeds: ")
				.with_context(|| format!("missing seeds line on: {seeds_line}"))?;
			for seed in seeds_line.split_ascii_whitespace() {
				let seed = seed.parse::<u64>().context("invalid seed")?;
				seeds.push(seed);
			}
			for (&seed_start, &seed_length) in seeds.iter().tuples() {
				seed_ranges.push(seed_start..=seed_start + seed_length);
			}
		}

		// Sections
		for section in sections {
			let mut lines = section.split('\n');
			let mut title_line = lines
				.next()
				.with_context(|| format!("missing title line on: {section}"))?;
			title_line = title_line
				.strip_suffix(" map:")
				.with_context(|| format!("missing ` map:` suffix on: {title_line}"))?;
			let (from, to) = title_line
				.split_once("-to-")
				.with_context(|| format!("missing `-to-` separator on: {title_line}"))?;

			let mut redirects = Vec::with_capacity(32);
			for entry_line in lines {
				let mut entry = entry_line.split_ascii_whitespace();
				let dest = entry
					.next()
					.with_context(|| format!("missing `from` on: {entry_line}"))?;
				let source = entry
					.next()
					.with_context(|| format!("missing `to` on: {entry_line}"))?;
				let length = entry
					.next()
					.with_context(|| format!("missing `length` on: {entry_line}"))?;
				if let Some(rem) = entry.next() {
					bail!("unexpected extra characters on: {rem}");
				}
				let dest = dest
					.parse::<u64>()
					.with_context(|| format!("invalid `from`: {from}"))?;
				let source = source
					.parse::<u64>()
					.with_context(|| format!("invalid `to`: {to}"))?;
				let length = length
					.parse::<u64>()
					.with_context(|| format!("invalid `length`: {length}"))?;
				redirects.push(Redirect {
					dest: dest..=dest + length - 1,
					source: source..=source + length - 1,
				});
			}
			redirects.sort_by_key(|r| *r.source.start());
			mappings.push(Mapping {
				from,
				to,
				redirects,
			});
		}

		let mut score1 = u64::MAX;
		for &seed in &seeds {
			let mut id = seed;
			for mapping in &mappings {
				id = mapping.map_id(id);
			}
			score1 = score1.min(id);
		}

		let mut score2 = u64::MAX;
		let mut id_ranges = Vec::with_capacity(32);
		let mut id_ranges_next = Vec::with_capacity(32);
		for seed_range in seed_ranges {
			id_ranges.clear();
			id_ranges.push(seed_range);
			for mapping in &mappings {
				// compact_ranges(&mut id_ranges);
				for id_range in id_ranges.iter().cloned() {
					mapping.map_id_range(id_range, &mut id_ranges_next);
				}
				swap(&mut id_ranges, &mut id_ranges_next);
				id_ranges_next.clear();
			}
			let lowest = id_ranges
				.iter()
				.min_by_key(|r| r.start())
				.context("no lowest id")?;
			score2 = score2.min(*lowest.start());
		}

		Ok((score1, score2))
	}
}

// The input is not actually complicated enough to make this worth it, so leaving it out
// fn compact_ranges(ranges: &mut Vec<RangeInclusive<u64>>) {
// 	ranges.sort_by_key(|r| *r.start());
// 	let mut i = ranges.len() - 1;
// 	while i > 0 {
// 		let right = &ranges[i].clone();
// 		let left = &mut ranges[i - 1];
// 		if right.start() < left.end() {
// 			if right.end() > left.end() {
// 				*left = *left.start()..=*right.end();
// 			}
// 			ranges.remove(i);
// 		}
// 		i -= 1;
// 	}
// }

#[allow(dead_code)]
#[derive(Debug)]
struct Mapping<'s> {
	from: &'s str,
	to: &'s str,
	redirects: Vec<Redirect>,
}
impl<'s> Mapping<'s> {
	pub fn map_id(&self, id: u64) -> u64 {
		for redirect in &self.redirects {
			if let Some(dest) = redirect.map_id(id) {
				return dest;
			}
		}
		id
	}
	fn map_id_range(&self, mut id: RangeInclusive<u64>, out: &mut Vec<RangeInclusive<u64>>) {
		for redirect in &self.redirects {
			redirect.map_id_range(&mut id, out);
			if id.is_empty() {
				return;
			}
		}
		out.push(id);
	}
}

#[derive(Debug)]
struct Redirect {
	source: RangeInclusive<u64>,
	dest: RangeInclusive<u64>,
}

impl Redirect {
	pub fn map_id(&self, seed: u64) -> Option<u64> {
		if self.source.contains(&seed) {
			let offset = seed - self.source.start();
			let dest = self.dest.start() + offset;
			Some(dest)
		} else {
			None
		}
	}
	pub fn map_id_range(&self, id: &mut RangeInclusive<u64>, out: &mut Vec<RangeInclusive<u64>>) {
		if id.start() < self.source.start() {
			if id.end() < self.source.start() {
				out.push(id.clone());
				*id = u64::MAX..=u64::MAX - 1;
			} else {
				out.push(*id.start()..=*self.source.start() - 1);
				*id = *self.source.start()..=*id.end();
			}
		}
		if id.start() < self.source.end() {
			if id.end() < self.source.end() {
				out.push(
					*self.dest.start() + id.start() - self.source.start()
						..=*self.dest.start() + id.end() - self.source.start(),
				);
				*id = u64::MAX..=u64::MAX - 1;
			} else {
				out.push(
					*self.dest.start() + id.start() - self.source.start()
						..=*self.dest.start() + self.source.end() - self.source.start(),
				);
				*id = *self.source.end() + 1..=*id.end();
			}
		}
	}
}
