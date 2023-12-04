pub mod helpers;
pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;

use crate::AocApp;
use clap::Parser;

#[allow(clippy::module_name_repetitions)]
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
	/// Advent of Code 2022
	#[clap(name = "2022")]
	Year2022 {
		#[clap(subcommand)]
		day: year2022::Year2022,
	},
	/// Advent of Code 2022
	#[clap(name = "2023")]
	Year2023 {
		#[clap(subcommand)]
		day: year2023::Year2023,
	},
}

impl AocYear {
	pub fn run(&self, app: &AocApp) -> anyhow::Result<()> {
		match self {
			AocYear::Year2015 { day } => day.run(app),
			AocYear::Year2016 { day } => day.run(app),
			AocYear::Year2017 { day } => day.run(app),
			AocYear::Year2018 { day } => day.run(app),
			AocYear::Year2019 { day } => day.run(app),
			AocYear::Year2020 { day } => day.run(app),
			AocYear::Year2021 { day } => day.run(app),
			AocYear::Year2022 { day } => day.run(app),
			AocYear::Year2023 { day } => day.run(app),
		}
	}

	pub fn run_all(app: &AocApp) -> anyhow::Result<()> {
		year2015::Year2015::run_all(app)?;
		year2016::Year2016::run_all(app)?;
		year2017::Year2017::run_all(app)?;
		year2018::Year2018::run_all(app)?;
		year2019::Year2019::run_all(app)?;
		year2020::Year2020::run_all(app)?;
		year2021::Year2021::run_all(app)?;
		year2022::Year2022::run_all(app)?;
		year2023::Year2023::run_all(app)?;
		Ok(())
	}
}

#[macro_export]
macro_rules! run_days {
	($year:ident, $self:ident, $app:ident, [$($day:ident),* $(,)*]) => {
		match $self {
			$year::RunAll => $year::run_all($app),
			$(Self::$day(day) => {
				println!("### {} - {}", stringify!($year), stringify!($day));
				day.input.preload()?;
				let start = std::time::Instant::now();
				let res = day.run($app);
				let taken = start.elapsed();
				let (score1, score2) = anyhow::Context::context(res, "failed to run day")?;
				if !$app.hide_scores {
					println!("Step 1: {score1}");
					println!("Step 2: {score2}");
				}
				if $app.verbose >= 1 {
					println!("_{} Time Taken: {:?}_", stringify!($day), taken);
				}
				Ok(())
			})*
		}
	}
}

#[macro_export]
macro_rules! run_all_days {
	($self:ident, $app:ident, [$($day:ident),* $(,)*]) => {{
		{
			let _ = $app;
		}
		println!("## {}", stringify!($self));
		let start = std::time::Instant::now();
		$({
			$self::$day(clap::Parser::parse_from(&[] as &[&str])).run($app)?;
		})*
		if $app.verbose >= 1 {
			println!("_{} Time Taken: {:?}_", stringify!($self), start.elapsed());
		}
		Ok(())
	}}
}
