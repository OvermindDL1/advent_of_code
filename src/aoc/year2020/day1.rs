use clap::Parser;
// use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file to use full of integers one per line
	pub input_file: PathBuf,
}

impl Day1 {
	pub fn run(&self) -> anyhow::Result<()> {
		let mut nums = Vec::with_capacity(1024);
		{
			let mut line = String::with_capacity(16);
			let mut data = BufReader::new(File::open(&self.input_file)?);
			while let Ok(len) = data.read_line(&mut line) {
				if len == 0 {
					break;
				}
				let trimmed = line.trim();
				if !trimmed.is_empty() {
					nums.push(trimmed.parse::<u32>()?);
				}
				line.clear();
			}
		}
		nums.sort();

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
