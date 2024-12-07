#![feature(iter_map_windows)]

use itertools::Itertools;
use std::collections::hash_map::IntoValues;
#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

#[doc = include_str!("../p1.md")]
fn main() {
    util::DayInput::find::<4>().solve_with(solve);
}

/// # Constraints
/// The word may appear in any direction, all 8 directions (cardinal *and* diagonal)
/// Input is a rectangle, meaning every line is the same length (amount of characters)
/// # Solution
/// How many times do the word `XMAS` appear in the input?
/// ## Potential A:
/// Create an iterator for every "direction", then yield them all.
/// Each iterator skips some amount of characters.
/// If the width of the rectangle is 8, then a center-right iter would skip no characters,
/// but a down-left iter would skip some 7 characters. These iterators are only correct if the
/// next character in them are on the right line: The center-right iter does not yield four
/// characters at the 0+6th place (there are only 0..8 characters in this example).
/// To filter out these cases of each iterator,
/// we can either make sure the iterator fails for the case, or make it never happen.
/// Never making it happen, or never generating it, is possible by starting the iterator with a
/// certain offset or ending it with a certain offset.
/// ### Note:
/// The sample is 10 characters wide.
fn solve(input: impl AsRef<str>) -> u32 {
    let lines = input.as_ref().lines();
    let line_count = lines.clone().count();

    fn diagonal_iter<'s>(
        lines: impl Iterator<Item = &'s str>,
        compare_fn: impl Fn((usize, usize, char)) -> (usize, char),
    ) -> IntoValues<usize, Vec<char>> {
        lines
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (y, x, c)))
            .map(compare_fn)
            .into_group_map()
            .into_values()
    }
    let diagonals_down_right = diagonal_iter(lines.clone(), |(y, x, c)| (x + line_count - y, c));
    let diagonals_down_left =
        diagonal_iter(lines.clone(), |(y, x, c): (usize, usize, char)| (x + y, c));
    let verticals = lines
        .clone()
        .flat_map(|line| line.chars().enumerate())
        .into_group_map()
        .into_values();

    let horizontals = lines.flat_map(|line| line.chars().map_windows(|&seq: &[char; 4]| seq));

    // first 3 iterators are group mapped
    diagonals_down_right
        .chain(diagonals_down_left)
        .chain(verticals)
        .flat_map(|directional_line| {
            // directions get a slice to compare with
            directional_line
                .into_iter()
                .map_windows(|&seq: &[char; 4]| seq)
        })
        // horizontal direction (also has slices)
        .chain(horizontals)
        // if the four-long sequence is `XMAS` (or the reverse), it is counted.
        .filter(|&seq| seq == ['X', 'M', 'A', 'S'] || seq == ['S', 'A', 'M', 'X'])
        .count() as u32
}

#[cfg(test)]
mod p1test {
    const SAMPLE: &str = include_str!("SAMPLE");
    const SAMPLE_FILTERED: &str = include_str!("SAMPLE_FILTERED1");

    //    #[test]
    //    fn samples_are_different() {
    //        pub const SAMPLE: &str = include_str!("SAMPLE");
    //        assert_ne!(SAMPLE, SAMPLE);
    //    }

    #[test]
    fn sample_solves() {
        assert_eq!(super::solve(SAMPLE), 18);
    }

    #[test]
    fn sample_filtered_solves() {
        assert_eq!(super::solve(SAMPLE_FILTERED), 18);
    }

    #[test]
    fn sample_filtered_is_same() {
        assert_eq!(super::solve(SAMPLE), super::solve(SAMPLE_FILTERED));
    }

    //#[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/4")), 2536);
    }
}
