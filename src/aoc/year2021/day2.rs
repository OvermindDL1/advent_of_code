use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::num::NonZeroU8;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day2 {
	/// The input file of "commands"
	#[clap(default_value = "inputs/2021/day2.input")]
	pub input_file: PathBuf,
}

enum Commands {
	Forward(NonZeroU8),
	Down(NonZeroU8),
	Up(NonZeroU8),
}

#[derive(Default)]
struct Pos {
	depth: u32,
	fore: u32,
	aim: u32,
}

impl Pos {
	fn solution(&self) -> u32 {
		self.depth * self.fore
	}
}

impl Day2 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let commands = map_trimmed_nonempty_lines_of_file(&self.input_file, |line| {
			match line
				.split_once(' ')
				.context("input is not a command then space then a number")?
			{
				("forward", n) => Ok(Commands::Forward(
					n.parse().context("input is not a number")?,
				)),
				("down", n) => Ok(Commands::Down(n.parse().context("input is not a number")?)),
				("up", n) => Ok(Commands::Up(n.parse().context("input is not a number")?)),
				_ => anyhow::bail!("input is not a valid command of forward|down|up then a number"),
			}
		})?;
		println!(
			"Step 1: {}",
			commands
				.iter()
				.fold(Pos::default(), |mut pos, cmd| {
					match cmd {
						Commands::Forward(n) => pos.fore += n.get() as u32,
						Commands::Down(n) => pos.depth += n.get() as u32,
						Commands::Up(n) => pos.depth -= n.get() as u32,
					}
					pos
				})
				.solution()
		);
		println!(
			"Step 2: {}",
			commands
				.iter()
				.fold(Pos::default(), |mut pos, cmd| {
					match cmd {
						Commands::Down(n) => pos.aim += n.get() as u32,
						Commands::Up(n) => pos.aim -= n.get() as u32,
						Commands::Forward(n) => {
							pos.fore += n.get() as u32;
							pos.depth += pos.aim * n.get() as u32;
						}
					}
					pos
				})
				.solution()
		);

		Ok(())
	}
}
