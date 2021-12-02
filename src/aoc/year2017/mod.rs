use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2017 {
	/// Run all the Advent of Code 2017 days
	RunAll,
}

impl Year2017 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2017, self, app, [])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2017, app, [])
	}
}
