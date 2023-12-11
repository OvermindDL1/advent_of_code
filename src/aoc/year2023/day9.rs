use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use itertools::Itertools;

type ValuesVec = tinyvec::ArrayVec<[i64; 24]>;

#[derive(Debug, Parser)]
pub struct Day9 {
	/// The input file of "Oasis And Sand Instability Sensor" data
	#[clap(default_value_t = DataFrom::internal(2023, 9))]
	pub input: DataFrom,
}

impl Day9 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(i64, i64)> {
		let values = map_trimmed_nonempty_lines_of_file(&self.input, |line| {
			Ok(line
				.split_whitespace()
				.map(str::parse)
				.collect::<Result<ValuesVec, _>>()?)
		})?;

		let (front, back) = values
			.iter()
			.map(|v| differences_predicted(v))
			.reduce(|f, b| (f.0 + b.0, f.1 + b.1))
			.context("values input is empty")?;

		Ok((back, front))
	}
}

fn differences_predicted(values: &[i64]) -> (i64, i64) {
	let predictions = values
		.iter()
		.copied()
		.tuple_windows()
		.map(|(a, b)| b - a)
		.collect::<ValuesVec>();
	let (front_prediction, back_prediction) = match predictions.len() {
		0 => (0, 0),
		1 => (predictions[0], predictions[0]),
		_ if predictions.iter().copied().dedup().count() == 1 => (predictions[0], predictions[0]),
		_ => differences_predicted(&predictions),
	};
	(
		values[0] - front_prediction,
		values[values.len() - 1] + back_prediction,
	)
}

#[cfg(test)]
mod tests {
	crate::run_basic_tests!(
		super::Day9,
		example: (
			"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
",
			(114, 2),
		),
	);
}
