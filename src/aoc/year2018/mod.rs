use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Year2018 {}

impl Year2018 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		todo!()
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		crate::run_all_days!(Year2016, app, [])
	}
}
