use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Day11 {
	/// The input file of "monkeys"
	#[clap(default_value_t = DataFrom::Internal {year: 2022, day: 11})]
	pub input: DataFrom,
}

type MonkeyID = u8;
type Worry = u64;

#[derive(Debug, Clone)]
enum Op {
	Mult(Worry),
	Add(Worry),
	Pow2,
}

#[derive(Debug, Clone)]
struct Monkey {
	id: MonkeyID,
	worries: VecDeque<Worry>,
	operation: Op,
	test_div: Worry,
	if_true_throw_to: MonkeyID,
	if_false_throw_to: MonkeyID,
	inspections: u64,
}

impl Display for Monkey {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Monkey {}: ", self.id)?;
		for w in &self.worries {
			write!(f, "{} ", w)?;
		}
		Ok(())
	}
}

impl FromStr for Monkey {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (monkey_id_str, rest) = s
			.trim()
			.split_once('\n')
			.context("invalid monkey ID line")?;
		let id = match monkey_id_str
			.trim()
			.split_once(' ')
			.context("invalid monkey ID line definition")?
		{
			("Monkey", id) => id
				.trim_matches(|c| c == ' ' || c == ':')
				.parse::<u8>()
				.context("invalid monkey ID")?,
			unhandled => bail!("invalid monkey ID line definition: {unhandled:?}"),
		};

		let (worries_str, rest) = rest
			.trim()
			.split_once('\n')
			.context("invalid monkey items line")?;
		let ("Starting items", worries_str) = worries_str.trim().split_once(": ").context("invalid monkey items definition line")? else {
				bail!("invalid monkey items definition line");
			};
		let worries: VecDeque<Worry> = worries_str
			.trim()
			.split(", ")
			.map(Worry::from_str)
			.collect::<Result<_, _>>()
			.context("invalid item value")?;

		let (operation_str, rest) = rest
			.trim()
			.split_once('\n')
			.context("invalid monkey operation line")?;
		let ("Operation", operation_str) = operation_str.trim().split_once(": ").context("invalid monkey operation definition line")? else {
				bail!("invalid monkey operation definition line");
			};
		let ("new = old ", operation_str) = operation_str.trim().split_at("new = old ".len()) else {
				bail!("invalid monkey operation action");
			};
		let operation = match operation_str.trim().split_at(1) {
			("+", val) => Op::Add(
				val.trim()
					.parse()
					.context("invalid monkey operation value")?,
			),
			("*", " old") => Op::Pow2,
			("*", val) => Op::Mult(
				val.trim()
					.parse()
					.context("invalid monkey operation value")?,
			),
			unknown => bail!("invalid monkey operation: {unknown:?}"),
		};

		let (test_div_str, rest) = rest
			.trim()
			.split_once('\n')
			.context("invalid monkey test line")?;
		let ("Test: divisible by ", test_div_str) = test_div_str.trim().split_at("Test: divisible by ".len()) else {
				bail!("invalid monkey test definition line: {test_div_str}");
			};
		let test_div = test_div_str
			.trim()
			.parse()
			.context("invalid monkey test value")?;

		let (if_true_throw_to_str, if_false_throw_to) = rest
			.trim()
			.split_once('\n')
			.context("invalid monkey if * throw to line")?;
		let ("If true: throw to monkey ", if_true_throw_to_str) = if_true_throw_to_str.trim().split_at("If true: throw to monkey ".len()) else {
				bail!("invalid monkey if true throw to definition line: {if_true_throw_to_str}");
			};
		let ("If false: throw to monkey ", if_false_throw_to_str) = if_false_throw_to.trim().split_at("If false: throw to monkey ".len()) else {
				bail!("invalid monkey if false throw to definition line: {if_false_throw_to}");
			};
		let if_true_throw_to = if_true_throw_to_str
			.trim()
			.parse()
			.context("invalid monkey if true throw to value")?;
		let if_false_throw_to = if_false_throw_to_str
			.trim()
			.parse()
			.context("invalid monkey if false throw to value")?;

		Ok(Monkey {
			id,
			worries,
			operation,
			test_div,
			if_true_throw_to,
			if_false_throw_to,
			inspections: 0,
		})
	}
}

impl Monkey {
	fn inspect_and_throw<const DIV: Worry>(
		&mut self,
		test_mod: Worry,
	) -> Option<(MonkeyID, Worry)> {
		let worry = self.worries.pop_front()?;
		self.inspections += 1;
		let worry = match self.operation {
			Op::Mult(val) => worry * val,
			Op::Add(val) => worry + val,
			Op::Pow2 => worry * worry,
		};
		let worry = worry / DIV;
		let worry = worry % test_mod;
		let to = if (worry % self.test_div) == 0 {
			self.if_true_throw_to
		} else {
			self.if_false_throw_to
		};
		Some((to, worry))
	}
}

impl Day11 {
	fn round<const DIV: Worry>(monkeys: &mut [Monkey], test_mod: Worry) {
		for i in 0..monkeys.len() {
			while let Some((to, worry)) = monkeys[i].inspect_and_throw::<DIV>(test_mod) {
				monkeys[to as usize].worries.push_back(worry);
			}
		}
	}

	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let input = self.input.as_cow_str();
		let input = input.as_ref();

		let mut monkeys: Vec<Monkey> = input
			.split("\n\n")
			.map(Monkey::from_str)
			.collect::<anyhow::Result<_>>()?;
		monkeys.iter().enumerate().for_each(|(i, m)| {
			assert_eq!(i, m.id as usize);
		});
		let mut monkeys2 = monkeys.clone();

		let test_mod = monkeys.iter().map(|m| m.test_div).product::<Worry>();
		// dbg!(test_mod);

		// monkeys.iter().for_each(|m| println!("{m}"));
		// println!();
		for _round in 1..=20 {
			Self::round::<3>(&mut monkeys, test_mod);
			// println!("After round {round}, the monkeys are holding items with these worry levels:");
			// monkeys.iter().for_each(|m| println!("{m}"));
			// println!();
		}
		// for m in &monkeys {
		// 	println!("Monkey {} inspected items {} times.", m.id, m.inspections);
		// }
		// println!();

		monkeys.sort_by_key(|m| m.inspections);
		let (_worst, best) = monkeys.split_at(monkeys.len() - 2);
		let score1 = best.iter().map(|m| m.inspections).product::<u64>();

		// for m in &monkeys2 {
		// 	println!("Monkey {} inspected items {} times.", m.id, m.inspections);
		// }
		// println!();
		for _round in 1..=10_000 {
			Self::round::<1>(&mut monkeys2, test_mod);
			// if round == 1 || round == 20 || round % 1000 == 0 {
			// 	println!("== After round {round} ==");
			// 	for m in &monkeys2 {
			// 		println!("Monkey {} inspected items {} times.", m.id, m.inspections);
			// 	}
			// 	println!();
			// }
		}

		monkeys2.sort_by_key(|m| m.inspections);
		let (_worst, best) = monkeys2.split_at(monkeys.len() - 2);
		let score2 = best.iter().map(|m| m.inspections).product::<u64>();

		println!("Step 1: {}", score1);
		println!("Step 2: {}", score2);

		Ok(())
	}
}
