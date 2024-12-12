//#![doc = include_str!("../p2.md")]

#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

fn main() {
    util::DayInput::find::<10>().solve_with(solve);
}

fn solve(input: impl AsRef<str>) -> u64 {
    todo!()
}
#[cfg(test)]
mod tests {

    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/10")), 0);
    }
}
