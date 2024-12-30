#![doc = include_str!("../p1.md")]

use common::*;
use itertools::Itertools;
use winnow::Parser;

mod common;

fn main() {
    util::DayInput::find::<24>().solve_with(solve);
}

/// # Problem
/// What is the number produced by the bits of the `z`-wires at the end of the simulation?
/// # Solution
/// The "simulation" can be calculated from the output nodes (`z`-wires) by going through the
/// connection graph. We assume that all `z`-wires are present in the `connections` data.
fn solve(input: impl AsRef<str>) -> u64 {
    // get initial states and connections
    let (mut states, connections) = parse_device
        .parse_next(&mut input.as_ref())
        .expect("parsable");

    connections
        .iter()
        .filter(|connection| connection.dest.0.starts_with("z"))
        .sorted_by_key(|connection| {
            connection
                .dest
                .0
                .trim_start_matches("z")
                .parse::<u8>()
                .unwrap()
        })
        .map(|connection| descend_graph_cached(&connection.dest, &connections, &mut states))
        .enumerate()
        .filter_map(|(i, state)| {
            if state {
                Some(2u64.pow(i as u32))
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 4);
    }
    #[test]
    fn larger_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE_LARGER")), 2024);
    }

    #[test]
    fn input_solvable() {
        assert_eq!(
            super::solve(include_str!("../../inputs/24")),
            66055249060558
        );
    }
}
