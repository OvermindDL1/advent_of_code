use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day3 {
	/// The input file of "gear data"
	#[clap(default_value_t = DataFrom::internal(2023, 3))]
	pub input: DataFrom,
}

impl Day3 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u32, u32)> {
		let data = self.input.as_cow_u8()?;
		let stride = data
			.iter()
			.position(|&b| b == b'\n')
			.context("no newlines found in input")?
			+ 1;

		let mut score1 = 0;
		let mut score2 = 0;
		let mut i = 0;
		while i < data.len() {
			let b = data[i];
			if b.is_ascii_digit() {
				if let Some(part_number) = is_part(&data, i, stride) {
					i += part_number.len() - 1;
					let part_number = part_number
						.iter()
						.copied()
						.fold(0u32, |acc, b| acc * 10 + (b - b'0') as u32);
					score1 += part_number;
				}
			} else if b == b'*' {
				if let Some(score) = get_dual_gear_score(&data, i, stride) {
					score2 += score;
				}
			}
			i += 1;
		}

		Ok((score1, score2))
	}
}

fn is_symbol(b: u8) -> bool {
	!b.is_ascii_digit() && b != b'.' && b != b'\n'
}

fn is_part(data: &[u8], left: usize, stride: usize) -> Option<&[u8]> {
	let right = data[left..]
		.iter()
		.copied()
		.position(|b| !b.is_ascii_digit())
		.unwrap_or(data.len() + 1)
		- 1 + left;
	let do_left = left % stride != 0;
	let do_right = right % stride != stride - 1;
	let do_top = left >= stride;
	let do_bottom = left + stride < data.len();
	if do_left {
		if is_symbol(data[left - 1]) {
			return Some(&data[left..=right]);
		}
		if do_top && is_symbol(data[left - stride - 1]) {
			return Some(&data[left..=right]);
		}
		if do_bottom && is_symbol(data[left + stride - 1]) {
			return Some(&data[left..=right]);
		}
	}
	if do_right {
		if is_symbol(data[right + 1]) {
			return Some(&data[left..=right]);
		}
		if do_top && is_symbol(data[right - stride + 1]) {
			return Some(&data[left..=right]);
		}
		if do_bottom && is_symbol(data[right + stride + 1]) {
			return Some(&data[left..=right]);
		}
	}
	if do_top {
		for &b in &data[(left - stride)..=(right - stride)] {
			if is_symbol(b) {
				return Some(&data[left..=right]);
			}
		}
	}
	if do_bottom {
		for &b in &data[(left + stride)..=(right + stride)] {
			if is_symbol(b) {
				return Some(&data[left..=right]);
			}
		}
	}
	None
}

fn get_dual_gear_score(data: &[u8], gear: usize, stride: usize) -> Option<u32> {
	let do_left = gear % stride != 0;
	let do_right = gear % stride != stride - 1;
	let do_top = gear >= stride;
	let do_bottom = gear + stride < data.len();

	let mut count = 0;
	let mut score = 1;

	if do_left {
		if let Some(value) = get_number_at(data, gear - 1) {
			count += 1;
			score *= value;
			if count > 2 {
				return None;
			}
		}
	}

	if do_right {
		if let Some(value) = get_number_at(data, gear + 1) {
			count += 1;
			score *= value;
			if count > 2 {
				return None;
			}
		}
	}

	if do_top {
		if let Some(value) = get_number_at(data, gear - stride) {
			count += 1;
			score *= value;
			if count > 2 {
				return None;
			}
		} else {
			// Middle is not a number, so test both corners instead
			if let Some(value) = get_number_at(data, gear - stride - 1) {
				count += 1;
				score *= value;
				if count > 2 {
					return None;
				}
			}
			if let Some(value) = get_number_at(data, gear - stride + 1) {
				count += 1;
				score *= value;
				if count > 2 {
					return None;
				}
			}
		}
	}

	if do_bottom {
		if let Some(value) = get_number_at(data, gear + stride) {
			count += 1;
			score *= value;
			if count > 2 {
				return None;
			}
		} else {
			// Middle is not a number, so test both corners instead
			if let Some(value) = get_number_at(data, gear + stride - 1) {
				count += 1;
				score *= value;
				if count > 2 {
					return None;
				}
			}
			if let Some(value) = get_number_at(data, gear + stride + 1) {
				count += 1;
				score *= value;
				if count > 2 {
					return None;
				}
			}
		}
	}

	if count == 2 {
		Some(score)
	} else {
		None
	}
}

fn get_number_at(data: &[u8], mid: usize) -> Option<u32> {
	if !data[mid].is_ascii_digit() {
		return None;
	}
	let mut left = mid;
	while left > 0 && data[left - 1].is_ascii_digit() {
		left -= 1;
	}
	let mut right = mid;
	while right < data.len() && data[right + 1].is_ascii_digit() {
		right += 1;
	}
	let value = data[left..=right]
		.iter()
		.copied()
		.fold(0u32, |acc, b| acc * 10 + (b - b'0') as u32);
	Some(value)
}
