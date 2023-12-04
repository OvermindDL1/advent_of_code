use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day2 {
	/// The input file of "Dice counts"
	#[clap(default_value_t = DataFrom::internal(2023, 2))]
	pub input: DataFrom,
}

impl Day2 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u32, u32)> {
		// (red, green, blue)
		let step1_max_color_counts = (12, 13, 14);
		let mut score1 = 0;
		let mut score2 = 0;
		process_trimmed_nonempty_lines_of_file(&self.input, |mut line| {
			let orig_line = line;
			line = line
				.strip_prefix("Game ")
				.with_context(|| format!("missing `Game ` prefix on line: {orig_line}"))?;
			let (game_number, line) = line
				.split_once(": ")
				.with_context(|| format!("missing `: ` on line: {orig_line}"))?;
			let game_number = game_number
				.parse::<u32>()
				.with_context(|| format!("failed to parse game number on line: {orig_line}"))?;
			let mut step1_possible = true;
			let mut max_color_counts = (0, 0, 0);
			for cubes in line.split("; ") {
				let mut red = 0;
				let mut green = 0;
				let mut blue = 0;
				for cube in cubes.split(", ") {
					let (count, color) = cube.split_once(' ').with_context(|| {
						format!("missing ` ` between count and color on line: {orig_line}")
					})?;
					let count =
						count.parse::<u32>().with_context(|| {
							format!("failed to parse count `{count}` from `{cube}` on line: {orig_line}")
						})?;
					match color {
						"red" if red == 0 => {
							red = count;
							max_color_counts.0 = max_color_counts.0.max(count);
						}
						"green" if green == 0 => {
							green = count;
							max_color_counts.1 = max_color_counts.1.max(count);
						}
						"blue" if blue == 0 => {
							blue = count;
							max_color_counts.2 = max_color_counts.2.max(count);
						}
						"red" | "green" | "blue" => bail!("duplicate {color} on line: {orig_line}"),
						_ => bail!("unknown color on line: {orig_line}"),
					}
				}
				if red > step1_max_color_counts.0
					|| green > step1_max_color_counts.1
					|| blue > step1_max_color_counts.2
				{
					step1_possible = false;
				}
			}

			if step1_possible {
				score1 += game_number;
			}
			score2 += max_color_counts.0 * max_color_counts.1 * max_color_counts.2;
			Ok(())
		})?;

		Ok((score1, score2))
	}
}
