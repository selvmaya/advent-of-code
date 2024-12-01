mod p1;

use itertools::Itertools;
#[allow(unused_imports)]
use winnow::{ascii::*, combinator::*, error::*, prelude::*, token::*, Parser};

#[doc = include_str!("../p2.md")]
fn main() {
	let similarity_score = solve(util::day_input::<2>());
	println!("Similarity score: {}", similarity_score);
}

fn solve(input: &str) -> u32 {
	let lists = p1::parse_lists.parse(input).expect("input still parsable");

	// find how many times each number in list 0 appears in list 1, then multiply
	// the number with its amount of appearances (in list 1), and sum these
	// together.

	let left = lists.0.into_iter().collect::<Vec<_>>();
	let counts = lists.1.into_iter().filter(|b| left.contains(b)).counts();
	left.into_iter()
		.map(|a| a * *counts.get(&a).unwrap_or(&0) as u32)
		.sum()
}

#[cfg(test)]
mod test {
	#[test]
	fn solve_sample() {
		// sample is reused from p1
		assert_eq!(crate::solve(crate::p1::test::SAMPLE), 31);
	}
}
