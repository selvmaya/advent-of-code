#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

mod common;

#[doc = include_str!("p1.rs")]
fn main() {
    util::DayInput::find::<3>().solve_with(solve);
}

fn solve(input: impl AsRef<str>) -> u32 {
    parse_all_components
        .parse_next(&mut input.as_ref())
        .expect("parsable")
        .into_iter()
        .map(|(a, b)| a * b)
        .sum()
}

fn parse_all_components(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    repeat(
        1..,
        repeat_till(0.., any.void(), parse_components).map(|(_, ab): (Vec<_>, _)| ab),
    )
    .parse_next(input)
}

fn parse_components(input: &mut &str) -> PResult<(u32, u32)> {
    delimited("mul(", separated_pair(dec_uint, ',', dec_uint), ")".void()).parse_next(input)
}

#[cfg(test)]
mod p1test {
    #[test]
    fn sample_solves() {
        assert_eq!(super::solve(super::common::SAMPLE), 161);
    }
}
