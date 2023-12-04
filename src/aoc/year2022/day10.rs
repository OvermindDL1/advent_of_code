use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;
use std::fmt::Write;

#[derive(Debug, Parser)]
pub struct Day10 {
	/// The input file of "opcodes"
	#[clap(default_value_t = DataFrom::internal(2022, 10))]
	pub input: DataFrom,
}

impl Day10 {
	#[allow(clippy::cast_possible_wrap)]
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(i64, String)> {
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

		let mut score2 = String::new();
		xs.iter()
			.copied()
			.enumerate()
			.skip(1)
			.try_for_each(|(i, x)| {
				let i = (i - 1) % 40;
				if i == 0 {
					writeln!(score2).context("failed to write score2")?;
				}
				if (isize::try_from(x)? - isize::try_from(i)?).abs() > 1 {
					writeln!(score2).context("failed to write score2")?;
				} else {
					writeln!(score2, "█").context("failed to write score2")?;
				}
				Ok::<_, anyhow::Error>(())
			})?;
		writeln!(score2).context("failed to write score2")?;

		Ok((score1, score2))
	}
}
