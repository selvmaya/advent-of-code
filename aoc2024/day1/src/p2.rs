#![doc = include_str!("../p2.md")]

#[cfg(test)]
mod common;
mod parse;

use std::collections::HashMap;

#[allow(unused_imports)]
use winnow::{ascii::*, combinator::*, error::*, prelude::*, token::*, Parser};

#[cfg_attr(test, allow(unused))]
fn main() {
    util::DayInput::find::<1>().solve_with(solve);
}

fn solve(input: impl AsRef<str>) -> u32 {
    let (left, right) = parse::parse_lists
        .parse_next(&mut input.as_ref())
        .expect("input still parsable");

    // find how many times each number in list 0 appears in list 1, then
    // multiply the number with its amount of appearances (in list 1), and sum
    // these together.

    //    let left = lists.0.into_iter().collect::<Vec<_>>();
    //    let counts = lists.1.into_iter().filter(|b|
    // left.contains(b)).counts();    left.into_iter()
    //        .map(|a| a * *counts.get(&a).unwrap_or(&0) as u32)
    //        .sum()

    let right = right.into_iter().fold(HashMap::new(), |mut acc, next_key| {
        acc.entry(next_key).and_modify(|a| *a += 1).or_insert(1);
        acc
    });
    left.into_iter()
        .filter_map(|key| right.get(&key).map(|val| key * val))
        .sum()
}

#[cfg(test)]
mod p2test {

    #[test]
    fn solve_sample() {
        // sample is reused from p1
        assert_eq!(super::solve(crate::common::SAMPLE), 31);
    }
}
