#![doc = include_str!("../p2.md")]

mod common;

use common::*;
use std::collections::{HashMap, HashSet};
use winnow::Parser;

fn main() {
    util::DayInput::find::<19>().solve_with(solve);
}

/// # Problem
/// What is the sum of each design's amount of arrangements/combinations (from available)?
fn solve(input: impl AsRef<str>) -> u64 {
    let (available, wanted_designs) = parse_input
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    let mut known_values: HashMap<StripePattern, u64> = HashMap::new();
    wanted_designs
        .into_iter()
        .map(|design| design.count_possible_cached(&available, &mut known_values))
        .sum()
}

impl StripePattern {
    fn count_possible_cached(
        self,
        initial: &HashSet<StripePattern>,
        known_values: &mut HashMap<StripePattern, u64>,
    ) -> u64 {
        if let Some(&num) = known_values.get(&self) {
            num
        } else {
            let subpattern_ways: u64 = self
                .single_removed_sub_patterns(initial)
                .map(|sub_pattern| sub_pattern.count_possible_cached(initial, known_values))
                .sum();
            let this_ways = if initial.contains(&self) {
                subpattern_ways + 1
            } else {
                subpattern_ways
            };
            // we now know the amount of ways to make this, reuse for other purposes
            known_values.insert(self, this_ways);
            this_ways
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 16);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(
            super::solve(include_str!("../../inputs/19")),
            606411968721181,
            "actual"
        );
    }
}
