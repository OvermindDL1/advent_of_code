pub mod aoc;

use std::time::Instant;
// use chrono::prelude::*;
use clap::Parser;
//use once_cell::sync::Lazy;

#[derive(Debug, Parser)]
pub struct AocApp {
	/// Level of verbosity, can be used multiple times for more verbosity
	#[clap(short, long, parse(from_occurrences))]
	pub verbose: u8,
	/// The command to execute
	#[clap(subcommand)]
	command: AocAppCommand,
}

impl AocApp {
	pub fn run(&self) -> anyhow::Result<()> {
		match &self.command {
			AocAppCommand::Run(aoc) => {
				// let start = Instant::now();
				let res = aoc.run(self);
				// if self.verbose >= 1 {
				// 	println!("_Total Program Time Taken: {:?}_", start.elapsed());
				// }
				res
			}
			AocAppCommand::RunAll => {
				println!("# OvermindDL1's Advent Of Code");
				let start = Instant::now();
				let res = aoc::AocYear::run_all(&self);
				if self.verbose >= 1 {
					println!("_All Time Taken: {:?}_", start.elapsed());
				}
				res
			}
			AocAppCommand::TUI => {
				todo!("still need to make the TUI")
			}
		}
	}
}

#[derive(Debug, Parser)]
pub enum AocAppCommand {
	#[clap(flatten)]
	Run(aoc::AocYear),
	RunAll,
	TUI,
}
