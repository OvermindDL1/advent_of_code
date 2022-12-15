use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Day15 {
	/// The input file of "Sensor and Beacon locations"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 15})]
	pub input: DataFrom,
}

type Coord = i64;

struct Sensor {
	loc: (Coord, Coord),
	beacon: (Coord, Coord),
}

impl FromStr for Sensor {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let (_, input) = input.split_at(b"Sensor at x=".len());
		let (sx, input) = input.split_once(", y=").context("missing sensor ', y='")?;
		let (sy, input) = input
			.split_once(": closest beacon is at x=")
			.context("missing ': closest beacon is at x='")?;
		let (bx, by) = input.split_once(", y=").context("missing beacon ', y='")?;
		Ok(Sensor {
			loc: (sx.parse()?, sy.parse()?),
			beacon: (bx.parse()?, by.parse()?),
		})
	}
}

impl Sensor {
	fn range(&self) -> Coord {
		(self.loc.0 - self.beacon.0).abs() + (self.loc.1 - self.beacon.1).abs()
	}

	fn in_known_range(&self, x: Coord, y: Coord) -> bool {
		let range = self.range();
		let x = (x - self.loc.0).abs();
		let y = (y - self.loc.1).abs();
		x + y <= range
	}

	fn bounds_overlap(&self, x: RangeInclusive<Coord>, y: RangeInclusive<Coord>) -> bool {
		let range = self.range();
		if *x.start() > self.loc.0 + range || *x.end() < self.loc.0 - range {
			return false;
		}
		if *y.start() > self.loc.1 + range || *y.end() < self.loc.1 - range {
			return false;
		}
		true
	}
}

struct Sensors {
	sensors: Vec<Sensor>,
	bounds: (RangeInclusive<Coord>, RangeInclusive<Coord>),
}

impl FromStr for Sensors {
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let sensors = input
			.lines()
			.map(Sensor::from_str)
			.collect::<anyhow::Result<Vec<_>>>()?;
		let (mut min_x, mut max_x) = (0, 0);
		let (mut min_y, mut max_y) = (0, 0);
		for s in &sensors {
			let range = s.range();
			min_x = min_x.min(s.loc.0 - range);
			max_x = max_x.max(s.loc.0 + range);
			min_y = min_y.min(s.loc.1 - range);
			max_y = max_y.max(s.loc.1 + range);
		}
		Ok(Sensors {
			sensors,
			bounds: (min_x..=max_x, min_y..=max_y),
		})
	}
}

impl Display for Sensors {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut height_max = self.bounds.0.start().abs().max(self.bounds.0.end().abs());
		let mut height_digits = 0;
		while height_max > 0 {
			height_digits += 1;
			height_max = height_max / 10;
		}
		let mut width_max = self.bounds.1.start().abs().max(self.bounds.1.end().abs());
		let mut width_digits = 0;
		while width_max > 0 {
			width_digits += 1;
			width_max = width_max / 10;
		}
		for _ in 0..=width_digits {
			write!(f, " ")?;
		}
		write!(f, "  ")?;
		for x in self.bounds.0.clone() {
			if x < 0 && x % 5 == 0 {
				write!(f, "-")?;
			} else {
				write!(f, " ")?;
			}
		}
		writeln!(f)?;
		for i in (0..=height_digits).rev() {
			for _ in 0..=width_digits {
				write!(f, " ")?;
			}
			write!(f, "  ")?;
			for x in self.bounds.0.clone() {
				if x % 5 == 0 {
					let c = (x.abs() / (10 as Coord).pow(i).max(1)) % 10;
					write!(f, "{}", (c as u8 + b'0') as char)?;
				} else {
					write!(f, " ")?;
				}
			}
			writeln!(f)?;
		}
		for y in self.bounds.1.clone() {
			write!(f, "{y:width$} ", width = width_digits + 1)?; // The +1 in case negative
			self.for_each_cell(self.bounds.0.clone(), y..=y, |_x, _y, cell| {
				let c = match cell {
					Data::Unknown => '?',
					Data::KnownEmpty => ' ',
					Data::Beacon => 'B',
					Data::Sensor => 'S',
				};
				write!(f, "{}", c)
			})?;
			writeln!(f)?;
		}
		Ok(())
	}
}

impl Sensors {
	/// Calls back along Y first, then X, in range order.
	fn fold_cells<A, E>(
		&self,
		acc: A,
		x: RangeInclusive<Coord>,
		y: RangeInclusive<Coord>,
		mut f: impl FnMut(A, Coord, Coord, Data) -> Result<A, E>,
	) -> Result<A, E> {
		let mut acc = Some(acc);
		self.for_each_cell(x, y, |x, y, cell| {
			acc = Some(f(acc.take().expect("invalid fold state"), x, y, cell)?);
			Ok(())
		})?;
		Ok(acc.take().expect("invalid fold state"))
	}

	/// Calls back along Y first, then X, in range order.
	fn for_each_cell<E>(
		&self,
		x: RangeInclusive<Coord>,
		y: RangeInclusive<Coord>,
		mut f: impl FnMut(Coord, Coord, Data) -> Result<(), E>,
	) -> Result<(), E> {
		let mut line_cache = Vec::new();
		let mut cache: Option<&Sensor> = None;
		for y in y {
			line_cache.clear();
			self.fill_sensors_in_range(&mut line_cache, x.clone(), y..=y);
			for x in x.clone() {
				if let Some(s) = cache {
					if !s.in_known_range(x, y) {
						cache = None;
					}
				}
				if cache.is_none() {
					cache = line_cache.iter().find(|s| s.in_known_range(x, y)).cloned();
				}
				let cell = if let Some(s) = cache {
					if s.beacon == (x, y) {
						Data::Beacon
					} else if s.loc == (x, y) {
						Data::Sensor
					} else {
						Data::KnownEmpty
					}
				} else {
					Data::Unknown
				};
				f(x, y, cell)?;
			}
		}
		Ok(())
	}

	fn fill_sensors_in_range<'s>(
		&'s self,
		sensors: &mut Vec<&'s Sensor>,
		x: RangeInclusive<Coord>,
		y: RangeInclusive<Coord>,
	) {
		for s in &self.sensors {
			if s.bounds_overlap(x.clone(), y.clone()) {
				sensors.push(s);
			}
		}
	}

	fn find_empty(
		&self,
		xr: RangeInclusive<Coord>,
		yr: RangeInclusive<Coord>,
	) -> Option<(Coord, Coord)> {
		fn get_range_on_y(s: &Sensor, y: Coord) -> RangeInclusive<Coord> {
			let range = s.range();
			let x_range = range - (y - s.loc.1).abs();
			(s.loc.0 - x_range)..=(s.loc.0 + x_range)
		}
		fn rebuild_ranges_on_y(line_cache: &mut [(RangeInclusive<Coord>, &Sensor)], y: Coord) {
			for (range, s) in line_cache.iter_mut() {
				*range = get_range_on_y(s, y);
			}
		}
		fn rebuild_caches<'s>(
			this: &'s Sensors,
			line_cache: &mut Vec<(RangeInclusive<Coord>, &'s Sensor)>,
			xr: RangeInclusive<Coord>,
			y: Coord,
		) {
			line_cache.clear();
			for s in &this.sensors {
				if s.bounds_overlap(xr.clone(), y..=y) {
					line_cache.push((get_range_on_y(s, y), s));
				}
			}
			line_cache.sort_by_key(|(r, _s)| *r.start());
		}
		fn find_empty_in_cache(
			range_cache: &Vec<(RangeInclusive<Coord>, &Sensor)>,
			mut x: Coord,
		) -> Coord {
			for (r, _s) in range_cache {
				if r.contains(&x) {
					x = r.end() + 1;
				}
			}
			x
		}
		let mut line_cache: Vec<(RangeInclusive<Coord>, &Sensor)> = Vec::new();
		for y in yr {
			rebuild_ranges_on_y(&mut line_cache, y);
			let x = find_empty_in_cache(&line_cache, *xr.start());
			if x <= *xr.end() {
				rebuild_caches(self, &mut line_cache, xr.clone(), y);
				let x = find_empty_in_cache(&line_cache, *xr.start());
				if x <= *xr.end() {
					return Some((x, y));
				}
			}
		}
		None
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Data {
	Unknown,
	KnownEmpty,
	Beacon,
	Sensor,
}

impl Data {
	fn is_known_and_not_beacon(&self) -> bool {
		self != &Data::Unknown && self != &Data::Beacon
	}
}

impl Day15 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let input = self.input.as_cow_str();
		let input = input.as_ref();

		let sensors: Sensors = input.parse()?;
		// println!("{sensors}");

		let score1 = sensors.fold_cells::<_, anyhow::Error>(
			0,
			sensors.bounds.0.clone(),
			2000000..=2000000,
			|acc, _x, _y, cell| Ok(acc + i32::from(cell.is_known_and_not_beacon())),
		)?;

		println!("Step 1: {}", score1);

		let coords2 = sensors
			.find_empty(0..=4000000, 0..=4000000)
			.context("didn't find an unknown spot")?;
		println!("Step 2: {}", coords2.0 * 4000000 + coords2.1);

		Ok(())
	}
}
