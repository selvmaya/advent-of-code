#![feature(iter_map_windows)]
#![doc = include_str!("../p2.md")]

use itertools::Itertools;
#[allow(unused_imports)]
use winnow::{
	ascii::*,
	combinator::*,
	error::*,
	token::*,
	{PResult, Parser}
};

fn main() {
	util::DayInput::find::<4>().solve_with(solve);
}

/// Count amount of X's (diagonal lines; cardinal can be ignored) made of `MAS`,
/// where either direction is allowed for each line.
/// # Solution A:
/// For
fn solve(input: impl AsRef<str>) -> u32 {
	input
		.as_ref()
		.lines()
		// take three lines at a time
		// ...........
		// ...........
		// ...........
		.map_windows(|&three_lines: &[&str; 3]| three_lines)
		.flat_map(|three_lines| {
			// in these lines, generate all square iterators (3x3 going right)
			// |-|........
			// |x|........
			// |-|........
			three_lines
				.into_iter()
				.flat_map(|line| {
					// each line has three characters
					// |-|........
					line.chars()
						.map_windows(|three_chars: &[char; 3]| *three_chars)
						.enumerate()
				})
				// flatten then group according to char depth and not line height
				// [111|222|333],      [111|111|111],
				// [111|222|333],  ->  [222|222|222],
				// [111|222|333],      [333|333|333],
				.into_group_map()
				.into_values()
		})
		.map(|v| v[..].try_into().unwrap())
		// consider squares on all lines (three lines tall each)
		// |-|........
		// |x|........
		// |-|........
		// ...........
		// ...........
		.filter(|square: &[[char; 3]; 3]| {
			fn check_slice(slice: &[char; 3]) -> bool {
				slice == &['M', 'A', 'S'] || slice == &['S', 'A', 'M']
			}
			// check each diagonal ("up-right" + "down_right"), other data is irrelevant
			check_slice(&[square[0][0], square[1][1], square[2][2]])
				&& check_slice(&[square[0][2], square[1][1], square[2][0]])
		})
		.count() as u32
}

#[cfg(test)]
mod p2test {
	const SAMPLE: &str = include_str!("SAMPLE");
	const SAMPLE_FILTERED: &str = include_str!("SAMPLE_FILTERED2");

	#[test]
	fn sample_solves() {
		assert_eq!(super::solve(SAMPLE), 9);
	}

	#[test]
	fn sample_filtered_solves() {
		assert_eq!(super::solve(SAMPLE_FILTERED), 9);
	}

	#[test]
	fn sample_filtered_is_same() {
		assert_eq!(super::solve(SAMPLE), super::solve(SAMPLE_FILTERED));
	}

	//#[ignore]
	#[test]
	fn input_solvable() {
		assert_eq!(super::solve(include_str!("../../inputs/4")), 1875);
	}
}
