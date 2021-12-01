use anyhow::Context;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn process_lines_of_file(
	filepath: &Path,
	mut cb: impl FnMut(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	let mut line = String::with_capacity(16);
	let mut data = BufReader::new(File::open(filepath)?);
	while data.read_line(&mut line)? > 0 {
		cb(&line)?;
		let trimmed = line.trim();
		if !trimmed.is_empty() {}
		line.clear();
	}
	Ok(())
}

pub fn process_trimmed_lines_of_file(
	filepath: &Path,
	mut cb: impl FnMut(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	process_lines_of_file(filepath, |line| {
		cb(line.trim())?;
		Ok(())
	})
}

pub fn process_trimmed_nonempty_lines_of_file(
	filepath: &Path,
	mut cb: impl FnMut(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	process_trimmed_lines_of_file(filepath, |line| {
		if !line.is_empty() {
			cb(line)?;
		}
		Ok(())
	})
}

pub fn map_trimmed_nonempty_lines_of_file<R, F: FnMut(&str) -> anyhow::Result<R>>(
	filepath: &Path,
	mut cb: F,
) -> anyhow::Result<Vec<R>> {
	let mut results = Vec::with_capacity(8192);
	process_trimmed_nonempty_lines_of_file(filepath, |line| {
		results.push(cb(line).with_context(|| format!("Failed parsing line: {}", line))?);
		Ok(())
	})?;
	Ok(results)
}
