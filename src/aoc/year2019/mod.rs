use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2019 {
	/// Run all the Advent of Code 2019 days
	RunAll,
}

impl Year2019 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2019, self, app, [])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2019, app, [])
	}
}
