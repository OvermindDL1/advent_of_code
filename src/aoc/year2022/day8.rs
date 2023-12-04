use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;
use itertools::Itertools;
use std::fmt::Display;
use std::io::Write;
use std::ops::Range;
use termcolor::{BufferWriter, Color, ColorSpec, WriteColor};

#[derive(Debug, Parser)]
pub struct Day8 {
	/// The input file of "tree heights"
	#[clap(default_value_t = DataFrom::internal(2022, 8))]
	pub input: DataFrom,
}

struct HeightMap {
	width: usize,
	heights: Box<[u8]>,
}

impl From<&[u8]> for HeightMap {
	fn from(mut input: &[u8]) -> Self {
		while input.last() == Some(&b'\n') {
			input = &input[..input.len() - 1];
		}
		let width = input.iter().position(|&b| b == b'\n').unwrap();
		let length = bytecount::count(input, b'\n') + 1;
		let mut heights = vec![0; width * length].into_boxed_slice();
		for (b, h) in input
			.iter()
			.filter(|i| i.is_ascii_digit())
			.zip_eq(heights.iter_mut())
		{
			*h = b - b'0';
		}
		HeightMap { width, heights }
	}
}

impl Display for HeightMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (i, &height) in self.heights.iter().enumerate() {
			if i % self.width == 0 {
				writeln!(f)?;
			}
			write!(f, "{height}")?;
		}
		Ok(())
	}
}

impl HeightMap {
	fn length(&self) -> usize {
		self.heights.len() / self.width
	}

	fn width(&self) -> usize {
		self.width
	}

	fn size(&self) -> (usize, usize) {
		(self.width, self.length())
	}

	fn get(&self, x: usize, y: usize) -> u8 {
		self.heights[y * self.width + x]
	}

	fn all_heights_below_or_eq(&self, height: u8, x: Range<usize>, y: Range<usize>) -> bool {
		for y in y {
			for x in x.clone() {
				if self.get(x, y) >= height {
					return false;
				}
			}
		}
		true
	}

	#[allow(clippy::range_plus_one)]
	fn is_visible(&self, x: usize, y: usize) -> bool {
		let height = self.get(x, y);
		let (w, l) = self.size();
		self.all_heights_below_or_eq(height, 0..x, y..(y + 1))
			|| self.all_heights_below_or_eq(height, (x + 1)..w, y..(y + 1))
			|| self.all_heights_below_or_eq(height, x..(x + 1), 0..y)
			|| self.all_heights_below_or_eq(height, x..(x + 1), (y + 1)..l)
	}

	#[allow(dead_code)]
	fn print_visible(&self) {
		for y in 0..self.length() {
			for x in 0..self.width() {
				if self.is_visible(x, y) {
					print!("🌲");
				} else {
					print!("  ");
				}
			}
			println!();
		}
		println!();
	}

	const COLORS: [Color; 10] = [
		Color::Rgb(0, 255 / 10, 0),
		Color::Rgb(0, 255 / 9, 0),
		Color::Rgb(0, 255 / 8, 0),
		Color::Rgb(0, 255 / 7, 0),
		Color::Rgb(0, 255 / 6, 0),
		Color::Rgb(0, 255 / 5, 0),
		Color::Rgb(0, 255 / 4, 0),
		Color::Rgb(0, 255 / 3, 0),
		Color::Rgb(0, 255 / 2, 0),
		Color::Rgb(0, 255, 0),
	];

	#[allow(dead_code)]
	fn print_trees(&self) -> anyhow::Result<()> {
		let bufw = BufferWriter::stdout(termcolor::ColorChoice::Always);
		for (i, &height) in self.heights.iter().enumerate() {
			let mut buf = bufw.buffer();
			if i % self.width == 0 {
				writeln!(&mut buf)?;
			}
			buf.set_color(ColorSpec::new().set_fg(Some(Self::COLORS[height as usize])))
				.unwrap();
			write!(&mut buf, "X")?;
			bufw.print(&buf)?;
		}
		let mut buf = bufw.buffer();
		writeln!(&mut buf)?;
		buf.reset()?;
		bufw.print(&buf)?;
		Ok(())
	}

	fn count_visible(&self) -> usize {
		let (w, l) = self.size();
		(0..l)
			.flat_map(|y| (0..w).filter(move |&x| self.is_visible(x, y)))
			.count()
	}

	fn get_scenic_score(&self, x: usize, y: usize) -> usize {
		let mut score = [0; 4];
		let height = self.get(x, y);
		for x in (0..x).rev() {
			score[0] += 1;
			if self.get(x, y) >= height {
				break;
			}
		}
		for x in (x + 1)..self.width() {
			score[1] += 1;
			if self.get(x, y) >= height {
				break;
			}
		}
		for y in (0..y).rev() {
			score[2] += 1;
			if self.get(x, y) >= height {
				break;
			}
		}
		for y in (y + 1)..self.length() {
			score[3] += 1;
			if self.get(x, y) >= height {
				break;
			}
		}
		score.into_iter().product()
	}

	fn best_scenic_score(&self) -> usize {
		let (w, l) = self.size();
		(0..l)
			.flat_map(|y| (0..w).map(move |x| self.get_scenic_score(x, y)))
			.max()
			.unwrap_or(0)
	}
}

impl Day8 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(usize, usize)> {
		let input = self.input.as_cow_u8()?;
		let input = input.as_ref();

		let map = HeightMap::from(input);
		// println!("{}", &map);
		// map.print_trees()?;
		// map.print_visible();

		Ok((map.count_visible(), map.best_scenic_score()))
	}
}
