use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Parser)]
pub struct Day10 {
	/// The input file of "pipes"
	#[clap(default_value_t = DataFrom::internal(2023, 10))]
	pub input: DataFrom,
}

impl Day10 {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<(usize, usize)> {
		let input = self.input.as_cow_u8()?;
		let map = Map::new(&input)?;

		if app.verbose >= 2 {
			println!("{map}");
		}

		let score1 = map.start_loop.len() / 2;
		let score2 = map.count_inside;

		Ok((score1, score2))
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
	Empty = b'.',
	Vertical = b'|',
	Horizontal = b'-',
	TopRight = b'L',
	BottomRight = b'F',
	BottomLeft = b'7',
	TopLeft = b'J',
	Start = b'S',
}

impl Pipe {
	pub fn connects_down(self) -> bool {
		matches!(self, Pipe::Vertical | Pipe::BottomLeft | Pipe::BottomRight)
	}

	pub fn connects_up(self) -> bool {
		matches!(self, Pipe::Vertical | Pipe::TopLeft | Pipe::TopRight)
	}

	pub fn connects_left(self) -> bool {
		matches!(self, Pipe::Horizontal | Pipe::TopLeft | Pipe::BottomLeft)
	}

	pub fn connects_right(self) -> bool {
		matches!(self, Pipe::Horizontal | Pipe::TopRight | Pipe::BottomRight)
	}

	pub fn get_next_coord(self, this: Coord, from: Coord) -> anyhow::Result<Coord> {
		match self {
			Pipe::Empty => bail!("invalid empty pipe at {this:?}"),
			Pipe::Vertical if this.up() == from => Ok(this.down()),
			Pipe::Vertical if this.down() == from => Ok(this.up()),
			Pipe::Horizontal if this.left() == from => Ok(this.right()),
			Pipe::Horizontal if this.right() == from => Ok(this.left()),
			Pipe::TopRight if this.up() == from => Ok(this.right()),
			Pipe::TopRight if this.right() == from => Ok(this.up()),
			Pipe::BottomRight if this.right() == from => Ok(this.down()),
			Pipe::BottomRight if this.down() == from => Ok(this.right()),
			Pipe::BottomLeft if this.down() == from => Ok(this.left()),
			Pipe::BottomLeft if this.left() == from => Ok(this.down()),
			Pipe::TopLeft if this.left() == from => Ok(this.up()),
			Pipe::TopLeft if this.up() == from => Ok(this.left()),
			Pipe::Start => {
				bail!("can't get next coordinate from a Start at {this:?} from {from:?}")
			}
			_ => bail!("invalid pipe {self:?} at {this:?} from {from:?}"),
		}
	}
}

impl From<u8> for Pipe {
	fn from(value: u8) -> Self {
		match value {
			b'|' => Pipe::Vertical,
			b'-' => Pipe::Horizontal,
			b'L' => Pipe::TopRight,
			b'F' => Pipe::BottomRight,
			b'7' => Pipe::BottomLeft,
			b'J' => Pipe::TopLeft,
			b'S' => Pipe::Start,
			_ => Pipe::Empty,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(u16, u16);

impl Coord {
	pub fn up(self) -> Coord {
		Coord(self.0, self.1.wrapping_sub(1))
	}

	pub fn down(self) -> Coord {
		Coord(self.0, self.1.wrapping_add(1))
	}

	pub fn left(self) -> Coord {
		Coord(self.0.wrapping_sub(1), self.1)
	}

	pub fn right(self) -> Coord {
		Coord(self.0.wrapping_add(1), self.1)
	}

	pub fn get_pipe(self, map: &Map) -> Pipe {
		let idx = map.coord_to_idx(self);
		Pipe::from(map.map.get(idx).copied().unwrap_or(Pipe::Empty as u8))
	}
}

#[allow(dead_code)]
#[derive(Debug)]
enum CacheSide {
	UR(Coord),
	UL(Coord),
	DR(Coord),
	DL(Coord),
	Up(Coord),
	Down(Coord),
	Left(Coord),
	Right(Coord),
}

struct Map<'s> {
	width: usize,
	// height: usize,
	start: usize,
	map: &'s [u8],
	annotated_map: Box<[u8]>,
	start_loop: Vec<Coord>,
	count_inside: usize,
}

impl<'s> Display for Map<'s> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for y in 0..self.map.len() / self.width {
			// let start = y * self.width;
			// let end = start + self.width - 1;
			// f.write_str(std::str::from_utf8(&self.annotated_map[start..end]).unwrap())?;
			// f.write_str("\n")?;
			for x in 0..self.width {
				let idx = y * self.width + x;
				let c = self.annotated_map[idx];
				match c {
					b' ' => f.write_char('.')?,
					b'#' => f.write_char('#')?,
					// b'J' | b'L' | b'7' | b'F' | b'|' | b'-' => f.write_char('█')?,
					b'J' => f.write_char('╯')?,
					b'L' => f.write_char('╰')?,
					b'7' => f.write_char('╮')?,
					b'F' => f.write_char('╭')?,
					b'|' => f.write_char('│')?,
					b'-' => f.write_char('─')?,
					// b'J' => f.write_char('┛')?,
					// b'L' => f.write_char('┗')?,
					// b'7' => f.write_char('┓')?,
					// b'F' => f.write_char('┏')?,
					// b'|' => f.write_char('┃')?,
					// b'-' => f.write_char('━')?,
					c => f.write_char(c as char)?,
				}
			}
			f.write_char('\n')?;
		}
		Ok(())
	}
}

impl<'s> Map<'s> {
	fn coord_to_idx(&self, coord: Coord) -> usize {
		(coord.1 as usize) * self.width + (coord.0 as usize)
	}

	// Already verified it is valid in new so truncation cannot happen:
	#[allow(clippy::cast_possible_truncation)]
	fn idx_to_coord(&self, idx: usize) -> Coord {
		Coord((idx % self.width) as u16, (idx / self.width) as u16)
	}

	fn get_loop_from(&self, start: usize) -> anyhow::Result<Vec<Coord>> {
		let mut looped = Vec::with_capacity(1024);
		let start = self.idx_to_coord(start);

		let mut current = start;
		looped.push(current);
		if current.up().get_pipe(self).connects_down() {
			current = current.up();
		} else if current.down().get_pipe(self).connects_up() {
			current = current.down();
		} else if current.left().get_pipe(self).connects_right() {
			current = current.left();
		} else if current.right().get_pipe(self).connects_left() {
			current = current.right();
		} else {
			bail!("invalid start pipe at {:?}", current);
		}

		loop {
			let pipe = current.get_pipe(self);
			if current == start {
				break;
			}
			let prior = *looped.last().context("looped should never be empty")?;
			looped.push(current);
			current = pipe.get_next_coord(current, prior)?;
		}

		Ok(looped)
	}

	#[allow(clippy::too_many_lines)]
	pub fn new(map_data: &'s [u8]) -> anyhow::Result<Self> {
		let width = map_data
			.iter()
			.position(|&b| b == b'\n')
			.context("input has no newlines")?
			+ 1;
		if map_data.len() % width != 0 {
			bail!("input is not a rectangle");
		}
		let height = map_data.len() / width;
		if width >= u16::MAX as usize && height >= u16::MAX as usize {
			bail!("input is too large");
		}
		let start = map_data
			.iter()
			.position(|&b| b == Pipe::Start as u8)
			.context("input has no start")?;
		let mut map = Self {
			width,
			// height,
			start,
			start_loop: Vec::new(),
			map: map_data,
			annotated_map: vec![b' '; map_data.len()].into_boxed_slice(),
			count_inside: 0,
		};
		map.start_loop = map.get_loop_from(map.start)?;
		for &coord in &map.start_loop {
			let idx = map.coord_to_idx(coord);
			map.annotated_map[idx] = map.map[idx];
		}
		let first_pipe = map.idx_to_coord(
			map.annotated_map
				.iter()
				.position(|&b| b != b' ')
				.context("annotated map is empty")?,
		);
		if ![Pipe::BottomRight, Pipe::Start].contains(&first_pipe.get_pipe(&map)) {
			bail!("first pipe is not a bottom-right corner at {first_pipe:?}, malformed annotated map:\n{map}");
		}
		map.count_inside = map.flood_fill(CacheSide::DR(first_pipe));
		Ok(map)
	}

	#[allow(clippy::too_many_lines)]
	fn flood_fill(&mut self, entry: CacheSide) -> usize {
		use CacheSide::*;
		let mut filled_count = 0;
		let mut cache = Vec::with_capacity(1024);
		let mut done = vec![false; self.annotated_map.len()];
		cache.push(entry);
		while let Some(entry) = cache.pop() {
			let coord = match entry {
				UR(coord) | UL(coord) | DR(coord) | DL(coord) | Up(coord) | Down(coord)
				| Left(coord) | Right(coord) => coord,
			};
			let idx = self.coord_to_idx(coord);
			let tile = &mut self.annotated_map[idx];
			if done[idx] {
				continue;
			}
			done[idx] = true;
			match *tile {
				b' ' => {
					filled_count += 1;
					*tile = b'#';
					cache.extend([
						Down(coord.up()),
						Right(coord.left()),
						Left(coord.right()),
						Up(coord.down()),
					]);
				}
				b'|' if matches!(entry, UR(_) | DR(_) | Right(_)) => {
					cache.extend([
						Left(coord.right()),
						UL(coord.down().right()),
						DL(coord.up().right()),
					]);
				}
				b'|' if matches!(entry, UL(_) | DL(_) | Left(_)) => {
					cache.extend([
						Right(coord.left()),
						UR(coord.down().left()),
						DR(coord.up().left()),
					]);
				}
				b'-' if matches!(entry, UL(_) | UR(_) | Up(_)) => {
					cache.extend([
						Down(coord.up()),
						DL(coord.up().right()),
						DR(coord.up().left()),
					]);
				}
				b'-' if matches!(entry, DL(_) | DR(_) | Down(_)) => {
					cache.extend([
						Up(coord.down()),
						UL(coord.down().right()),
						UR(coord.down().left()),
					]);
				}
				b'J' if matches!(entry, UR(_) | DR(_) | DL(_) | Right(_) | Down(_)) => {
					cache.extend([
						Up(coord.down()),
						Left(coord.right()),
						UL(coord.down().right()),
						DL(coord.up().right()),
						UR(coord.down().left()),
					]);
				}
				b'J' if matches!(entry, UL(_)) => cache.push(DR(coord.up().left())),
				b'L' if matches!(entry, UL(_) | DL(_) | DR(_) | Left(_) | Down(_)) => {
					cache.extend([
						Up(coord.down()),
						Right(coord.left()),
						UR(coord.down().left()),
						DR(coord.up().left()),
						UL(coord.down().right()),
					]);
				}
				b'L' if matches!(entry, UR(_)) => cache.push(DL(coord.up().right())),
				b'F' if matches!(entry, UL(_) | UR(_) | DL(_) | Left(_) | Up(_)) => {
					cache.extend([
						Down(coord.up()),
						Right(coord.left()),
						DR(coord.up().left()),
						UR(coord.down().left()),
						DL(coord.up().right()),
					]);
				}
				b'F' if matches!(entry, DR(_)) => cache.push(UL(coord.down().right())),
				b'7' if matches!(entry, UL(_) | UR(_) | DR(_) | Right(_) | Up(_)) => {
					cache.extend([
						Down(coord.up()),
						Left(coord.right()),
						DL(coord.up().right()),
						UL(coord.down().right()),
						DR(coord.up().left()),
					]);
				}
				b'7' if matches!(entry, DL(_)) => cache.push(UR(coord.down().left())),
				b'S' if matches!(entry, DR(_)) => {
					cache.push(UL(coord.down().right()));
				}
				_ => {}
			}
		}
		filled_count
	}
}

#[cfg(test)]
mod tests {
	crate::run_basic_tests!(
		super::Day10,
		simple_example: (".....
.S-7.
.|.|.
.L-J.
.....
", (4, 1),
		),
		busy_example: ("-L|F7
7S-7|
L|7||
-L-J|
L|-JF
", (4, 1),
		),
		count_inside: ("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
", (80, 10),
		),
	);
}
