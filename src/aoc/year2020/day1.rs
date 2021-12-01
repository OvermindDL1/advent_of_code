use crate::aoc::helpers::*;
use clap::Parser;
use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file to use full of integers one per line
	#[clap(default_value = "inputs/2020/day1.input")]
	pub input_file: PathBuf,
}

impl Day1 {
	pub fn run(&self) -> anyhow::Result<()> {
		let mut nums =
			map_trimmed_nonempty_lines_of_file(
				&self.input_file,
				|line| Ok(line.parse::<usize>()?),
			)?;
		nums.sort_unstable();

		for a in 0..nums.len() {
			for b in a + 1..nums.len() {
				match (nums[a] + nums[b]).cmp(&2020) {
					Ordering::Less => {}
					Ordering::Equal => {
						println!("Step 1: {}", nums[a] * nums[b]);
						break;
					}
					Ordering::Greater => {
						break;
					}
				}
			}
		}

		for a in 0..nums.len() {
			for b in a + 1..nums.len() {
				for c in b + 1..nums.len() {
					match (nums[a] + nums[b] + nums[c]).cmp(&2020) {
						Ordering::Less => {}
						Ordering::Equal => {
							println!("Step 2: {}", nums[a] * nums[b] * nums[c]);
							break;
						}
						Ordering::Greater => {
							break;
						}
					}
				}
			}
		}

		// nums.iter()
		// 	.tuple_combinations()
		// 	.filter(|(&a, &b)| a + b == 2020)
		// 	.map(|(a, b)| a * b)
		// 	.for_each(|a| println!("Step 1: {}", a));
		//
		// nums.iter()
		// 	.tuple_combinations()
		// 	.filter(|(&a, &b, &c)| a + b + c == 2020)
		// 	.map(|(a, b, c)| a * b * c)
		// 	.for_each(|a| println!("Step 2: {}", a));

		Ok(())
	}
}
