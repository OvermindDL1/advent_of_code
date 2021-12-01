use advent_of_code::AocApp;
use clap::Parser;

fn main() -> anyhow::Result<()> {
	let args = AocApp::parse();
	if args.verbose > 1 {
		println!("{:?}", args);
	}
	args.run()
}
