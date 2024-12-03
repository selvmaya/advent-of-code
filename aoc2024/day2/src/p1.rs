mod common;

use itertools::Itertools;
#[allow(unused_imports)]
use winnow::{
	ascii::*,
	combinator::*,
	error::*,
	{PResult, Parser}
};

#[doc = include_str!("p1.md")]
fn main() {
	let res = solve(util::day_input::<2>());
	println!("Result: {}", res);
}

/// Each line is a report,
/// and each report is a list of levels (numbers, separated by spaces).
fn solve(input: impl AsRef<str>) -> u32 {
	input
		.as_ref()
		.lines()
		.filter(|line| {
			let level = line
				.split(' ')
				.map(|a| a.parse::<u32>())
				.collect::<Result<Vec<_>, _>>()
				.expect("all numbers should be parsable");
			level
				.iter()
				.tuple_windows()
				.all(|(&a, &b)| (1..=3).contains(&(a as i32 - b as i32)))
				|| level
					.iter()
					.tuple_windows()
					.all(|(&a, &b)| (1..=3).contains(&(b as i32 - a as i32)))
		})
		.count() as u32
}

#[cfg(test)]
mod p1test {
	#[test]
	fn sample_solvable() {
		assert_eq!(super::solve(super::common::SAMPLE), 2);
	}
}
