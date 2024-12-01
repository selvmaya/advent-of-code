#[allow(unused_imports)]
use winnow::{ascii::*, combinator::*, error::*, prelude::*, token::*, Parser};

#[doc = include_str!("../p1.md")]
fn main() {
	let res = solve(util::day_input::<1>());
	println!("Result: {}", res);
}

fn solve(input: &str) -> u32 {
	let mut data = parse_lists.parse(input).expect("needs to be parsable");
	data.0.sort();
	data.1.sort();

	data.0
		.into_iter()
		.zip(data.1)
		.map(|(a, b)| a.abs_diff(b))
		.sum()
}

pub fn parse_lists(input: &mut &str) -> PResult<(Vec<u32>, Vec<u32>)> {
	separated(
		1..,
		separated_pair(parse_num, space1, parse_num)
			.context(StrContext::Label("line with two numbers")),
		line_ending
	)
	.context(StrContext::Label("lines"))
	.parse_next(input)
	.map(|a: Vec<_>| a.into_iter().unzip())
}

fn parse_num(input: &mut &str) -> PResult<u32> {
	digit1
		.context(StrContext::Label("number parsing"))
		.parse_next(input)
		.map(|n| n.parse::<u32>().expect("number has a reasonable size"))
}

#[cfg(test)]
pub mod test {
	use winnow::Parser;

	use super::parse_lists;

	pub const SAMPLE: &str = {
		"3   4
4   3
2   5
1   3
3   9
3   3"
	};

	#[test]
	fn sample_parsing() {
		let v1 = vec![3, 4, 2, 1, 3, 3];
		let v2 = vec![4, 3, 5, 3, 9, 3];
		assert_eq!(parse_lists.parse(SAMPLE).expect("can parse"), (v1, v2));
	}

	#[test]
	fn solve_sample() {
		assert_eq!(crate::solve(SAMPLE), 11);
	}
}
