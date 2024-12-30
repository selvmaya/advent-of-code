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
    util::DayInput::find::<25>().solve_with(solve);
}

fn solve(input: impl AsRef<str>) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), todo!());
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/25")), todo!());
    }
}
