use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Day5 {
	/// The input file of lines as coordinates
	#[clap(default_value = "inputs/2021/day5.input")]
	pub input_file: PathBuf,
}

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Debug)]
struct Line(Point, Point);

struct Grid(Box<[u8]>, (usize, usize));

impl Debug for Grid {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "Grid: {}x{}", self.1 .0, self.1 .1)?;
		for array in self.0.chunks(self.1 .0) {
			for &cell in array {
				if cell == 0 {
					write!(f, ".")?;
				} else {
					write!(f, "{}", cell)?;
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl FromStr for Point {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, y) = s.split_once(',').context("invalid point line")?;
		Ok(Point {
			x: x.parse()?,
			y: y.parse()?,
		})
	}
}

impl FromStr for Line {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (point1, point2) = s.split_once(" -> ").context("invalid points line")?;
		let point1 = point1.parse()?;
		let point2 = point2.parse()?;
		Ok(Line(point1, point2))
	}
}

impl Line {
	fn get_minimum_size(&self) -> (i32, i32) {
		(self.0.x.max(self.1.x), self.0.y.max(self.1.y))
	}

	fn dir(&self) -> (i32, i32) {
		let ox = self.0.x.cmp(&self.1.x) as i32;
		let oy = self.0.y.cmp(&self.1.y) as i32;
		(-ox, -oy)
	}

	fn is_straight(&self) -> bool {
		self.is_straight_x() || self.is_straight_y()
	}

	fn is_straight_x(&self) -> bool {
		self.0.x == self.1.x
	}

	fn is_straight_y(&self) -> bool {
		self.0.y == self.1.y
	}
}

impl Grid {
	fn from_lines<'a>(lines: impl IntoIterator<Item = &'a Line>, size: (i32, i32)) -> Grid {
		let grid = vec![0u8; ((size.0 + 1) * (size.1 + 1)) as usize];
		let mut grid = Grid(
			grid.into_boxed_slice(),
			(size.0 as usize + 1, size.1 as usize + 1),
		);
		grid.fill_lines(lines);
		grid
	}

	fn fill_lines<'a>(&mut self, lines: impl IntoIterator<Item = &'a Line>) {
		for line in lines {
			let (ox, oy) = line.dir();
			let mut x = line.0.x;
			let mut y = line.0.y;
			while x != line.1.x || y != line.1.y {
				self.place(x, y);
				x += ox;
				y += oy;
			}
			self.place(line.1.x, line.1.y);
		}
	}

	fn place(&mut self, x: i32, y: i32) {
		let index = y as usize * self.1 .0 + x as usize;
		self.0[index] += 1;
	}

	fn count_above_1(&self) -> u32 {
		self.0.iter().filter(|&&cell| cell > 1).count() as u32
	}
}

impl Day5 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let lines = map_trimmed_nonempty_lines_of_file(&self.input_file, Line::from_str)?;
		let size = lines.iter().fold((0, 0), |(x, y), line| {
			let (min_x, min_y) = line.get_minimum_size();
			(x.max(min_x), y.max(min_y))
		});
		let grid_step1 = Grid::from_lines(lines.iter().filter(|l| l.is_straight()), size);
		let grid_step2 = Grid::from_lines(&lines, size);

		println!("Step 1: {}", grid_step1.count_above_1());
		println!("Step 2: {}", grid_step2.count_above_1());

		Ok(())
	}
}
