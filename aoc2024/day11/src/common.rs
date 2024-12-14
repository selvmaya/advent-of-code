use winnow::{ascii::dec_uint, combinator::separated, PResult, Parser};

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Stone(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stones(pub Vec<Stone>);

pub fn parse_stones(input: &mut &str) -> PResult<Stones> {
	separated(1.., parse_stone, " ")
		.parse_next(input)
		.map(Stones)
}

fn parse_stone(input: &mut &str) -> PResult<Stone> {
	dec_uint.map(Stone).parse_next(input)
}
