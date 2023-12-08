use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;
use indexmap::IndexMap;
use itertools::Itertools;
use num::integer::Integer;

#[derive(Debug, Parser)]
pub struct Day8 {
	/// The input file of "map and direction" data
	#[clap(default_value_t = DataFrom::internal(2023, 8))]
	pub input: DataFrom,
}

impl Day8 {
	#[allow(clippy::too_many_lines)]
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		let input = self.input.as_cow_str()?;
		let (directions, map_lines) = input
			.split_once("\n\n")
			.with_context(|| "Failed to split input into directions and map from:\n{input}")?;
		let directions = directions.trim().as_bytes();

		let mut map = IndexMap::with_capacity(1024);
		for line in map_lines.trim().lines() {
			let line = line.trim();
			match *line.as_bytes() {
				[k0, k1, k2, b' ', b'=', b' ', b'(', l0, l1, l2, b',', b' ', r0, r1, r2, b')'] => {
					map.insert([k0, k1, k2], ([l0, l1, l2], [r0, r1, r2]));
				}
				ref invalid => bail!("invalid map line: {invalid:?}"),
			}
		}
		let map: IndexMap<_, _> = map
			.iter()
			.map(|(k, (l, r))| {
				Ok((
					k.to_owned(),
					(
						map.get_index_of(l).with_context(|| {
							format!("missing left key of {:?}", std::str::from_utf8(l))
						})?,
						map.get_index_of(r).with_context(|| {
							format!("missing right key of {:?}", std::str::from_utf8(l))
						})?,
					),
				))
			})
			.collect::<anyhow::Result<_>>()?;

		let mut score1 = 0;
		// let mut current = *b"AAA";
		let mut current = map.get_index_of(b"AAA").context("invalid map key: AAA")?;
		let destination = map.get_index_of(b"ZZZ").context("invalid map key: AAA")?;
		for dir in directions.iter().copied().cycle() {
			score1 += 1;
			let entry = map.get_index(current).with_context(|| {
				format!(
					"invalid map key: {:?}",
					map.get_index(current).map(|k| std::str::from_utf8(k.0))
				)
			})?;
			current = match dir {
				b'L' => entry.1 .0,
				b'R' => entry.1 .1,
				_ => bail!("invalid direction: {dir}"),
			};
			if current == destination {
				break;
			}
		}

		let mut currents: Vec<_> = map
			.keys()
			.enumerate()
			.filter(|(_idx, k)| k[2] == b'A')
			.map(|(idx, _k)| idx)
			.collect();
		let mut cycles = Vec::with_capacity(currents.len());
		let mut counts = Vec::with_capacity(2);
		for current in &mut currents {
			let mut count = 0u64;
			let mut first_match = None;
			let mut dir_iter = directions.iter().copied().cycle();
			while first_match != Some(*current) || counts.len() <= 1 {
				for dir in &mut dir_iter {
					count += 1;
					let (_, entry) = map.get_index(*current).with_context(|| {
						format!(
							"invalid map key: {:?}",
							map.get_index(*current).map(|k| std::str::from_utf8(k.0))
						)
					})?;
					*current = match dir {
						b'L' => entry.0,
						b'R' => entry.1,
						_ => bail!("invalid direction: {dir}"),
					};
					if map.get_index(*current).context("missing known key")?.0[2] == b'Z' {
						if first_match.is_none() {
							first_match = Some(*current);
						}
						counts.push(count);
						count = 0;
						break;
					}
				}
			}
			if counts.iter().copied().dedup().count() != 1 {
				// Somehow this input problem has them all be the same length, from the initial run to
				// every Z cycle, which makes this mathy-easy, but if input is fed into this that
				// doesn't meet this requirement then bail out now.
				bail!("Invalid input, unhandled differing stride length: {counts:?}");
			}
			cycles.push(counts.first().copied().context("counts is empty")?);
			counts.clear();
		}

		let score2 = cycles
			.iter()
			.copied()
			.reduce(|a, b| a.lcm(&b))
			.context("cycles is empty")?;

		// dbg!(currents
		// 	.iter()
		// 	.map(|k| std::str::from_utf8(k).unwrap())
		// 	.join(","));
		// let mut cycler = HashMap::with_capacity(1024);
		// for current in &mut currents {
		// 	let mut count = 0u64;
		// 	let mut current_cycles = Vec::with_capacity(8);
		// 	for dir in directions.iter().copied().cycle() {
		// 		count += 1;
		// 		let entry = map.get(current).with_context(|| {
		// 			format!("Invalid map key: {:?}", std::str::from_utf8(current))
		// 		})?;
		// 		*current = match dir {
		// 			b'L' => entry.0,
		// 			b'R' => entry.1,
		// 			_ => bail!("Invalid direction: {dir}"),
		// 		};
		// 		if current[2] == b'Z' {
		// 			if current_cycles.is_empty() {
		// 				current_cycles.push(count);
		// 			}
		// 		}
		//
		// 		if current[2] == b'Z' {
		// 			if let Some(entry) = cycler.get_mut(&next) {
		// 				// *entry = count;
		// 				cycles.push((*entry, count - *entry));
		// 				cycler.clear();
		// 				break;
		// 			}
		// 		}
		// 		cycler.insert(next, count);
		// 		*current = next;
		// 	}
		// }
		//
		// let mut score2 = 0;
		// let mut remaining: Vec<_> = cycles
		// 	.iter()
		// 	.copied()
		// 	.map(|(start, cycle)| start + cycle)
		// 	.collect();
		// loop {
		// 	let min = remaining
		// 		.iter()
		// 		.copied()
		// 		.min()
		// 		.context("remaining is empty")?;
		// 	remaining.iter_mut().for_each(|r| *r -= min);
		// 	score2 += min;
		// 	dbg!((score2, &remaining, &cycles));
		// 	if remaining.iter().copied().all(|r| r == 0) {
		// 		break;
		// 	}
		// 	remaining
		// 		.iter_mut()
		// 		.enumerate()
		// 		.filter(|(_, r)| **r == 0)
		// 		.for_each(|(i, r)| {
		// 			*r = cycles[i].1;
		// 		});
		// }

		Ok((score1, score2))
	}
}

#[cfg(test)]
mod tests {
	crate::run_basic_tests!(
		super::Day8,
		simple_example: (
			"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
",
			(6, 6),
		),
		full_example: (
			"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
",
			(2, 2),
		),
		score2_example: ("LR

	AAA = (11B, XXX)
	11B = (XXX, ZZZ)
	ZZZ = (11B, XXX)
	22A = (22B, XXX)
	22B = (22C, 22C)
	22C = (22Z, 22Z)
	22Z = (22B, 22B)
	XXX = (XXX, XXX)
	", (0, 6)),
	);
}
