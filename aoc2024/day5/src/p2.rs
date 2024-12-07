mod p1;

use itertools::Itertools;
use p1::PageNumber;
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

#[doc = include_str!("../p2.md")]
fn main() {
    util::DayInput::find::<5>().solve_with(solve);
}

/// Use previous knowledge to create some sufficient sorted set, then apply to updates.
fn solve(input: impl AsRef<str>) -> u32 {
    let (x_smaller_than_y, upgrades) = p1::parse
        .parse_next(&mut input.as_ref())
        .expect("still parsable");

    let mut rules = x_smaller_than_y
        .into_iter()
        .into_group_map()
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().collect()))
        .collect::<HashMap<PageNumber, HashSet<_>>>();
    // build a sorted vec
    // this approach might not work
    let mut sorted_list = Vec::new();
    for (x, ys) in rules {
        let insert_pos = ys
            .iter()
            .filter_map(|y| sorted_list.iter().position(|n| n == y))
            .min()
            .unwrap_or(sorted_list.len()); // give the best chance for future elements
        sorted_list.insert(insert_pos, x)
    }
    // assume sorted correctly, we just need to run it against the upgrades and see if the
    // filtered version matches with them. If not, we can use the filtered version to get the
    // right result
    upgrades
        .into_iter()
        .filter_map(|upgrade| {
            let filtered_sorted = sorted_list
                .iter()
                .copied()
                .filter(|&a| upgrade.contains(&a))
                .collect::<Vec<_>>();
            if filtered_sorted != upgrade {
                Some(filtered_sorted[filtered_sorted.len() / 2].0 as u32)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod p2test {
    const SAMPLE: &str = include_str!("SAMPLE");

    #[test]
    fn sample_solves_wrong() {
        assert_ne!(super::solve(SAMPLE), 205);
    }

    #[test]
    fn sample_solves() {
        assert_eq!(super::solve(SAMPLE), 123);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/5")), 0);
    }
}
