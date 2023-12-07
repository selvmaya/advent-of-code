use itertools::{Itertools, PeekingNext};
use std::any::type_name;
use std::cell::Cell;
use std::char::TryFromCharError;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter};
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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Ranking {
    // lexicographically ordered enum, value ascending from top->bottom
    HighCard(Card),
    Pair(Card),
    TwoPair(OrderedPair),
    ThreeOfAKind(Card),
    FullHouse(OrderedPair),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}
impl<'a> Debug for Ranking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}",
            match self {
                Ranking::HighCard(a) => "HC",
                Ranking::Pair(a) => "PR",
                Ranking::TwoPair(pair) => "2P",
                Ranking::ThreeOfAKind(a) => "3K",
                Ranking::FullHouse(pair) => "FH",
                Ranking::FourOfAKind(a) => "4K",
                Ranking::FiveOfAKind(a) => "5K",
            },
            match self {
                Ranking::TwoPair(pair) | Ranking::FullHouse(pair) => format!("{:?}", pair),
                Ranking::HighCard(single)
                | Ranking::Pair(single)
                | Ranking::ThreeOfAKind(single)
                | Ranking::FourOfAKind(single)
                | Ranking::FiveOfAKind(single) => format!("{}  ", single),
            }
        )
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct OrderedPair {
    // lexicographically ordered struct, priority descending top->bottom
    highest: Card,
    lowest: Card,
}
impl<'a> Debug for OrderedPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}&{}", self.highest, self.lowest)
    }
}
impl From<(Card, Card)> for OrderedPair {
    fn from((highest, lowest): (Card, Card)) -> Self {
        OrderedPair { highest, lowest }
    }
}
impl<'a> From<&'a Hand> for Ranking {
    fn from(hand: &'a Hand) -> Self {
        let ((a_count, a), (b)) = {
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
            5 => Ranking::FiveOfAKind(*a),
            4 => Ranking::FourOfAKind(*a),
            3 => match b {
                Some((2, b)) => Ranking::FullHouse(OrderedPair::from((*a, *b))),
                _ => Ranking::ThreeOfAKind(*a),
            },
            2 => match b {
                Some((2, b)) => Ranking::TwoPair(OrderedPair::from((*a, *b))),
                _ => Ranking::Pair(*a),
            },
            1 => Ranking::HighCard(*a),
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
            Ordering::Equal => self.0.cmp(&other.0), // assume this is correct?
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

pub(crate) fn part_1(input: &str) -> String {
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
        }) // note: group_by is consecutive
        .fold(
            BTreeMap::new(),
            |mut bid_groupings: BTreeMap<Ranking, Number>, (hand, bid)| {
                *bid_groupings.entry(Ranking::from(&hand)).or_default() += bid;
                bid_groupings // when bids are summed together, they can be multiplied by rank "strength" (relative)
            },
        )
        .into_values()
        .enumerate() // defines strength by iteration order of BTreeMap
        .map(|(index, bid_sum)| (index as Number + 1) * bid_sum)
        .sum::<Number>()
        .to_string()
}
// instead of unique hands it should be unique hand TYPES that determine strength

pub(crate) fn part_2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
                Ranking::Pair(Card::Ace)
            );
        }
    }

    const TEST_INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT), "6440");
    }

    #[test]
    #[ignore]
    fn part_1_rank_test_1() {
        const EXTRA_INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
QQQJA 1";
        assert_eq!(part_1(EXTRA_INPUT), "6445");
    }

    #[test]
    // #[ignore]
    fn part_1_rank_test_2() {
        const EXTRA_INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
QQQJA 1";
        assert_eq!(part_1(EXTRA_INPUT), "6446");
    }

    mod regression {
        // only works on my own input
        use super::*;

        #[test]
        // #[cfg_attr(feature = "regression", ignore)]
        // #[ignore]
        fn not_previously_encountered() {
            let output = part_1(std::fs::read_to_string("input/day_7.txt").unwrap().as_str());
            assert_ne!(output, "250400505");
            assert_ne!(output, "33518063");
        }
    }

    #[test]
    #[ignore]
    fn part_2_works() {
        todo!();
        // assert_eq!(part_2(TEST_INPUT), "71503");
    }
}