use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2016 {
	/// Run all the Advent of Code 2016 days
	RunAll,
}

impl Year2016 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2016, self, app, [])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2016, app, [])
	}
}
