#![doc = include_str!("../p2.md")]

mod common;

use std::collections::{BTreeMap, HashMap};

use common::*;
#[allow(unused_imports)]
use winnow::{
	ascii::*,
	combinator::*,
	error::*,
	token::*,
	{PResult, Parser}
};

fn main() {
	util::DayInput::find::<11>().solve_with(solve);
}

fn solve(input: impl AsRef<str>) -> u64 {
	let initial = parse_stones(&mut input.as_ref()).expect("parsable");
	len_after_blinks::<75>(initial)
}

fn len_after_blinks<const N: u8>(stones: Stones) -> u64 {
	fn count_after_steps(stone: Stone, blinks: u8, visited: &mut BTreeMap<Stone, u64>) -> u64 {
		if blinks == 0 {
			1 // there is one step at sub-step
		} else if let Some(&prev_state) = visited.get(&stone) {
			prev_state
		} else {
			let sub_blinks = blinks - 1;
			match stone.0 {
				// num is zero
				0 => count_after_steps(Stone(1), sub_blinks, visited),
				// if amount of digits in number is even
				n if (n.ilog10() + 1) % 2 == 0 => {
					let digit_len = n.ilog10() + 1;
					let half_digit_len = digit_len / 2;
					let cut_off: u64 = 10_u64.pow(half_digit_len);
					let left_digits = n / cut_off;
					let right_digits = n % cut_off;
					count_after_steps(Stone(left_digits), sub_blinks, visited)
						+ count_after_steps(Stone(right_digits), sub_blinks, visited)
				},
				// all other numbers
				n => count_after_steps(Stone(n * 2024), sub_blinks, visited)
			}
		}
	}

	let mut visited: BTreeMap<Stone, u64> = BTreeMap::new();
	stones
		.0
		.into_iter()
		.map(|stone| count_after_steps(stone, N, &mut visited))
		.sum()
}

#[cfg(test)]
mod tests {
	use crate::common::{Stone, Stones};

	#[test]
	fn samples() {
		assert_eq!(
			super::len_after_blinks::<25>(Stones(vec![Stone(125), Stone(17)])),
			55312
		)
	}

	#[test]
	fn input_solvable() {
		assert_eq!(super::solve(include_str!("../../inputs/11")), 0);
	}
}
