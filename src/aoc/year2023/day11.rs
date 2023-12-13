use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day11 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 11))]
	pub input: DataFrom,
}

const MILLION: i64 = 1_000_000;

#[allow(clippy::similar_names)]
impl Day11 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(i64, i64)> {
		let input = self.input.as_cow_u8()?;
		let width = input
			.iter()
			.position(|&b| b == b'\n')
			.context("no newline in input")?;
		let expansions = (0..width)
			.map(|i| input.iter().skip(i).step_by(width + 1).all(|&b| b == b'.'))
			.collect::<Vec<_>>();

		let mut coords = Vec::with_capacity(512);
		let mut coord_2 = (0i64, 0i64);
		let mut coord_1m = (0i64, 0i64);
		for line in input.split(|&b| b == b'\n') {
			coord_2.0 = 0;
			coord_1m.0 = 0;
			let mut again = true;
			for (c, expansion) in line.iter().copied().zip(expansions.iter().copied()) {
				let has_galaxy = c == b'#';
				if has_galaxy {
					coords.push((coord_2, coord_1m));
					again = false;
				}
				coord_2.0 += if expansion { 2 } else { 1 };
				coord_1m.0 += if expansion { MILLION } else { 1 };
			}
			coord_2.1 += if again { 2 } else { 1 };
			coord_1m.1 += if again { MILLION } else { 1 };
		}

		let (score1, score2) = coords
			.iter()
			.copied()
			.enumerate()
			.flat_map(|(i, first)| {
				coords
					.iter()
					.copied()
					.skip(i + 1)
					.map(move |second| (first, second))
			})
			.map(|(((x0, y0), (x02, y02)), ((x1, y1), (x12, y12)))| {
				let dist1 = (x0 - x1).abs() + (y0 - y1).abs();
				let dist2 = (x02 - x12).abs() + (y02 - y12).abs();
				(dist1, dist2)
			})
			.fold((0, 0), |(s1, s2), (a1, a2)| (s1 + a1, s2 + a2));

		Ok((score1, score2))
	}
}

#[cfg(test)]
mod tests {
	crate::run_basic_tests!(
		super::Day11,
		example: (
			"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", (374, 82000210),
		),
	);
}
