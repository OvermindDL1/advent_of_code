use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2015 {
	/// Run all the Advent of Code 2015 days
	RunAll,
}

impl Year2015 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2015, self, app, [])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2015, app, [])
	}
}
