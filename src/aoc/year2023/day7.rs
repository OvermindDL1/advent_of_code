use crate::aoc::helpers::*;
use crate::AocApp;
use anyhow::bail;
use clap::Parser;
use itertools::Itertools;

#[derive(Debug, Parser)]
pub struct Day7 {
	/// The input file of ""
	#[clap(default_value_t = DataFrom::internal(2023, 7))]
	pub input: DataFrom,
}

impl Day7 {
	pub fn run(&self, _app: &AocApp) -> anyhow::Result<(u64, u64)> {
		let input = self.input.as_cow_str()?;
		let mut hands = Vec::with_capacity(512);
		for hand in input.trim().split('\n').map(|line| {
			if let Some((cards, bid)) = line.trim().split_once(' ') {
				let mut card_types = [CardValue::Two; 5];
				for (idx, card) in cards.as_bytes()[..5].iter().copied().enumerate() {
					card_types[idx] = card.try_into()?;
				}
				let bid = bid.parse::<u64>()?;
				Ok(Hand::new(card_types, bid))
			} else {
				bail!("Invalid line: {}", line)
			}
		}) {
			hands.push(hand?);
		}

		hands.sort_by(Hand::cmp_rank);

		// let mut final_count = 0;
		// for (rank, hand) in hands.iter().enumerate() {
		// 	let rank = rank as u64 + 1;
		// 	final_count += rank * hand.bid;
		// 	println!(
		// 		"{rank}: {:?} -> {:?} -> {} -> Score: {}",
		// 		hand.highest_type,
		// 		hand.cards.iter().format(" "),
		// 		hand.bid,
		// 		rank * hand.bid,
		// 	);
		// }
		// println!("Final score: {final_count}");

		let score1 = hands
			.iter()
			.enumerate()
			.map(|(idx, hand)| hand.bid * (idx as u64 + 1))
			.sum::<u64>();

		hands.iter_mut().for_each(Hand::convert_jacks_to_jokers);
		hands.sort_by(Hand::cmp_rank);
		// let mut final_count = 0;
		// for (rank, hand) in hands.iter().enumerate() {
		// 	let rank = rank as u64 + 1;
		// 	final_count += rank * hand.bid;
		// 	println!(
		// 		"{rank}: {:?} -> {:?} -> {} -> Score: {}",
		// 		hand.highest_type,
		// 		hand.cards.iter().format(" "),
		// 		hand.bid,
		// 		rank * hand.bid,
		// 	);
		// }
		// println!("Final score: {final_count}");
		let score2 = hands
			.iter()
			.enumerate()
			.map(|(idx, hand)| hand.bid * (idx as u64 + 1))
			.sum::<u64>();

		Ok((score1, score2))
	}
}

#[derive(Debug)]
struct Hand {
	cards: [CardValue; 5],
	bid: u64,
	// The bits of this correspond to the HandType
	highest_type: HandType,
}

#[repr(u8)]
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
enum HandType {
	HighCard = 1 << 0,
	OnePair = 1 << 1,
	TwoPairs = 1 << 2,
	ThreeOfAKind = 1 << 3,
	FullHouse = 1 << 4,
	FourOfAKind = 1 << 5,
	FiveOfAKind = 1 << 6,
}

#[repr(u8)]
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
enum CardValue {
	Joker,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

impl TryFrom<u8> for CardValue {
	type Error = anyhow::Error;

	fn try_from(card: u8) -> Result<Self, Self::Error> {
		Ok(match card {
			b'2' => CardValue::Two,
			b'3' => CardValue::Three,
			b'4' => CardValue::Four,
			b'5' => CardValue::Five,
			b'6' => CardValue::Six,
			b'7' => CardValue::Seven,
			b'8' => CardValue::Eight,
			b'9' => CardValue::Nine,
			b'T' => CardValue::Ten,
			b'J' => CardValue::Jack,
			b'Q' => CardValue::Queen,
			b'K' => CardValue::King,
			b'A' => CardValue::Ace,
			_ => bail!("Invalid card `{card}`"),
		})
	}
}

impl Hand {
	pub fn new(cards: [CardValue; 5], bid: u64) -> Self {
		let highest_type = Self::calculate_highest_card_type(cards);
		Self {
			cards,
			bid,
			highest_type,
		}
	}

	fn calculate_highest_card_type(mut cards: [CardValue; 5]) -> HandType {
		cards.sort();
		let jokers = cards
			.iter()
			.filter(|&&card| card == CardValue::Joker)
			.count();
		let cardsj = &cards[jokers..];
		let count_unique_values = cardsj.iter().copied().dedup().count();

		// Five of a kind
		if count_unique_values <= 1 {
			return HandType::FiveOfAKind;
		}
		// Four of a kind
		if count_unique_values <= 2
			&& cardsj
				.windows(4 - jokers)
				.any(|cards| cards.iter().copied().dedup().count() == 1)
		{
			return HandType::FourOfAKind;
		}
		// FullHouse
		if jokers <= 1 && count_unique_values == 2 && cardsj[0] == cardsj[1] && cards[3] == cards[4]
		{
			return HandType::FullHouse;
		}
		if jokers <= 3 && count_unique_values == 2 {
			return HandType::FullHouse;
		}
		if jokers == 4 {
			return HandType::FullHouse;
		}
		// Three of a kind
		if count_unique_values >= 2
			&& cardsj
				.windows(3 - jokers)
				.any(|cards| cards.iter().copied().dedup().count() == 1)
		{
			return HandType::ThreeOfAKind;
		}
		if jokers == 3 {
			return HandType::ThreeOfAKind;
		}
		// Two pairs
		if jokers == 0
			&& cards
				.iter()
				.copied()
				.tuple_windows()
				.filter(|(a, b)| a == b)
				.count() == 2
		{
			return HandType::TwoPairs;
		}
		if jokers == 2 && count_unique_values >= 2 {
			return HandType::TwoPairs;
		}
		// One pair
		if jokers == 2 || count_unique_values == 4 {
			return HandType::OnePair;
		}
		// High card
		HandType::HighCard
	}

	pub fn cmp_rank(&self, other: &Self) -> std::cmp::Ordering {
		match self.highest_type.cmp(&other.highest_type) {
			std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
			ordering => ordering,
		}
	}

	pub fn convert_jacks_to_jokers(&mut self) {
		for card in &mut self.cards {
			if *card == CardValue::Jack {
				*card = CardValue::Joker;
			}
		}
		self.highest_type = Self::calculate_highest_card_type(self.cards);
	}
}
