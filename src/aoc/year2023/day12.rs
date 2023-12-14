use crate::aoc::helpers::*;
use crate::AocApp;
use ahash::AHashMap;
use anyhow::{bail, Context};
use clap::Parser;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Parser)]
pub struct Day12 {
	/// The input file of "spring statuses"
	#[clap(default_value_t = DataFrom::internal(2023, 12))]
	pub input: DataFrom,
}

impl Day12 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		let mut cache = AHashMap::with_capacity(512);

		let score1 = {
			let entries = map_trimmed_nonempty_lines_of_file(&self.input, |line| {
				let (states, rle) = line
					.split_once(' ')
					.context("line missing a single space")?;
				let states: StateArray = states
					.as_bytes()
					.iter()
					.copied()
					.map(|c| {
						Ok(match c {
							b'.' => SpringState::Good,
							b'#' => SpringState::Bad,
							b'?' => SpringState::Unknown,
							b => bail!("unexpected character: {}", b as char),
						})
					})
					.collect::<anyhow::Result<_>>()?;

				let rle: RleArray = rle.split(',').map(str::parse).collect::<Result<_, _>>()?;

				Ok(SpringRow { states, rle })
			})?;

			entries
				.iter()
				.map(|e| {
					cache.clear();
					e.count_possibilities_cached(&mut cache, 0, 0, 0)
				})
				.sum()
		};

		let score2 = {
			let entries = map_trimmed_nonempty_lines_of_file(&self.input, |line| {
				let (states, rle) = line
					.split_once(' ')
					.context("line missing a single space")?;
				let states_iter = states.as_bytes().iter().copied();
				let states: StateArray = states_iter
					.clone()
					.chain(std::iter::once(b'?'))
					.chain(states_iter.clone())
					.chain(std::iter::once(b'?'))
					.chain(states_iter.clone())
					.chain(std::iter::once(b'?'))
					.chain(states_iter.clone())
					.chain(std::iter::once(b'?'))
					.chain(states_iter)
					.map(|c| {
						Ok(match c {
							b'.' => SpringState::Good,
							b'#' => SpringState::Bad,
							b'?' => SpringState::Unknown,
							b => bail!("unexpected character: {}", b as char),
						})
					})
					.collect::<anyhow::Result<_>>()?;

				let mut rle: RleArray = rle.split(',').map(str::parse).collect::<Result<_, _>>()?;
				// rle.reverse();
				let rle_len = rle.len();
				for _ in 0..4 {
					for i in 0..rle_len {
						rle.push(rle[i]);
					}
				}

				Ok(SpringRow { states, rle })
			})?;

			entries
				.iter()
				.map(|e| {
					cache.clear();
					e.count_possibilities_cached(&mut cache, 0, 0, 0)
				})
				.sum()
		};

		Ok((score1, score2))
	}
}

type StateArray = Vec<SpringState>;
type RleArray = Vec<u8>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpringState {
	Unknown,
	Bad,
	Good,
}

impl Debug for SpringState {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			SpringState::Unknown => f.write_str("?"),
			SpringState::Bad => f.write_str("#"),
			SpringState::Good => f.write_str("."),
		}
	}
}

impl Default for SpringState {
	fn default() -> Self {
		Self::Unknown
	}
}

#[derive(Debug, Clone)]
struct SpringRow {
	states: StateArray,
	rle: RleArray,
}

impl SpringRow {
	fn count_possibilities_cached(
		&self,
		cache: &mut AHashMap<u32, u64>,
		si: u8,
		ri: u8,
		existing_rle_value: u8,
	) -> u64 {
		let key = u32::from_ne_bytes([si, ri, existing_rle_value, 0]);
		if let Some(&res) = cache.get(&key) {
			return res;
		}
		if si as usize >= self.states.len() {
			if ri as usize == self.rle.len() {
				return 1;
			}
			if ri as usize == self.rle.len() - 1 && self.rle[ri as usize] == existing_rle_value {
				return 1;
			}
			return 0;
		}
		let mut count = 0;
		let state = self.states.get(si as usize).copied().unwrap_or_default();
		if matches!(state, SpringState::Good | SpringState::Unknown) {
			if existing_rle_value == 0 {
				count += self.count_possibilities_cached(cache, si + 1, ri, 0);
			} else if self.rle.get(ri as usize).copied() == Some(existing_rle_value) {
				count += self.count_possibilities_cached(cache, si + 1, ri + 1, 0);
			}
		}
		if matches!(state, SpringState::Bad | SpringState::Unknown)
			&& (ri as usize) < self.rle.len()
		{
			count += self.count_possibilities_cached(cache, si + 1, ri, existing_rle_value + 1);
		}
		let _ = cache.insert(key, count);
		count
	}
}

#[cfg(test)]
mod tests {
	crate::run_basic_tests!(
		super::Day12,
		trivial_example: (
			"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1
", (6, 6),
		),
		main_example: (
			"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
", (21, 525_152)
		),
	);
}
