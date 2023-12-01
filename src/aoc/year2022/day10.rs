use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day10 {
	/// The input file of "opcodes"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 10})]
	pub input: DataFrom,
}

impl Day10 {
	#[allow(clippy::cast_possible_wrap)]
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut xs = Vec::with_capacity(1024);
		xs.push(1); // Buffer to align the cycles
		xs.push(1);
		process_trimmed_nonempty_lines_of_file(&self.input, |line| {
			let x = *xs.last().context("somehow xs is empty")?;
			match line.split_at(4) {
				("noop", "") => xs.push(x),
				("addx", amt) => {
					xs.push(x);
					xs.push(x + amt.trim().parse::<i64>().context("invalid addx amount")?);
				}
				unhandled => bail!("unhandled opcode: {:?}", unhandled),
			}
			Ok(())
		})?;
		xs.pop(); // Don't care about last state, keeps rest clean

		// xs.iter().copied().enumerate().for_each(|(i, x)| {
		// 	println!("{}: {} -> {}", i, x, i as isize * x as isize);
		// });

		if xs.len() <= 220 {
			bail!("input did not create enough states: {}", xs.len());
		}
		let score1 = [20, 60, 100, 140, 180, 220]
			.into_iter()
			.map(|i| xs[i] * i as i64)
			.sum::<i64>();
		println!("Step 1: {score1}");

		print!("Step 2:");
		xs.iter()
			.copied()
			.enumerate()
			.skip(1)
			.try_for_each(|(i, x)| {
				let i = (i - 1) % 40;
				if i == 0 {
					println!();
				}
				if (isize::try_from(x)? - isize::try_from(i)?).abs() > 1 {
					print!(" ");
				} else {
					print!("█");
				}
				Ok::<_, anyhow::Error>(())
			})?;
		println!();

		Ok(())
	}
}
