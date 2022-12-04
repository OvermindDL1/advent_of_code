use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day2 {
	/// The input file of "games"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 2})]
	pub input: DataFrom,
}

impl Day2 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut score1 = 0;
		let mut score2 = 0;
		process_trimmed_nonempty_lines_of_file(&self.input, |line| {
			let line = &line.as_bytes()[0..=2];
			let l = line[0] as i8 - 'A' as i8 + 1;
			let r = line[2] as i8 - 'X' as i8 + 1;
			{
				let op = l;
				let sp = r;
				let w = (sp - op + 1 + 3) % 3;
				let s = w * 3 + sp;
				score1 += s as usize;
			}
			{
				let op = l;
				let w = r - 1;
				let sp = (op + w - 1 - 1 + 3) % 3 + 1;
				let s = w * 3 + sp;
				score2 += s as usize;
			}
			Ok(())
		})?;
		println!("Step 1: {score1}");
		println!("Step 2: {score2}");
		Ok(())
	}
}
