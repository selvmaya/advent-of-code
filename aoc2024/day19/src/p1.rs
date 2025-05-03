#![doc = include_str!("../p1.md")]

use itertools::Itertools;
use std::collections::HashSet;
use winnow::Parser;

mod common;
use common::*;

fn main() {
    util::DayInput::find::<19>().solve_with(solve);
}

/// # Problem
/// How many designs are possible?
/// # Definitions
/// Designs are towels with patterns of colored stripes:
/// `ggw` is a double-green and white towel.
/// # Solution
/// If we can eat up the design by taking away from the pool of available patterns,
/// we can ensure a correct design exists. This is branching recursion, DFS.
/// ## Optimization
/// Trying to eat the design using *all* patterns may be unnecessary:
/// The amount of branches will explode while no progress is made.
/// Steady progress is better for the time complexity, one might intuit.
/// We can do this without changing the theoretical outcome by compacting.
/// A strategy would be to remove composite values from the patterns tried for each design.
/// The caching we use is the initial + all discovered composites. We should not remove anything
/// from this cache.
fn solve(input: impl AsRef<str>) -> usize {
    let (initial_available, wanted_designs) = parse_input
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    let compressed = compress(initial_available.clone());
    let mut cache = initial_available;
    wanted_designs
        .into_iter()
        .enumerate()
        .map(|(i, design)| {
            eprintln!("{}: {}", i, design.0.iter().join(""));
            design.test_possible_cached(&compressed, &mut cache)
        })
        .filter(|&result| result)
        .count()
}

fn compress(raw_patterns: HashSet<StripePattern>) -> HashSet<StripePattern> {
    raw_patterns
        .clone()
        // order of "removal" (or iteration) does not matter
        .into_iter()
        .filter(|p| {
            // keep this pattern if there are no subpatterns that also exist
            !p.single_removed_sub_patterns(&raw_patterns)
                .any(|sub_pattern| raw_patterns.contains(&sub_pattern))
        })
        .collect()
}

impl StripePattern {
    fn test_possible_cached(
        self,
        compressed: &HashSet<StripePattern>,
        validated: &mut HashSet<StripePattern>,
    ) -> bool {
        if validated.contains(&self) {
            true
        } else if self
            .single_removed_sub_patterns(compressed)
            .any(|sub_pattern| sub_pattern.test_possible_cached(compressed, validated))
        // a valid subtree could create pattern
        {
            // remember for other subtree branches or top level tests
            validated.insert(self);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 6);
    }

    //    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/19")), 280);
    }
}
