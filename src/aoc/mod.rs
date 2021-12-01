pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum AocYear {
	/// Advent of Code 2015
	#[clap(name = "2015")]
	Year2015 {
		#[clap(subcommand)]
		day: year2015::Year2015,
	},
	/// Advent of Code 2016
	#[clap(name = "2016")]
	Year2016 {
		#[clap(subcommand)]
		day: year2016::Year2016,
	},
	/// Advent of Code 2017
	#[clap(name = "2017")]
	Year2017 {
		#[clap(subcommand)]
		day: year2017::Year2017,
	},
	/// Advent of Code 2018
	#[clap(name = "2018")]
	Year2018 {
		#[clap(subcommand)]
		day: year2018::Year2018,
	},
	/// Advent of Code 2019
	#[clap(name = "2019")]
	Year2019 {
		#[clap(subcommand)]
		day: year2019::Year2019,
	},
	/// Advent of Code 2020
	#[clap(name = "2020")]
	Year2020 {
		#[clap(subcommand)]
		day: year2020::Year2020,
	},
	/// Advent of Code 2021
	#[clap(name = "2021")]
	Year2021 {
		#[clap(subcommand)]
		day: year2021::Year2021,
	},
}

impl AocYear {
	pub fn run(&self) -> anyhow::Result<()> {
		match self {
			AocYear::Year2015 { day } => day.run(),
			AocYear::Year2016 { day } => day.run(),
			AocYear::Year2017 { day } => day.run(),
			AocYear::Year2018 { day } => day.run(),
			AocYear::Year2019 { day } => day.run(),
			AocYear::Year2020 { day } => day.run(),
			AocYear::Year2021 { day } => day.run(),
		}
	}
}

#[macro_export]
macro_rules! run_days {
	($self:ident, [$($day:ident),* $(,)*]) => {
		match $self {
			$(Self::$day(day) => day.run(),)*
		}
	}
}
