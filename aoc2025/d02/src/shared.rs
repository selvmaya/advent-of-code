use std::{ops::RangeInclusive, str::FromStr};
use winnow::{
    self,
    ascii::dec_uint,
    combinator::{separated, separated_pair},
    Parser, Result as WResult,
};

pub(crate) const REAL_INPUT: &str = include_str!("../../inputs/d02");

pub(crate) fn digits(mut x: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    while x > 0 {
        digits.push((x % 10) as u8);
        x /= 10;
    }
    digits.reverse();
    digits
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Solution(pub u64);

pub(crate) struct Puzzle(pub Vec<ProductIdRange>);
impl FromStr for Puzzle {
    type Err = winnow::error::ContextError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        parse_puzzle(&mut s)
    }
}
pub(crate) fn parse_puzzle(input: &mut &str) -> WResult<Puzzle> {
    separated(0.., parse_product_id_range, ',')
        .parse_next(input)
        .map(Puzzle)
}

/// Assume product ranges are always increasing
pub(crate) struct ProductIdRange(pub RangeInclusive<u64>);

pub(crate) fn parse_product_id_range(input: &mut &str) -> WResult<ProductIdRange> {
    separated_pair(dec_uint, '-', dec_uint)
        .parse_next(input)
        .map(|(a, b)| ProductIdRange(a..=b))
}
