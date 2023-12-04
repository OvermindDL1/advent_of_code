use crate::aoc::helpers::*;
use crate::AocApp;
use clap::Parser;
use itertools::Itertools;
use nom::branch::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::error::VerboseError;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Parser)]
pub struct Day13 {
	/// The input file of "number lists"
	#[clap(default_value_t = DataFrom::internal(2022, 13))]
	pub input: DataFrom,
}

type Value = u8;

#[derive(Clone, PartialEq, Eq)]
enum Data {
	Value(Value),
	List(Vec<Data>), // Top-most is always a List
}

impl PartialOrd for Data {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Data {
	fn cmp(&self, other: &Self) -> Ordering {
		fn compare_lists(a: &[Data], b: &[Data]) -> Ordering {
			for vals in a.iter().zip_longest(b.iter()) {
				use itertools::EitherOrBoth::*;
				match vals {
					Both(a, b) => {
						let cmp = a.cmp(b);
						if cmp != Ordering::Equal {
							return cmp;
						}
					}
					Right(_) => return Ordering::Less,
					Left(_) => return Ordering::Greater,
				}
			}
			Ordering::Equal
		}
		match (self, other) {
			(Data::Value(a), Data::Value(b)) => a.cmp(b),
			(Data::List(a), Data::List(b)) => compare_lists(a, b),
			(Data::Value(a), Data::List(b)) => compare_lists(&[Data::Value(*a)], b),
			(Data::List(a), Data::Value(b)) => compare_lists(a, &[Data::Value(*b)]),
		}
	}
}

impl Debug for Data {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Data::Value(v) => write!(f, "{v}"),
			Data::List(l) => {
				write!(f, "[")?;
				for (i, v) in l.iter().enumerate() {
					if i > 0 {
						write!(f, ",")?;
					}
					write!(f, "{v:?}")?;
				}
				write!(f, "]")
			}
		}
	}
}

impl Data {
	fn parse_number(input: &str) -> IResult<&str, Value, VerboseError<&str>> {
		let (input, value) = digit1(input)?;
		let value = value
			.parse()
			.expect("should never fail as digit1 parsed only digits");
		Ok((input, value))
	}

	fn parse_list(input: &str) -> IResult<&str, Vec<Data>, VerboseError<&str>> {
		delimited(
			char('['),
			separated_list0(char(','), Self::parse_data),
			char(']'),
		)(input)
	}

	fn parse_data(input: &str) -> IResult<&str, Data, VerboseError<&str>> {
		alt((
			map(Self::parse_number, Data::Value),
			map(Self::parse_list, Data::List),
		))(input)
	}

	fn parse_data_line(input: &str) -> IResult<&str, Data, VerboseError<&str>> {
		terminated(Self::parse_data, line_ending)(input)
	}

	fn parse_data_pair(input: &str) -> IResult<&str, (Data, Data), VerboseError<&str>> {
		pair(Self::parse_data_line, Self::parse_data_line)(input)
	}

	fn parse_list_of_data_pairs(
		input: &str,
	) -> Result<Vec<(Data, Data)>, nom::Err<VerboseError<&str>>> {
		let (input, list) = separated_list0(line_ending, Self::parse_data_pair)(input)?;
		eof(input)?;
		Ok(list)
	}
}

impl Day13 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(usize, usize)> {
		let input = self.input.as_cow_str()?;
		let input = input.as_ref();

		let data = Data::parse_list_of_data_pairs(input)
			.map_err(|e| anyhow::anyhow!("parse error: {e:?}"))?;

		// for (a, b) in &data {
		// 	println!("{a:?}");
		// 	println!("{b:?}");
		// 	println!("{:?}\n", a.cmp(b));
		// }

		let score1 = data
			.iter()
			.enumerate()
			.filter(|(_, (a, b))| a.cmp(b) != Ordering::Greater)
			.map(|(i, _)| i + 1)
			.sum::<usize>();

		let dividers = [
			Data::parse_data("[[2]]")
				.map_err(|e| anyhow::anyhow!("parse error: {e:?}"))?
				.1,
			Data::parse_data("[[6]]")
				.map_err(|e| anyhow::anyhow!("parse error: {e:?}"))?
				.1,
		];
		let mut data: Vec<Data> = data.into_iter().flat_map(|(a, b)| [a, b]).collect();
		data.extend(dividers.iter().cloned());
		data.sort();
		// dbg!(&data);

		let score2 = data
			.iter()
			.enumerate()
			.filter(|(_i, d)| dividers.contains(d))
			.map(|(i, _d)| i + 1)
			.product::<usize>();

		Ok((score1, score2))
	}
}
