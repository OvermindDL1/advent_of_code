use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Debug, Parser)]
pub struct Day5 {
	/// The input file of "stacks" and "commands"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 5})]
	pub input: DataFrom,
}

#[derive(Debug)]
struct Move {
	count: usize,
	from: usize,
	to: usize,
}

impl Display for Move {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"move {} from {} to {}",
			self.count,
			self.from + 1,
			self.to + 1
		)
	}
}

impl Move {
	fn parse(s: &str) -> anyhow::Result<Move> {
		let (count, from, to) = s
			.split_whitespace()
			.flat_map(str::parse::<usize>)
			.collect_tuple()
			.with_context(|| format!("invalid move: {s}"))?;
		let from = from - 1;
		let to = to - 1;
		Ok(Move { count, from, to })
	}

	fn perform(&self, stacks: &mut CrateStacks) {
		assert!(self.to < stacks.0.len() && self.from < stacks.0.len());
		for _ in 0..self.count {
			let crate_ = stacks.0[self.from].pop().unwrap();
			stacks.0[self.to].push(crate_);
		}
	}

	fn perform_concurrent(&self, stacks: &mut CrateStacks) {
		assert!(self.to < stacks.0.len() && self.from < stacks.0.len());
		let (from, to) = if self.from < self.to {
			let (left, right) = stacks.0.split_at_mut(self.from + 1);
			(&mut left[self.from], &mut right[self.to - self.from - 1])
		} else {
			let (left, right) = stacks.0.split_at_mut(self.to + 1);
			(&mut right[self.from - self.to - 1], &mut left[self.to])
		};
		to.extend(from.drain(from.len() - self.count..));
	}
}

#[derive(Clone, Debug)]
struct CrateStacks(Vec<Vec<u8>>);

impl Display for CrateStacks {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let highest = self.0.iter().map(|v| v.len()).max().unwrap();
		for i in (0..highest).rev() {
			for stack in &self.0 {
				if i < stack.len() {
					write!(f, "[{}] ", stack[i] as char)?;
				} else {
					write!(f, "    ")?;
				}
			}
			writeln!(f)?;
		}
		for i in 1..=self.0.len() {
			write!(f, " {i}  ")?;
		}
		Ok(())
	}
}

impl CrateStacks {
	fn parse(input: &str) -> anyhow::Result<CrateStacks> {
		let input = input.as_bytes();
		let mut stacks = vec![];
		let mut input_stacks = input.rsplit(|&c| c == b'\n');
		let input_counts = input_stacks
			.next()
			.context("stack parsing missing final line")?;
		for (idx, sid) in input_counts.iter().enumerate() {
			if (b'1'..=b'9').contains(sid) {
				let mut stack = Vec::with_capacity(32);
				for crate_ in input_stacks
					.clone()
					.map(|l| l[idx])
					.take_while(|&c| c != b' ')
				{
					stack.push(crate_);
				}
				stacks.push(stack);
			}
		}
		Ok(CrateStacks(stacks))
	}
}

impl Day5 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let input = self.input.as_cow_str();
		let input = input.as_ref();

		let (input_stacks, input_commands) = input
			.split_once("\n\n")
			.context("input has no blank line to split on")?;
		let input_commands = input_commands.trim_start();

		let mut stacks = CrateStacks::parse(input_stacks)?;
		let moves = input_commands
			.lines()
			.map(Move::parse)
			.collect::<anyhow::Result<Vec<_>>>()?;
		let mut stacks2 = stacks.clone();

		// println!("{stacks}");
		for m in &moves {
			// println!("{m} ->");
			m.perform(&mut stacks);
			// println!("{stacks}");
		}
		let score1 = stacks
			.0
			.iter()
			.map(|s| *s.last().unwrap() as char)
			.collect::<String>();

		// println!("{stacks2}");
		for m in &moves {
			// println!("{m} ->");
			m.perform_concurrent(&mut stacks2);
			// println!("{stacks2}");
		}
		let score2 = stacks2
			.0
			.iter()
			.map(|s| *s.last().unwrap() as char)
			.collect::<String>();

		println!("Step 1: {}", score1);
		println!("Step 2: {}", score2);
		Ok(())
	}
}