#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![doc = include_str!("../p2.md")]

mod common;

use common::*;
use itertools::Itertools;
use std::collections::HashSet;

#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    prelude::*,
    stream::*,
    token::*,
    {PResult, Parser},
};

fn main() {
    util::DayInput::find::<24>().solve_with(solve);
}

/// # Problem
/// Given the same input as before, assume your system intends to do additions of `x` and `y`,
/// described by bits `x00...` and `y00...` with some middle nodes that cannot be removed.
/// If you *have* to swap the output of four pairs, each consisting of two different, arbitrary
/// connections/gates *exaclty*, and have it produce the output expected (addition of `x` and `y`),
/// then combine the outputs, sorted alphanumerically and joined with commas.
/// # Solution
/// We could assume that there are 8 unique wires involved,
/// such that swaps cannot cancel each other out or move in circles etc.
/// Since there is a constant cap on the amount of swaps (4),
/// a naive solution may be to just try all possible combinations of swaps:
/// `c*(c-1) + (c-2)*(c-3) + ... + (c-6)*(c-7)` possible connections?
/// Not great worst case time complexity, `O(n^2)`, but may work?
/// I think it only works if looking through states is as little overhead as possible, aka
/// reusing cache across multiple attempts.
/// We could build a sort of AST-like tree of value which can be naturally cached and partially
/// recalculated. We cannot reposition values though, so there is still some potentially large
/// amount of recalculation needed.
/// Instead of just trying every combination of connection output swaps, we could prune a lot of
/// branches by looking at whether some swap has made the correct solution impossible.
/// This can happen if we swap somewhere and the inputs are constant/based directly on initials,
/// and do not produce the correct bit.
/// For swaps, we do not need to go over all wire outputs, merely all *combinations of* wire
/// outputs; meaning we need to do around half as many as one might assume, and this is true for
/// every level, reducing search space perhaps by a factor of 2^4:
/// `(c*(c-1))/2 + ... ((c-6)*(c-7))/2`, still `O(n^2)` in the end.
/// The input connections are not *that* many though,
/// so perhaps we can consider `n` not that large and merely do some naive-ish solution.
fn solve(input: impl AsRef<str>) -> String {
    let (states, connections) = parse_device
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    // todo: implement addition based on states alone
    todo!("find the values to swap to find correct outcome")
}
/// no cached state, only initial, no swap memory (assume same-place allowed even if inefficient)
/// Note: `Connections` is a hashset, and does not provide mutation because of how elements are keys
fn with_swapped_remaining<'s>(
    initial_states: &States<'s>,
    initial_connections: &Connections,
) -> Option<[WireName<'s>; 4]> {
    fn sum_bits_ascending(iter: impl Iterator<Item = bool>) -> u32 {
        iter.enumerate()
            .map(|(i, state)| if state { 2u32.pow(i as u32) } else { 0 })
            .sum()
    }
    // todo: create verified valid output
    let x_sum = sum_bits_ascending(
        initial_states
            .iter()
            .filter(|(name, _)| name.0.starts_with("x"))
            .sorted_by_key(|(name, _)| name.0)
            .map(|(_, state)| *state),
    );
    let y_sum = sum_bits_ascending(
        initial_states
            .iter()
            .filter(|(name, _)| name.0.starts_with("y"))
            .sorted_by_key(|(name, _)| name.0)
            .map(|(_, state)| *state),
    );
    let expected_z_sum = x_sum + y_sum;

    let len = 0..initial_connections.len();
    len.clone()
        .into_iter()
        .combinations(4)
        .zip(len.into_iter().combinations(4))
        .filter_map(|(a, b)| {
            let mut swap_connections = initial_connections
                .iter()
                .cloned()
                .collect::<Vec<Connection>>();
            for (swap_a, swap_b) in a.into_iter().zip(b.into_iter()) {
                let a_dest = swap_connections[swap_a].dest;
                swap_connections[swap_a].dest = swap_connections[swap_b].dest;
                swap_connections[swap_b].dest = a_dest;
            }
            let swapped_connections = swap_connections.into_iter().collect::<HashSet<_>>();
            // WARN: you cannot naively descend graph, because a connection created naively may
            // be cyclical!
            todo!("descend graph, failing for cyclical graphs")
        })
        .next()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE_2")), "z00,z01,z02,z05");
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/24")), "");
    }
}
