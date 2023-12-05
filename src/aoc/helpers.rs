use crate::Inputs;
use anyhow::Context;
use arc_swap::ArcSwapOption;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
pub struct DataFrom {
	data: DataFromState,
	cache: ArcSwapOption<Cow<'static, str>>,
}

impl Clone for DataFrom {
	fn clone(&self) -> Self {
		DataFrom {
			data: self.data.clone(),
			cache: ArcSwapOption::default(),
		}
	}
}

#[derive(Clone, Debug)]
pub enum DataFromState {
	Internal { year: u16, day: u8 },
	Static(Cow<'static, str>),
	Stdin,
	FilePath(PathBuf),
}

impl DataFrom {
	#[must_use]
	pub fn internal(year: u16, day: u8) -> Self {
		DataFrom {
			data: DataFromState::Internal { year, day },
			cache: ArcSwapOption::default(),
		}
	}
	pub fn preload(&self) -> anyhow::Result<()> {
		let data = self.as_cow_str()?;
		self.cache.store(Some(Arc::new(data)));
		Ok(())
	}

	pub fn as_cow_str(&self) -> anyhow::Result<Cow<'static, str>> {
		use std::io::Read;
		if let Some(data) = &*self.cache.load() {
			return Ok(Cow::clone(data));
		}
		Ok(match &self.data {
			DataFromState::Internal { year, day } => {
				// let path = &format!("{year}/day{day}.input");
				let y0 = (year / 1000) as u8 + b'0';
				let y1 = ((year / 100) % 10) as u8 + b'0';
				let y2 = ((year / 10) % 10) as u8 + b'0';
				let y3 = (year % 10) as u8 + b'0';
				let d0 = (day / 10) + b'0';
				let d1 = (day % 10) + b'0';
				let path = &[
					y0, y1, y2, y3, b'/', b'd', b'a', b'y', d0, d1, b'.', b'i', b'n', b'p', b'u',
					b't',
				] as &[u8];
				let path = unsafe { std::str::from_utf8_unchecked(path) };
				let data = Inputs::get(path)
					.with_context(|| format!("missing {}", &path))
					.context("invalid internal input year and/or day")?;
				Cow::Owned(
					String::from_utf8(data.data.as_ref().to_vec())
						.context("input must be valid utf-8")?,
				)
			}
			DataFromState::Static(data) => data.clone(),
			DataFromState::Stdin => {
				let mut data = Vec::default();
				std::io::stdin()
					.read_to_end(&mut data)
					.context("invalid read from stdin")?;
				Cow::Owned(String::from_utf8(data).context("input must be valid utf-8")?)
			}
			DataFromState::FilePath(path) => {
				let data = std::fs::read_to_string(path)
					.with_context(|| format!("invalid read from path: {path:?}"))?;
				Cow::Owned(data)
			}
		})
	}

	pub fn as_cow_u8(&self) -> anyhow::Result<Cow<[u8]>> {
		use std::io::Read;
		if let Some(data) = &*self.cache.load() {
			if let Cow::Borrowed(data) = Arc::as_ref(data) {
				return Ok(Cow::Borrowed(data.as_bytes()));
			}
		}
		Ok(match &self.data {
			DataFromState::Internal { year, day } => {
				// let path = &format!("{year}/day{day}.input");
				let y0 = (year / 1000) as u8 + b'0';
				let y1 = ((year / 100) % 10) as u8 + b'0';
				let y2 = ((year / 10) % 10) as u8 + b'0';
				let y3 = (year % 10) as u8 + b'0';
				let d0 = (day / 10) + b'0';
				let d1 = (day % 10) + b'0';
				let path = &[
					y0, y1, y2, y3, b'/', b'd', b'a', b'y', d0, d1, b'.', b'i', b'n', b'p', b'u',
					b't',
				] as &[u8];
				let path = unsafe { std::str::from_utf8_unchecked(path) };
				let data = Inputs::get(path)
					.with_context(|| format!("missing {}", &path))
					.context("invalid internal year day")?;
				data.data
			}
			DataFromState::Static(data) => Cow::Borrowed(data.as_bytes()),
			DataFromState::Stdin => {
				let mut data = Vec::default();
				std::io::stdin()
					.read_to_end(&mut data)
					.context("invalid read from stdin")?;
				Cow::Owned(data)
			}
			DataFromState::FilePath(path) => {
				let data = std::fs::read(path)
					.with_context(|| format!("invalid read from path: {path:?}"))?;
				Cow::Owned(data)
			}
		})
	}
}

impl Display for DataFromState {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			DataFromState::Internal { year, day } => f.write_fmt(format_args!(":{year}:{day}")),
			DataFromState::Static(data) => f.write_str(data),
			DataFromState::Stdin => f.write_str("-"),
			DataFromState::FilePath(filepath) => {
				if let Some(p) = filepath.to_str() {
					f.write_str(p)
				} else {
					panic!("Internal file paths should always be UTF-8: {filepath:?}")
				}
			}
		}
	}
}

impl Display for DataFrom {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.data.fmt(f)
	}
}

impl FromStr for DataFrom {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(DataFrom {
			data: DataFromState::from_str(s)?,
			cache: ArcSwapOption::default(),
		})
	}
}

impl FromStr for DataFromState {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s == "-" {
			Ok(DataFromState::Stdin)
		} else if let Some(s) = s.strip_prefix(':') {
			let (year, day) = s.split_once(':').context("invalid :year:day order")?;
			let year = year.parse().with_context(|| "invalid :year:day order")?;
			let day = day.parse().with_context(|| "invalid :year:day order")?;
			Ok(DataFromState::Internal { year, day })
		} else if s.contains('\n') {
			Ok(DataFromState::Static(Cow::Owned(s.to_string())))
		} else {
			Ok(DataFromState::FilePath(PathBuf::from(s)))
		}
	}
}

impl From<&OsStr> for DataFromState {
	fn from(s: &OsStr) -> Self {
		if let Some(s) = s.to_str() {
			DataFromState::from_str(s).expect("can't happen")
		} else {
			DataFromState::FilePath(PathBuf::from(s))
		}
	}
}

pub fn process_lines_of_file(
	data: &DataFrom,
	mut cb: impl FnMut(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	use std::io::BufReader;
	if let Some(data) = &*data.cache.load() {
		for line in data.lines() {
			cb(line).with_context(|| format!("Failed parsing line: {data}"))?;
		}
		return Ok(());
	}
	match &data.data {
		DataFromState::Internal { year, day } => {
			// let path = format!("{year}/day{day}.input");
			let y0 = (year / 1000) as u8 + b'0';
			let y1 = ((year / 100) % 10) as u8 + b'0';
			let y2 = ((year / 10) % 10) as u8 + b'0';
			let y3 = (year % 10) as u8 + b'0';
			let d0 = (day / 10) + b'0';
			let d1 = (day % 10) + b'0';
			let path = &[
				y0, y1, y2, y3, b'/', b'd', b'a', b'y', d0, d1, b'.', b'i', b'n', b'p', b'u', b't',
			] as &[u8];
			let path = unsafe { std::str::from_utf8_unchecked(path) };
			let data = Inputs::get(path).with_context(|| format!("missing {path}"))?;
			for line in std::str::from_utf8(data.data.as_ref())?.lines() {
				cb(line).with_context(|| format!("Failed parsing line: {line}"))?;
			}
		}
		DataFromState::Static(data) => {
			for line in data.lines() {
				cb(line).with_context(|| format!("Failed parsing line: {line}"))?;
			}
		}
		DataFromState::Stdin => {
			let stdin = std::io::stdin();
			let mut handle = stdin.lock();
			let mut line = String::with_capacity(64);
			while handle.read_line(&mut line)? > 0 {
				cb(&line).with_context(|| format!("Failed parsing line: {line}"))?;
				line.clear();
			}
		}
		DataFromState::FilePath(filepath) => {
			let mut data = BufReader::new(File::open(filepath)?);
			let mut line = String::with_capacity(64);
			while data.read_line(&mut line)? > 0 {
				cb(&line).with_context(|| format!("Failed parsing line: {line}"))?;
				line.clear();
			}
		}
	}
	Ok(())
}

pub fn process_lines_of_file_bytes(
	data: &DataFrom,
	mut cb: impl FnMut(&[u8]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	match &data.data {
		DataFromState::Internal { year, day } => {
			// let path = format!("{year}/day{day}.input");
			let y0 = (year / 1000) as u8 + b'0';
			let y1 = ((year / 100) % 10) as u8 + b'0';
			let y2 = ((year / 10) % 10) as u8 + b'0';
			let y3 = (year % 10) as u8 + b'0';
			let d0 = (day / 10) + b'0';
			let d1 = (day % 10) + b'0';
			let path = &[
				y0, y1, y2, y3, b'/', b'd', b'a', b'y', d0, d1, b'.', b'i', b'n', b'p', b'u', b't',
			] as &[u8];
			let path = unsafe { std::str::from_utf8_unchecked(path) };
			let data = Inputs::get(path).with_context(|| format!("missing `{path}`"))?;
			for line in data.data.as_ref().split(|&b| b == b'\n') {
				cb(line).with_context(|| {
					format!("Failed parsing line: {:?}", std::str::from_utf8(line))
				})?;
			}
		}
		DataFromState::Static(data) => {
			for line in data.as_bytes().split(|&b| b == b'\n') {
				cb(line).with_context(|| {
					format!("Failed parsing line: {:?}", std::str::from_utf8(line))
				})?;
			}
		}
		DataFromState::Stdin => {
			let stdin = std::io::stdin();
			let mut handle = stdin.lock();
			let mut line = Vec::with_capacity(64);
			while handle.read_until(b'\n', &mut line)? > 0 {
				cb(&line).with_context(|| {
					format!("Failed parsing line: {:?}", std::str::from_utf8(&line))
				})?;
				line.clear();
			}
		}
		DataFromState::FilePath(filepath) => {
			let mut data = std::io::BufReader::new(File::open(filepath)?);
			let mut line = Vec::with_capacity(64);
			while data.read_until(b'\n', &mut line)? > 0 {
				cb(&line).with_context(|| {
					format!("Failed parsing line: {:?}", std::str::from_utf8(&line))
				})?;
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

pub fn process_trimmed_lines_of_file_bytes(
	data: &DataFrom,
	mut cb: impl FnMut(&[u8]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	process_lines_of_file_bytes(data, |mut line| {
		while !line.is_empty() {
			if line[0].is_ascii_whitespace() {
				line = &line[1..];
			} else {
				break;
			}
		}
		while !line.is_empty() {
			if line[line.len() - 1].is_ascii_whitespace() {
				line = &line[..line.len() - 1];
			} else {
				break;
			}
		}
		cb(line)?;
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

pub fn process_trimmed_nonempty_lines_of_file_bytes(
	data: &DataFrom,
	mut cb: impl FnMut(&[u8]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
	process_trimmed_lines_of_file_bytes(data, |line| {
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

pub fn fold_trimmed_nonempty_lines_of_file_bytes<R, F: FnMut(R, &[u8]) -> anyhow::Result<R>>(
	data: &DataFrom,
	acc: R,
	mut cb: F,
) -> anyhow::Result<R> {
	let mut acc = Some(acc);
	process_trimmed_nonempty_lines_of_file_bytes(data, |line| {
		acc = Some(cb(
			acc.take().context("failed to acquire accumulator value")?,
			line,
		)?);
		Ok(())
	})?;
	acc.take().context("failed to return accumulator value")
}
