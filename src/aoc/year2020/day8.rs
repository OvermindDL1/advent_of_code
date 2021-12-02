use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Day8 {
	/// The input file to use with the parseable instructions
	#[clap(default_value = "inputs/2020/day8.input")]
	pub input_file: PathBuf,
}

#[derive(Debug)]
enum Insns {
	Acc(i32),
	Jmp(i32),
	Nop(i32),
}

impl FromStr for Insns {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (insn, arg) = s.split_once(' ').context("Failed to split instruction")?;
		let arg = arg.parse::<i32>().context("Failed to parse arg")?;
		match insn {
			"acc" => Ok(Insns::Acc(arg)),
			"jmp" => Ok(Insns::Jmp(arg)),
			"nop" => Ok(Insns::Nop(arg)),
			_ => anyhow::bail!("Unknown instruction: {}", insn),
		}
	}
}

struct Program {
	insns: Vec<Insns>,
	run_counts: Vec<u32>,
	acc: i32,
	ip: i32,
}

impl Program {
	fn new(insns: Vec<Insns>) -> Self {
		Self {
			run_counts: vec![0; insns.len()],
			insns,
			acc: 0,
			ip: 0,
		}
	}

	fn run_once(&mut self) -> anyhow::Result<()> {
		*self
			.run_counts
			.get_mut(self.ip as usize)
			.context("ran out of instructions")? += 1;
		match self.insns[self.ip as usize] {
			Insns::Acc(arg) => {
				self.acc += arg;
				self.ip += 1;
			}
			Insns::Jmp(arg) => {
				self.ip += arg;
			}
			Insns::Nop(_arg) => {
				self.ip += 1;
			}
		}
		Ok(())
	}

	fn has_next_insn(&self) -> bool {
		self.ip < self.insns.len() as i32
	}

	fn next_insn_run_count(&self) -> anyhow::Result<u32> {
		Ok(*self
			.run_counts
			.get(self.ip as usize)
			.context("ran out of instructions")?)
	}

	fn reset(&mut self) {
		self.acc = 0;
		self.ip = 0;
		self.run_counts.iter_mut().for_each(|x| *x = 0);
	}

	// true means it halted properly, false means it didn't
	fn reset_and_run_until_halt_or_insn_run_more_than(
		&mut self,
		count: u32,
	) -> anyhow::Result<bool> {
		self.reset();
		while self.has_next_insn() {
			if self.next_insn_run_count()? >= count {
				return Ok(false);
			}
			self.run_once()?;
		}
		Ok(true)
	}

	// true means it flipped something, false means it didn't
	fn flip_jmp_nop_at(&mut self, ip: i32) -> anyhow::Result<bool> {
		let insn = self
			.insns
			.get_mut(ip as usize)
			.context("ran out of instructions")?;
		match *insn {
			Insns::Jmp(arg) => *insn = Insns::Nop(arg),
			Insns::Nop(arg) => *insn = Insns::Jmp(arg),
			_ => return Ok(false),
		}
		Ok(true)
	}
}

impl Day8 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let insns = map_trimmed_nonempty_lines_of_file(&self.input_file, |line| {
			line.parse::<Insns>().context("Failed to parse instruction")
		})?;
		let mut program = Program::new(insns);

		if !program.reset_and_run_until_halt_or_insn_run_more_than(1)? {
			println!("Step 1: {}", program.acc);
		} else {
			anyhow::bail!("program terminated when it shouldn't have");
		}

		for i in (0..program.insns.len()).rev() {
			if !program.flip_jmp_nop_at(i as i32)? {
				continue;
			}
			if program.reset_and_run_until_halt_or_insn_run_more_than(1)? {
				println!("Step 2: {}", program.acc);
				break;
			}
			program.flip_jmp_nop_at(i as i32)?;
		}

		Ok(())
	}
}
