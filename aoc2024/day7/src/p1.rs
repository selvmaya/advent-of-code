mod parse;

use itertools::{repeat_n, Itertools};
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
    util::DayInput::find::<7>().solve_with(solve);
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

fn solve(input: impl AsRef<str>) -> u64 {
    let equations = parse::parse_equations
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    equations
        .into_iter()
        .filter(|(test_value, operands)| {
            let operation_count = operands.len() - 1;
            repeat_n([Operation::Add, Operation::Mul].iter(), operation_count)
                .multi_cartesian_product()
                .any(|operations| {
                    let mut operands = operands.iter();
                    let first = operands.next().expect("some element");
                    operands
                        .zip(operations)
                        .fold(*first, |sum, (operand, operation)| {
                            //                            eprint!(" {:?} {}", operation, operand);
                            match operation {
                                Operation::Add => sum + operand,
                                Operation::Mul => sum * operand,
                            }
                        })
                        == *test_value
                })
        })
        //                .inspect(|(res, _)| eprintln!("! Line with {} was solvable.", res))
        .map(|(test_value, _)| test_value)
        .sum::<u64>()
}

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
pub struct PageNumber(pub u8);

#[cfg(test)]
mod p1test {
    const SAMPLE: &str = include_str!("SAMPLE");

    #[test]
    fn sample_solves() {
        assert_eq!(super::solve(SAMPLE), 3749);
    }

    //    #[ignore]
    #[test]
    fn input_solvable() {
        assert_ne!(
            super::solve(include_str!("../../inputs/7")),
            1228638627,
            "this is too low"
        );
        assert_eq!(super::solve(include_str!("../../inputs/7")), 4364915411363);
    }
}
