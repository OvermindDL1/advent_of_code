use crate::AocApp;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day1 {
	/// The input file of "calories"
	#[clap(default_value = "inputs/2022/day1.input")]
	pub input_file: PathBuf,
}

impl Day1 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		Ok(())
	}
}
