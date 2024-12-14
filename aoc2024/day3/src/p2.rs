#![doc = include_str!("../p2.md")]

#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

fn main() {
    util::DayInput::find::<3>().solve_with(solve);
}

#[derive(Debug, Clone)]
enum State {
    Read,
    Skip,
}
fn parse_state(input: &mut &str) -> PResult<Instruction> {
    alt(("do()".value(State::Read), "don't()".value(State::Skip)))
        .context(StrContext::Label("State instruction"))
        .parse_next(input)
        .map(Instruction::SetState)
}

#[derive(Debug, Clone)]
struct Mul(u32, u32);
fn parse_mul(input: &mut &str) -> PResult<Instruction> {
    delimited("mul(", separated_pair(dec_uint, ",", dec_uint), ")")
        .context(StrContext::Label("Mul instruction"))
        .parse_next(input)
        .map(|(a, b)| Instruction::Mul(Mul(a, b)))
}

#[derive(Debug, Clone)]
enum Instruction {
    SetState(State),
    Mul(Mul),
}
fn parse_instruction(input: &mut &str) -> PResult<Instruction> {
    alt((parse_state, parse_mul))
        .context(StrContext::Label("Instruction"))
        .parse_next(input)
}
fn parse_all_instructions(input: &mut &str) -> PResult<Vec<Instruction>> {
    repeat(
        1..,
        repeat_till(0.., any.void(), parse_instruction)
            .map(|(_, instruction): (Vec<_>, Instruction)| instruction),
    )
    .parse_next(input)
}

fn solve(input: impl AsRef<str>) -> u32 {
    // create vec of instructions
    parse_all_instructions(&mut input.as_ref())
        .expect("parsable")
        .into_iter()
        // execute instructions
        .fold((0, State::Read), |(sum, state), next_instruction| {
            match (state, next_instruction) {
                (_, Instruction::SetState(next_state)) => (sum, next_state),
                (prev @ State::Read, Instruction::Mul(Mul(a, b))) => (sum + a * b, prev),
                (prev @ State::Skip, Instruction::Mul(..)) => (sum, prev),
            }
        })
        .0
}

#[cfg(test)]
pub mod p2test {
    pub const SAMPLE2: &str = include_str!("SAMPLE2");

    #[test]
    fn samples_are_different() {
        pub const SAMPLE1: &str = include_str!("SAMPLE1");
        assert_ne!(SAMPLE2, SAMPLE1);
    }

    #[test]
    fn sample_solves() {
        assert_eq!(super::solve(SAMPLE2), 48);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/3")), 103811193);
    }
}
