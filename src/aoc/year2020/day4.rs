use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::Context;
use clap::Parser;
use smol_str::SmolStr;

#[derive(Debug, Parser)]
pub struct Day4 {
	/// The input file to use with the parseable data blank line delimited
	#[clap(default_value_t = DataFrom::internal(2020, 4))]
	pub input: DataFrom,
}

#[derive(Default)]
struct Passport {
	byr: SmolStr,
	iyr: SmolStr,
	eyr: SmolStr,
	hgt: SmolStr,
	hcl: SmolStr,
	ecl: SmolStr,
	pid: SmolStr,
	cid: SmolStr,
}

impl Passport {
	fn is_valid(&self) -> bool {
		!self.byr.is_empty()
			&& !self.iyr.is_empty()
			&& !self.eyr.is_empty()
			&& !self.hgt.is_empty()
			&& !self.hcl.is_empty()
			&& !self.ecl.is_empty()
			&& !self.pid.is_empty()
	}

	fn is_full_valid(&self) -> bool {
		match self.byr.parse::<usize>() {
			Ok(v) if (1920..=2002).contains(&v) => (),
			_ => return false,
		}
		match self.iyr.parse::<usize>() {
			Ok(v) if (2010..=2020).contains(&v) => (),
			_ => return false,
		}
		match self.eyr.parse::<usize>() {
			Ok(v) if (2020..=2030).contains(&v) => (),
			_ => return false,
		}
		if self.hgt.ends_with("cm") {
			let size = self
				.hgt
				.trim_end_matches("cm")
				.parse::<usize>()
				.unwrap_or(0);
			if !(150..=193).contains(&size) {
				return false;
			}
		} else if self.hgt.ends_with("in") {
			let size = self
				.hgt
				.trim_end_matches("in")
				.parse::<usize>()
				.unwrap_or(0);
			if !(59..=76).contains(&size) {
				return false;
			}
		} else {
			return false;
		}
		if !self.hcl.starts_with('#') || self.hcl.len() != 7 {
			return false;
		}
		if !self.hcl.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
			return false;
		}
		match self.ecl.as_str() {
			"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
			_ => return false,
		}
		if self.pid.len() != 9 {
			return false;
		}
		self.pid.chars().all(|c| c.is_ascii_digit())
	}
}

impl Day4 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(usize, usize)> {
		let mut current = Passport::default();
		let mut passports = Vec::with_capacity(512);
		process_trimmed_lines_of_file(&self.input, |line| {
			if line.is_empty() {
				passports.push(std::mem::take(&mut current));
			} else {
				for kv in line.split_whitespace().map(|kv| {
					kv.split_once(':')
						.context("invalid key value pair, missing `:`")
				}) {
					let (key, value) = kv?;
					match key {
						"byr" => current.byr = value.into(),
						"iyr" => current.iyr = value.into(),
						"eyr" => current.eyr = value.into(),
						"hgt" => current.hgt = value.into(),
						"hcl" => current.hcl = value.into(),
						"ecl" => current.ecl = value.into(),
						"pid" => current.pid = value.into(),
						"cid" => current.cid = value.into(),
						_ => anyhow::bail!("invalid key: {}", key),
					}
				}
			}
			Ok(())
		})?;
		passports.push(current);

		let score1 = passports.iter().filter(|p| p.is_valid()).count();

		let score2 = passports.iter().filter(|p| p.is_full_valid()).count();

		Ok((score1, score2))
	}
}
