#[cfg(test)]
mod common;
mod parse;

use itertools::Itertools;
#[allow(unused_imports)]
use winnow::{ascii::*, combinator::*, error::*, prelude::*, token::*, Parser};

#[doc = include_str!("../p2.md")]
#[cfg_attr(test, allow(unused))]
fn main() {
    let similarity_score = solve(util::day_input::<1>().as_ref());
    println!("Similarity score: {}", similarity_score);
}

fn solve(mut input: &str) -> u32 {
    let lists = parse::parse_lists
        .parse_next(&mut input)
        .expect("input still parsable");

    // find how many times each number in list 0 appears in list 1, then multiply
    // the number with its amount of appearances (in list 1), and sum these
    // together.

    let left = lists.0.into_iter().collect::<Vec<_>>();
    let counts = lists.1.into_iter().filter(|b| left.contains(b)).counts();
    left.into_iter()
        .map(|a| a * *counts.get(&a).unwrap_or(&0) as u32)
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
