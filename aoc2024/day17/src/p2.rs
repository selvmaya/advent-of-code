//#![doc = include_str!("../p2.md")]

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
    util::DayInput::find::<13>().solve_with(solve);
}

/// # Problem
/// The lowest possible initial value for register A,
/// that makes the program output itself (its source program)?
/// # Approach
/// This must have something to do with optimization.
/// Brute forcing probably wouldn't work.
/// I can maybe analyze my input for specific patterns that can be optimized,
/// even if it doesn't generalize to other solutions?
/// I should start by doing the naive solution for the example though.
fn solve(input: impl AsRef<str>) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE_P2")), 117440);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/17")), 0);
    }
}
