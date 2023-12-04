#![allow(clippy::similar_names)]

use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Day7 {
	/// The input file of "terminal history"
	#[clap(default_value_t = DataFrom::internal(2022, 7))]
	pub input: DataFrom,
}

impl Day7 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(usize, usize)> {
		let input = self.input.as_cow_str()?;
		let input = input.as_ref().trim();

		let mut size_stack = Vec::new();
		let mut sizes = Vec::new();
		let mut cwd = PathBuf::new();
		let mut score1 = 0;
		for io in input.split('$').map(str::trim).filter(|l| !l.is_empty()) {
			let (cmd, response) = io
				.split_once('\n')
				.map_or((io, ""), |(cmd, response)| (cmd.trim(), response.trim()));
			match cmd
				.split_once(' ')
				.map_or((cmd, ""), |(cmd, args)| (cmd.trim(), args.trim()))
			{
				("cd", "/") if response.is_empty() => {
					cwd.clear();
					size_stack.clear();
					size_stack.push(0);
				}
				("cd", "..") if response.is_empty() => {
					cwd.pop();
					let size = size_stack.pop().context("size stack underflow")?;
					sizes.push(size);
					if size < 100_000 {
						score1 += size;
					}
					if let Some(s) = size_stack.last_mut() {
						*s += size;
					}
				}
				("cd", dir) if response.is_empty() => {
					cwd.push(dir);
					size_stack.push(0);
				}
				("ls", "") => {
					for line in response.split('\n') {
						match line
							.split_once(' ')
							.context("invalid `ls` response line of `{node}` in:\n{response}")?
						{
							("dir", _dir) => {}
							(size, _file) => {
								let size = size.parse::<usize>().context("invalid file size")?;
								*size_stack.last_mut().context("no current directory")? += size;
							}
						}
					}
				}
				(cmd, args) => {
					anyhow::bail!("unhandled command `{cmd} {args}` with response of: \n{response}")
				}
			}
		}
		let mut psize = 0;
		sizes.extend(size_stack.into_iter().rev().map(|size| {
			psize += size;
			psize
		}));

		sizes.sort_unstable();
		let largest = *sizes.last().context("no sizes")?;
		let free = 70_000_000_usize
			.checked_sub(largest)
			.context("somehow storing more than there is of free space")?;
		let need = 30_000_000_usize.saturating_sub(free);
		let idx = sizes.binary_search(&need).map_or_else(|i| i, |i| i);
		let score2 = sizes
			.get(idx)
			.copied()
			.context("no directory is large enough to free enough space")?;

		Ok((score1, score2))
	}
}
