use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::{bail, Context};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day4 {
	/// The input file of bingo calls and cards
	#[clap(default_value_t = DataFrom::Internal {year: 2021, day: 4})]
	pub input: DataFrom,
}

#[derive(Debug, Clone, Copy)]
enum CardCell {
	Unmarked(u8),
	Marked(u8),
}

impl Default for CardCell {
	fn default() -> Self {
		CardCell::Unmarked(0)
	}
}

impl CardCell {
	fn value(self) -> u8 {
		match self {
			CardCell::Marked(v) | CardCell::Unmarked(v) => v,
		}
	}

	fn mark(&mut self) {
		*self = CardCell::Marked(self.value());
	}

	fn is_marked(self) -> bool {
		matches!(self, CardCell::Marked(_))
	}

	fn get_if_unmarked(self) -> Option<u8> {
		match self {
			CardCell::Unmarked(v) => Some(v),
			CardCell::Marked(_) => None,
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
struct Card([CardCell; 5 * 5]);

impl Card {
	fn set(&mut self, x: usize, y: usize, value: u8) {
		self.0[y * 5 + x] = CardCell::Unmarked(value);
	}

	fn call(&mut self, call: u8) -> bool {
		for cell in &mut self.0 {
			if cell.value() == call {
				cell.mark();
				return self.is_complete();
			}
		}
		false
	}

	fn is_complete(&self) -> bool {
		(0..5).any(|idx| {
			self.0[idx * 5..(idx + 1) * 5]
				.iter()
				.all(|cell| cell.is_marked())
				|| (idx..25).step_by(5).all(|idx| self.0[idx].is_marked())
		})
	}

	fn solution(&self, winning_num: u8) -> u32 {
		self.0
			.iter()
			.copied()
			.filter_map(CardCell::get_if_unmarked)
			.map(u32::from)
			.sum::<u32>()
			* u32::from(winning_num)
	}
}

impl Day4 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<()> {
		let mut calls = Vec::with_capacity(1024);
		let mut cards = Vec::with_capacity(128);
		let mut card: Card = Card::default();
		let mut card_line = 0;
		process_trimmed_nonempty_lines_of_file(&self.input, |line| {
			if calls.is_empty() {
				for num in line.split(',') {
					calls.push(num.parse::<u8>().context("Failed to parse number")?);
				}
			} else {
				for (i, num) in line
					.split(' ')
					.map(str::trim)
					.filter(|s| !s.is_empty())
					.enumerate()
				{
					card.set(
						card_line,
						i,
						num.parse::<u8>().context("Failed to parse number")?,
					);
				}
				card_line += 1;
				if card_line == 5 {
					cards.push(card);
					card_line = 0;
					card = Card::default();
				}
			}
			Ok(())
		})?;

		if card_line == 0 {
			bail!("card lines not multiple of 5");
		}

		let mut winning_nums = Vec::with_capacity(cards.len());
		{
			let mut cards = cards.as_mut_slice();
			for call in calls {
				if cards.is_empty() {
					break;
				}
				let mut card_idx = 0;
				while card_idx < cards.len() {
					if cards[card_idx].call(call) {
						cards.swap(0, card_idx);
						cards = &mut cards[1..];
						winning_nums.push(call);
					} else {
						card_idx += 1;
					}
				}
			}
		}

		println!(
			"Step 1: {}",
			cards
				.first()
				.context("no input cards")?
				.solution(*winning_nums.first().context("no input cards")?)
		);
		println!(
			"Step 2: {}",
			cards
				.last()
				.context("no input cards")?
				.solution(*winning_nums.last().context("no input cards")?)
		);

		Ok(())
	}
}
