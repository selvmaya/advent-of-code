use glue::SolverFn;

pub const PARTS: &[SolverFn] = &[part_1::part_1, part_2::part_2];

#[cfg(test)]
const TEST_INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

mod part_1 {
	use super::*;

	use itertools::{Itertools, PeekingNext};
	use std::cmp::Ordering;
	use std::error::Error;
	use std::fmt::{Debug, Display, Formatter};
	use std::str::FromStr;
	use strum::{Display, EnumString};

	#[derive(Debug, Display, EnumString, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
	enum Card {
		// lexicographically ordered enum, value ascending top->bottom
		#[strum(serialize = "2")]
		Two,
		#[strum(serialize = "3")]
		Three,
		#[strum(serialize = "4")]
		Four,
		#[strum(serialize = "5")]
		Five,
		#[strum(serialize = "6")]
		Six,
		#[strum(serialize = "7")]
		Seven,
		#[strum(serialize = "8")]
		Eight,
		#[strum(serialize = "9")]
		Nine,
		#[strum(serialize = "T")]
		Ten,
		#[strum(serialize = "J")]
		Jack,
		#[strum(serialize = "Q")]
		Queen,
		#[strum(serialize = "K")]
		King,
		#[strum(serialize = "A")]
		Ace,
	}

	#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
	enum Ranking {
		// lexicographically ordered enum, value ascending from top->bottom
		HighCard,
		Pair,
		TwoPair,
		ThreeOfAKind,
		FullHouse,
		FourOfAKind,
		FiveOfAKind,
	}

	impl<'a> From<&'a Hand> for Ranking {
		fn from(hand: &'a Hand) -> Self {
			let ((a_count, _), (b)) = {
				let mut sorted_cards = hand
					.0
					.iter()
					.sorted() // sort twice because dedup is consecutive only
					.dedup_with_count()
					.sorted()
					.rev();
				(
					sorted_cards.next().expect("Array cannot be empty."),
					sorted_cards.next(), // None if a_count is 5
				)
			};
			// eprintln!("{}:{}, {:?}", a, a_count, b);
			match a_count {
				5 => Ranking::FiveOfAKind,
				4 => Ranking::FourOfAKind,
				3 => match b {
					Some((2, _)) => Ranking::FullHouse,
					_ => Ranking::ThreeOfAKind,
				},
				2 => match b {
					Some((2, _)) => Ranking::TwoPair,
					_ => Ranking::Pair,
				},
				1 => Ranking::HighCard,
				_ => unreachable!(),
			}
		}
	}

	#[derive(PartialEq, Eq)] // ordering takes array order into account
	struct Hand(Box<[Card; 5]>);

	impl Display for Hand {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.0.iter().join("")) // e.g. "4A62Q" (laid out exactly as structure)
		}
	}

	impl Debug for Hand {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			let ranking = Ranking::from(self);
			write!(f, "{} [{:?}]", self, ranking)
		}
	}

	impl Ord for Hand {
		fn cmp(&self, other: &Self) -> Ordering {
			match Ranking::from(self).cmp(&Ranking::from(other)) {
				Ordering::Equal => self.0.cmp(&other.0),
				order => order,
			}
		}
	}

	impl PartialOrd for Hand {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.cmp(other))
		}
	}

	impl FromStr for Hand {
		type Err = Box<dyn Error>;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			Ok(Hand(Box::new(
				s.chars()
					.map(|c| c.to_string().parse::<Card>())
					.collect::<Result<Vec<_>, _>>()?
					.as_slice()
					.try_into()
					.unwrap(),
			)))
		}
	}

	type Number = u64;

	pub(super) fn part_1(input: &str) -> String {
		input
			.lines()
			.map(|line| {
				let (hand, bid) = line.split_once(' ').unwrap();
				(
					hand.parse::<Hand>().unwrap(),
					bid.parse::<Number>().unwrap(),
				)
			})
			.sorted_by(|(a_hand, _), (b_hand, _)| {
				// eprint!("Comparing {:?} with {:?}: ", a_hand, b_hand);
				a_hand.cmp(b_hand)
			})
			.enumerate() // defines strength by iteration order of BTreeMap
			.map(|(index, (_hand, bid))| (index as Number + 1) * bid)
			.sum::<Number>()
			.to_string()
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn part_1_works() {
			assert_eq!(part_1(TEST_INPUT), "6440");
		}

		#[test]
		fn part_1_rank_test_2() {
			const EXTRA_INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
QQQJA 1";
			assert_eq!(part_1(EXTRA_INPUT), "6446");
		}

		mod unit {
			use super::*;

			#[test]
			fn parsing_works() {
				let parsed: Hand = r"KAAAA".parse().unwrap();
				assert_eq!(
					parsed,
					Hand(Box::new([
						Card::King,
						Card::Ace,
						Card::Ace,
						Card::Ace,
						Card::Ace
					])),
					"parsing is not correct"
				);
			}

			#[test]
			fn ordering_works() {
				let a: Hand = r"KAAAA".parse().unwrap();
				let b: Hand = r"AAAAK".parse().unwrap();
				assert!(a < b, "ordering should be based on card placement");
			}

			#[test]
			fn ranking_works() {
				assert_eq!(
					Ranking::from(&r"A542A".parse::<Hand>().unwrap()),
					Ranking::Pair
				);
			}
		}

		mod regression {
			// only works on my own input
			use super::*;

			#[test]
			#[ignore]
			fn not_previously_encountered() {
				let output = part_1(
					std::fs::read_to_string("../input/day_07.txt")
						.unwrap()
						.as_str(),
				);
				assert_ne!(output, "250400505");
				assert_ne!(output, "33518063");
			}
		}
	}
}

mod part_2 {
	use super::*;

	use itertools::Itertools;
	use std::cmp::Ordering;
	use std::error::Error;
	use std::fmt::{Debug, Display, Formatter};
	use std::str::FromStr;
	use strum::{Display, EnumString};

	#[derive(Debug, Display, EnumString, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
	enum Card {
		// lexicographically ordered enum, value ascending top->bottom
		#[strum(serialize = "J")]
		Joker,
		#[strum(serialize = "2")]
		Two,
		#[strum(serialize = "3")]
		Three,
		#[strum(serialize = "4")]
		Four,
		#[strum(serialize = "5")]
		Five,
		#[strum(serialize = "6")]
		Six,
		#[strum(serialize = "7")]
		Seven,
		#[strum(serialize = "8")]
		Eight,
		#[strum(serialize = "9")]
		Nine,
		#[strum(serialize = "T")]
		Ten,
		// "Jack" card is "Joker" card for part 2
		#[strum(serialize = "Q")]
		Queen,
		#[strum(serialize = "K")]
		King,
		#[strum(serialize = "A")]
		Ace,
	}

	#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
	enum Ranking {
		// lexicographically ordered enum, value ascending from top->bottom
		HighCard,
		Pair,
		TwoPair,
		ThreeOfAKind,
		FullHouse,
		FourOfAKind,
		FiveOfAKind,
	}

	impl<'a> From<&'a Hand> for Ranking {
		fn from(hand: &'a Hand) -> Self {
			let (joker_count, (a, b)) = {
				let mut lowest_cards = hand
					.0
					.iter()
					.sorted() // sort first time (only by card type) to line up deduplication
					.dedup_with_count()
					.peekable();
				(
					if *lowest_cards.peek().unwrap().1 == Card::Joker {
						lowest_cards.next().unwrap().0
					} else {
						0
					},
					{
						// sort second time, this time prioritizing count
						let mut sorted_cards = lowest_cards.sorted().rev();
						(
							sorted_cards.next(), // None if 5 jokers
							sorted_cards.next(), // None if a count is None or 5
						)
					},
				)
			};
			let a_count = joker_count
				+ if let Some((a_count, _)) = a {
					a_count
				} else {
					0
				};
			match a_count {
				5 => Ranking::FiveOfAKind,
				4 => Ranking::FourOfAKind,
				3 => match b {
					Some((2, _)) => Ranking::FullHouse,
					_ => Ranking::ThreeOfAKind,
				},
				2 => match b {
					Some((2, _)) => Ranking::TwoPair,
					_ => Ranking::Pair,
				},
				1 => Ranking::HighCard,
				_ => unreachable!(),
			}
		}
	}

	#[derive(PartialEq, Eq)] // ordering takes array order into account
	struct Hand(Box<[Card; 5]>);
	impl Display for Hand {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.0.iter().join("")) // e.g. "4A62Q" (laid out exactly as structure)
		}
	}
	impl Debug for Hand {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			let ranking = Ranking::from(self);
			write!(f, "{} [{:?}]", self, ranking)
		}
	}

	impl Ord for Hand {
		fn cmp(&self, other: &Self) -> Ordering {
			match Ranking::from(self).cmp(&Ranking::from(other)) {
				Ordering::Equal => self.0.cmp(&other.0),
				order => order,
			}
		}
	}
	impl PartialOrd for Hand {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.cmp(other))
		}
	}

	impl FromStr for Hand {
		type Err = Box<dyn Error>;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			Ok(Hand(Box::new(
				s.chars()
					.map(|c| c.to_string().parse::<Card>())
					.collect::<Result<Vec<_>, _>>()?
					.as_slice()
					.try_into()
					.unwrap(),
			)))
		}
	}

	type Number = u64;

	pub(super) fn part_2(input: &str) -> String {
		input
			.lines()
			.map(|line| {
				let (hand, bid) = line.split_once(' ').unwrap();
				(
					hand.parse::<Hand>().unwrap(),
					bid.parse::<Number>().unwrap(),
				)
			})
			.sorted_by(|(a_hand, _), (b_hand, _)| {
				// eprint!("Comparing {:?} with {:?}: ", a_hand, b_hand);
				a_hand.cmp(b_hand)
			})
			.enumerate() // defines strength by iteration order of BTreeMap
			.map(|(index, (_hand, bid))| (index as Number + 1) * bid)
			.sum::<Number>()
			.to_string()
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn part_2_works() {
			assert!(Card::Joker < Card::Two);
			assert_eq!(part_2(TEST_INPUT), "5905");
		}
	}
}