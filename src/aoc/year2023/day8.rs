use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day8 {
	/// The input file of "map and direction" data
	#[clap(default_value_t = DataFrom::internal(2023, 8))]
	pub input: DataFrom,
}

impl Day8 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		Ok((0, 0))
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
			(6, 0),
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
			(2, 0),
		),
	);
}
