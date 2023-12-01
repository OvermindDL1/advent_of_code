use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use itertools::Itertools;

use std::cmp::Ordering;

#[derive(Debug, Parser)]
pub struct Day3 {
	/// The input file of "diagnostic bits"
	#[clap(default_value_t = DataFrom::Internal {year: 2021, day: 3})]
	pub input: DataFrom,
}

impl Day3 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut width = 0;
		let mut nums = map_trimmed_nonempty_lines_of_file(&self.input, |line| {
			width = line.as_bytes().len();
			Ok(u32::from_str_radix(line, 2)?)
		})?;

		let width: u32 = width.try_into()?;
		let half_count = u32::try_from(nums.len())? / 2;
		let gamma = nums
			.iter()
			.copied()
			.fold([0u32; u32::BITS as usize], |mut arr, n| {
				(0..width).for_each(|i| arr[i as usize] += u32::from((n & (1 << i)) != 0));
				arr
			})
			.into_iter()
			.take(width as usize)
			.enumerate()
			.try_fold(0, |n, (i, b)| -> anyhow::Result<u32> {
				Ok(n | (u32::from(b > half_count) << u32::try_from(i)?))
			})?;
		let epsilon = gamma ^ ((1 << width) - 1);
		println!("Step 1: {}", gamma * epsilon);

		let (mut co2s, mut oxygens) =
			Self::sort_bits_into_slices(nums.as_mut_slice(), width as usize - 1);
		for idx in (0..width as usize - 1).rev() {
			if co2s.len() <= 1 && oxygens.len() <= 1 {
				break;
			}
			if co2s.len() > 1 {
				let (new_least, _) = Self::sort_bits_into_slices(co2s, idx);
				co2s = new_least;
			}
			if oxygens.len() > 1 {
				let (_, new_most) = Self::sort_bits_into_slices(oxygens, idx);
				oxygens = new_most;
			}
		}
		let co2_rating = *co2s.first().context("failed finding co2 value")?;
		let oxygen_rating = *oxygens.first().context("failed finding oxygen value")?;
		println!("Step 2: {}", oxygen_rating * co2_rating);

		Ok(())
	}

	fn sort_bits_into_slices(nums: &mut [u32], idx: usize) -> (&mut [u32], &mut [u32]) {
		let ones_count = nums
			.iter()
			.copied()
			.filter(|n| (n & (1 << idx)) != 0)
			.count();
		let mid = match (2 * ones_count).cmp(&nums.len()) {
			Ordering::Less => {
				nums.sort_by(|a, b| (a & 1 << idx).cmp(&(b & 1 << idx)).reverse());
				nums.iter()
					.copied()
					.find_position(|n| (n & (1 << idx)) == 0)
					.map_or(nums.len(), |(i, _v)| i)
			}
			Ordering::Equal => {
				nums.sort_by_key(|a| a & 1 << idx);
				ones_count
			}
			Ordering::Greater => {
				nums.sort_by_key(|a| a & 1 << idx);
				nums.iter()
					.copied()
					.find_position(|n| (n & (1 << idx)) != 0)
					.map_or(nums.len(), |(i, _v)| i)
			}
		};
		nums.split_at_mut(mid)
	}
}
