#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

pub fn parse_equations(input: &mut &str) -> PResult<Vec<(u64, Vec<u64>)>> {
    separated(1.., parse_equation, line_ending).parse_next(input)
}
fn parse_equation(input: &mut &str) -> PResult<(u64, Vec<u64>)> {
    separated_pair(dec_uint, ": ", parse_operands).parse_next(input)
}
fn parse_operands(input: &mut &str) -> PResult<Vec<u64>> {
    separated(2.., dec_uint::<&str, u64, ContextError>, " ").parse_next(input)
}