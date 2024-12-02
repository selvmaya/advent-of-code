#[allow(unused_imports)]
use winnow::{ascii::*, combinator::*, error::*, prelude::*, token::*, Parser};

mod common;
mod parse;

#[doc = include_str!("../p1.md")]
#[cfg_attr(test, allow(unused))]
fn main() {
    let res = solve(util::day_input::<1>().as_ref());
    println!("Result: {}", res);
}

fn solve(input: &str) -> u32 {
    let mut data = parse::parse_lists
        .parse(input)
        .expect("needs to be parsable");
    data.0.sort();
    data.1.sort();

    data.0
        .into_iter()
        .zip(data.1)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[cfg(test)]
pub mod p1test {
    use crate::common::SAMPLE;
    use crate::parse;
    use winnow::Parser;

    #[test]
    fn sample_parsing() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(
            parse::parse_lists.parse(SAMPLE).expect("can parse"),
            (v1, v2)
        );
    }

    #[test]
    fn solve_sample() {
        assert_eq!(super::solve(SAMPLE), 11);
    }
}
