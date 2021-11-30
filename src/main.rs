use advent_of_code::AocApp;
use clap::Parser;

fn main() -> anyhow::Result<()> {
	let args = AocApp::parse();
	println!("{:?}", args);
	args.run()
}
