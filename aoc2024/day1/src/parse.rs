use winnow::{PResult, Parser};
use winnow::ascii::{digit1, line_ending, space1};
use winnow::error::StrContext;
use winnow::combinator::{separated, separated_pair};

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