use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2018 {
	/// Run all the Advent of Code 2018 days
	RunAll,
}

impl Year2018 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		crate::run_days!(Year2018, self, app, [])
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2016, app, [])
	}
}
