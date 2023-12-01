use crate::Inputs;
use anyhow::Context;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum DataFrom {
	Internal { year: u16, day: u8 },
	Static(Cow<'static, str>),
	Stdin,
	FilePath(PathBuf),
}

impl DataFrom {
	pub fn as_cow_str(&self) -> anyhow::Result<Cow<str>> {
		Ok(match self {
			DataFrom::Internal { year, day } => {
				let path = format!("{year}/day{day}.input");
				let data = Inputs::get(&path)
					.with_context(|| format!("missing {}", &path))
					.context("invalid internal input year and/or day")?;
				Cow::Owned(
					String::from_utf8(data.data.as_ref().to_vec())
						.context("input must be valid utf-8")?,
				)
			}
			DataFrom::Static(data) => data.clone(),
			DataFrom::Stdin => {
				let mut data = Vec::default();
				std::io::stdin()
					.read_to_end(&mut data)
					.context("invalid read from stdin")?;
				Cow::Owned(String::from_utf8(data).context("input must be valid utf-8")?)
			}
			DataFrom::FilePath(path) => {
				let data = std::fs::read_to_string(path)
					.with_context(|| format!("invalid read from path: {path:?}"))?;
				Cow::Owned(data)
			}
		})
	}

	pub fn as_cow_u8(&self) -> anyhow::Result<Cow<[u8]>> {
		Ok(match self {
			DataFrom::Internal { year, day } => {
				let path = format!("{year}/day{day}.input");
				let data = Inputs::get(&path)
					.with_context(|| format!("missing {}", &path))
					.context("invalid internal year day")?;
				data.data
			}
			DataFrom::Static(data) => Cow::Borrowed(data.as_bytes()),
			DataFrom::Stdin => {
				let mut data = Vec::default();
				std::io::stdin()
					.read_to_end(&mut data)
					.context("invalid read from stdin")?;
				Cow::Owned(data)
			}
			DataFrom::FilePath(path) => {
				let data = std::fs::read(path)
					.with_context(|| format!("invalid read from path: {path:?}"))?;
				Cow::Owned(data)
			}
		})
	}
}

impl Display for DataFrom {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			DataFrom::Internal { year, day } => f.write_fmt(format_args!(":{year}:{day}")),
			DataFrom::Static(data) => f.write_str(data),
			DataFrom::Stdin => f.write_str("-"),
			DataFrom::FilePath(filepath) => {
				if let Some(p) = filepath.to_str() {
					f.write_str(p)
				} else {
					panic!("Internal file paths should always be UTF-8: {filepath:?}")
				}
			}
		}
	}
}

impl FromStr for DataFrom {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s == "-" {
			Ok(DataFrom::Stdin)
		} else if let Some(s) = s.strip_prefix(':') {
			let (year, day) = s.split_once(':').context("invalid :year:day order")?;
			let year = year.parse().with_context(|| "invalid :year:day order")?;
			let day = day.parse().with_context(|| "invalid :year:day order")?;
			Ok(DataFrom::Internal { year, day })
		} else if s.contains('\n') {
			Ok(DataFrom::Static(Cow::Owned(s.to_string())))
		} else {
			Ok(DataFrom::FilePath(PathBuf::from(s)))
		}
	}
}

impl From<&OsStr> for DataFrom {
	fn from(s: &OsStr) -> Self {
		if let Some(s) = s.to_str() {
			DataFrom::from_str(s).expect("can't happen")
		} else {
			DataFrom::FilePath(PathBuf::from(s))
		}
	}
}

pub fn process_lines_of_file(
	data: &DataFrom,
	mut cb: impl FnMut(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	match data {
		DataFrom::Internal { year, day } => {
			let path = format!("{year}/day{day}.input");
			let data = Inputs::get(&path).with_context(|| format!("missing {}", &path))?;
			for line in std::str::from_utf8(data.data.as_ref())?.lines() {
				cb(line).with_context(|| format!("Failed parsing line: {line}"))?;
			}
		}
		DataFrom::Static(data) => {
			for line in data.lines() {
				cb(line).with_context(|| format!("Failed parsing line: {line}"))?;
			}
		}
		DataFrom::Stdin => {
			let stdin = std::io::stdin();
			let mut handle = stdin.lock();
			let mut line = String::with_capacity(16);
			while handle.read_line(&mut line)? > 0 {
				cb(&line).with_context(|| format!("Failed parsing line: {line}"))?;
				line.clear();
			}
		}
		DataFrom::FilePath(filepath) => {
			let mut data = BufReader::new(File::open(filepath)?);
			let mut line = String::with_capacity(16);
			while data.read_line(&mut line)? > 0 {
				cb(&line).with_context(|| format!("Failed parsing line: {line}"))?;
				line.clear();
			}
		}
	}
	Ok(())
}

pub fn process_trimmed_lines_of_file(
	data: &DataFrom,
	mut cb: impl FnMut(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	process_lines_of_file(data, |line| {
		cb(line.trim())?;
		Ok(())
	})
}

pub fn process_trimmed_nonempty_lines_of_file(
	data: &DataFrom,
	mut cb: impl FnMut(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	process_trimmed_lines_of_file(data, |line| {
		if !line.is_empty() {
			cb(line)?;
		}
		Ok(())
	})
}

pub fn flat_map_trimmed_nonempty_lines_of_file<
	R: IntoIterator,
	F: FnMut(&str) -> anyhow::Result<R>,
>(
	data: &DataFrom,
	mut cb: F,
) -> anyhow::Result<Vec<<R as IntoIterator>::Item>> {
	let mut results = Vec::with_capacity(8192);
	process_trimmed_nonempty_lines_of_file(data, |line| {
		results.extend(cb(line)?);
		Ok(())
	})?;
	Ok(results)
}

pub fn map_trimmed_nonempty_lines_of_file<R, F: FnMut(&str) -> anyhow::Result<R>>(
	data: &DataFrom,
	mut cb: F,
) -> anyhow::Result<Vec<R>> {
	let mut results = Vec::with_capacity(8192);
	process_trimmed_nonempty_lines_of_file(data, |line| {
		results.push(cb(line)?);
		Ok(())
	})?;
	Ok(results)
}

pub fn fold_trimmed_nonempty_lines_of_file<R, F: FnMut(R, &str) -> anyhow::Result<R>>(
	data: &DataFrom,
	acc: R,
	mut cb: F,
) -> anyhow::Result<R> {
	let mut acc = Some(acc);
	process_trimmed_nonempty_lines_of_file(data, |line| {
		acc = Some(cb(
			acc.take().context("failed to acquire accumulator value")?,
			line,
		)?);
		Ok(())
	})?;
	acc.take().context("failed to return accumulator value")
}
