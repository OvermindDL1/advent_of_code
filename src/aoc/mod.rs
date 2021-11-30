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
	#[clap(name = "2015")]
	Year2015 {
		#[clap(subcommand)]
		year: year2015::Year2015,
	},
	#[clap(name = "2016")]
	Year2016 {
		#[clap(subcommand)]
		year: year2016::Year2016,
	},
	#[clap(name = "2017")]
	Year2017 {
		#[clap(subcommand)]
		year: year2017::Year2017,
	},
	#[clap(name = "2018")]
	Year2018 {
		#[clap(subcommand)]
		year: year2018::Year2018,
	},
	#[clap(name = "2019")]
	Year2019 {
		#[clap(subcommand)]
		year: year2019::Year2019,
	},
	#[clap(name = "2020")]
	Year2020 {
		#[clap(subcommand)]
		year: year2020::Year2020,
	},
	#[clap(name = "2021")]
	Year2021 {
		#[clap(subcommand)]
		year: year2021::Year2021,
	},
}

impl AocYear {
	pub fn run(&self) -> anyhow::Result<()> {
		match self {
			AocYear::Year2015 { year } => year.run(),
			AocYear::Year2016 { year } => year.run(),
			AocYear::Year2017 { year } => year.run(),
			AocYear::Year2018 { year } => year.run(),
			AocYear::Year2019 { year } => year.run(),
			AocYear::Year2020 { year } => year.run(),
			AocYear::Year2021 { year } => year.run(),
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
