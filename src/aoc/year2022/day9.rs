use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::bail;
use clap::Parser;
use std::collections::HashSet;

#[derive(Debug, Parser)]
pub struct Day9 {
	/// The input file of "movement commands"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 9})]
	pub input: DataFrom,
}

struct State {
	knots: [(isize, isize); 10],
	touched1: HashSet<(isize, isize)>,
	touched9: HashSet<(isize, isize)>,
}

impl Default for State {
	fn default() -> Self {
		let mut touched1 = HashSet::new();
		touched1.insert((0, 0));
		let touched9 = touched1.clone();
		Self {
			knots: [(0, 0); 10],
			touched1,
			touched9,
		}
	}
}

impl State {
	fn move_dir(&mut self, dir: (isize, isize)) {
		let (mut head, rest) = self
			.knots
			.split_first_mut()
			.expect("knots is empty somehow");
		head.0 += dir.0;
		head.1 += dir.1;
		for tail in rest {
			let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);
			if dx.abs() <= 1 && dy.abs() <= 1 {
				break;
			}
			tail.0 += dx.signum();
			tail.1 += dy.signum();
			head = tail;
		}
		self.touched1.insert(self.knots[1]);
		self.touched9.insert(self.knots[9]);
	}
}

impl Day9 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut state = State::default();
		process_trimmed_nonempty_lines_of_file(&self.input, |line| {
			match line.as_bytes() {
				[dir, b' ', count @ ..] => {
					let dir = match dir {
						b'U' => (0, 1),
						b'D' => (0, -1),
						b'R' => (1, 0),
						b'L' => (-1, 0),
						_ => bail!("Invalid direction: {}", *dir as char),
					};
					let count = count
						.iter()
						.copied()
						.map(|c| c - b'0')
						.fold(0, |acc, c| acc * 10 + c as usize);
					for _ in 0..count {
						state.move_dir(dir);
					}
				}
				unhandled => bail!("unhandled line: {unhandled:?}"),
			}
			Ok(())
		})?;

		println!("Step 1: {}", state.touched1.len());
		println!("Step 2: {}", state.touched9.len());
		Ok(())
	}
}
