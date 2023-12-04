use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU8;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Day14 {
	/// The input file of "lines"
	#[clap(default_value_t = DataFrom::internal(2022, 14))]
	pub input: DataFrom,
	/// Render sand animation at scale
	#[clap(short, long)]
	pub render: Option<NonZeroU8>,
}

type Coord = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
	Air,
	Stone,
	Sand,
}

struct Area {
	offset_x: Coord,
	width: Coord,
	oob: Tile,
	data: Box<[Tile]>,
}

impl Display for Area {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		assert!(self.offset_x + self.width < 1000);
		for i in (0..3).rev() {
			for x in self.offset_x..(self.offset_x + self.width) {
				if x == self.offset_x || x == self.offset_x + self.width - 1 || x == 500 {
					let c = (x / (10 as Coord).pow(i).max(1)) % 10;
					write!(f, "{}", (c as u8 + b'0') as char)?;
				} else {
					write!(f, " ")?;
				}
			}
			writeln!(f)?;
		}
		for x in self.offset_x..(self.offset_x + self.width) {
			if x == 500 {
				write!(f, "+")?;
				continue;
			}
			let c = match self[(x, 0)] {
				Tile::Air => ' ',
				Tile::Stone => '█',
				Tile::Sand => '▒',
			};
			write!(f, "{c}")?;
		}
		writeln!(f)?;
		for y in 1..self.height() {
			for x in self.offset_x..(self.offset_x + self.width) {
				let c = match self[(x, y)] {
					Tile::Air => ' ',
					Tile::Stone => '█',
					Tile::Sand => '▒',
				};
				write!(f, "{c}")?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl FromStr for Area {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let (mut min_x, mut max_x, mut max_y) = (Coord::MAX, Coord::MIN, Coord::MIN);
		let paths: Vec<_> = input
			.lines()
			.map(|line| {
				let path: Vec<_> = line
					.split(" -> ")
					.map(|nums| {
						let (x, y) = nums.split_once(',').context("invalid number format")?;
						let x = x.parse::<Coord>().context("invalid number")?;
						let y = y.parse::<Coord>().context("invalid number")?;
						min_x = min_x.min(x);
						max_x = max_x.max(x);
						max_y = max_y.max(y);
						Ok((x, y))
					})
					.collect::<anyhow::Result<_>>()?;
				Ok(path)
			})
			.collect::<anyhow::Result<_>>()?;
		let height = max_y + 3;
		let offset_x = min_x.min(500 - height);
		let width = max_x.max(500 + height) - offset_x + 1;
		let data = vec![Tile::Air; width as usize * height as usize].into_boxed_slice();
		let mut area = Area {
			offset_x,
			width,
			oob: Tile::Air,
			data,
		};
		for x in offset_x..(offset_x + width) {
			area[(x, height - 1)] = Tile::Stone;
		}
		for path in paths {
			let mut path = path.into_iter();
			let Some(mut prev) = path.next() else {
				continue;
			};
			for step in path {
				let mut xs = prev.0..=step.0;
				let mut ys = prev.1..=step.1;
				if xs.start() > xs.end() {
					xs = *xs.end()..=*xs.start();
				}
				if ys.start() > ys.end() {
					ys = *ys.end()..=*ys.start();
				}
				for y in ys {
					for x in xs.clone() {
						area[(x, y)] = Tile::Stone;
					}
				}
				prev = step;
			}
		}
		Ok(area)
	}
}

impl Index<(Coord, Coord)> for Area {
	type Output = Tile;

	fn index(&self, (x, y): (Coord, Coord)) -> &Self::Output {
		if x < self.offset_x || x >= self.offset_x + self.width || y >= self.height() {
			// out of bounds is always air
			return &self.oob;
		}
		&self.data[(x - self.offset_x + y * self.width) as usize]
	}
}

impl IndexMut<(Coord, Coord)> for Area {
	fn index_mut(&mut self, (x, y): (Coord, Coord)) -> &mut Self::Output {
		if x < self.offset_x || x >= self.offset_x + self.width || y >= self.height() {
			// out of bounds is always air and will always be so
			self.oob = Tile::Air;
			return &mut self.oob;
		}
		&mut self.data[(x - self.offset_x + y * self.width) as usize]
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveResult {
	HitNormal,
	HitBottom,
	Full,
}

impl Area {
	#[allow(clippy::cast_possible_truncation)]
	fn height(&self) -> Coord {
		self.data.len() as Coord / self.width
	}

	/// Returns if it is filled or flows into the abyss
	fn spawn_and_move_sand(&mut self) -> MoveResult {
		let mut pos = (500, 0);
		loop {
			if pos.0 < self.offset_x
				|| pos.0 >= self.offset_x + self.width
				|| pos.1 >= self.height()
			{
				panic!("sand fell out of bounds, fencepost error?");
			} else if self[(pos.0, pos.1 + 1)] == Tile::Air {
				pos.1 += 1;
			} else if self[(pos.0 - 1, pos.1 + 1)] == Tile::Air {
				pos.1 += 1;
				pos.0 -= 1;
			} else if self[(pos.0 + 1, pos.1 + 1)] == Tile::Air {
				pos.1 += 1;
				pos.0 += 1;
			} else {
				break;
			}
		}
		self[pos] = Tile::Sand;
		if pos.1 == self.height() - 2 {
			MoveResult::HitBottom
		} else if pos == (500, 0) {
			MoveResult::Full
		} else {
			MoveResult::HitNormal
		}
	}

	fn render_image(&self, pixel_size: u32) -> image::RgbaImage {
		let mut img = image::RgbaImage::new(self.width as u32, self.height() as u32);
		for y in 0..self.height() {
			for x in self.offset_x..(self.offset_x + self.width) {
				let c = match self[(x, y)] {
					Tile::Air => image::Rgba([0, 0, 0, 255]),
					Tile::Stone => image::Rgba([189, 142, 62, 255]),
					Tile::Sand => image::Rgba([252, 196, 108, 255]),
				};
				img.put_pixel((x - self.offset_x) as u32, y as u32, c);
			}
		}
		if pixel_size > 1 {
			img = image::imageops::resize(
				&img,
				pixel_size * self.width as u32,
				pixel_size * self.height() as u32,
				image::imageops::FilterType::Nearest,
			);
		}
		img
	}
}

impl Day14 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(usize, usize)> {
		let input = self.input.as_cow_str()?;
		let input = input.as_ref();

		let mut area: Area = input.parse()?;
		// dbg!((area.offset_x, area.width));
		// println!("{area}");
		let mut count = 0;
		let mut count2;
		if let Some(mult) = self.render {
			let mult = mult.get();
			let mut gif = image::codecs::gif::GifEncoder::new_with_speed(
				std::fs::File::create("2022-14.gif")?,
				1,
			);
			gif.set_repeat(image::codecs::gif::Repeat::Infinite)?;
			loop {
				let done = area.spawn_and_move_sand();
				let delay = if done == MoveResult::HitNormal {
					50
				} else {
					1000
				};
				gif.encode_frame(image::Frame::from_parts(
					area.render_image(mult as u32),
					0,
					0,
					image::Delay::from_numer_denom_ms(delay, 1),
				))?;
				if done != MoveResult::HitNormal {
					break;
				}
				count += 1;
			}
			count2 = count + 1; // The +1 for the last dropped sand above that we didn't care about before, but we do now
			while area.spawn_and_move_sand() != MoveResult::Full {
				count2 += 1;
				gif.encode_frame(image::Frame::from_parts(
					area.render_image(mult as u32),
					0,
					0,
					image::Delay::from_numer_denom_ms(50, 1),
				))?;
			}
			count2 += 1; // For the last dropped sand
			gif.encode_frame(image::Frame::from_parts(
				area.render_image(mult as u32),
				0,
				0,
				image::Delay::from_numer_denom_ms(1000, 1),
			))?;
		} else {
			// println!("Initial State:\n{area}");
			while area.spawn_and_move_sand() == MoveResult::HitNormal {
				// if [1, 2, 5, 22, 24].contains(&count) {
				// 	println!("Step: {count}\n{area}");
				// }
				count += 1;
			}
			// println!("Step: {count}\n{area}");
			count2 = count + 1; // The +1 for the last dropped sand above that we didn't care about before, but we do now
			while area.spawn_and_move_sand() != MoveResult::Full {
				count2 += 1;
			}
			count2 += 1; // For the last dropped sand
		}

		Ok((count, count2))
	}
}
