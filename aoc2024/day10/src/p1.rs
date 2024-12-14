#![doc = include_str!("../p1.md")]

use itertools::Itertools;
#[allow(unused_imports)]
use winnow::{
	ascii::*,
	combinator::*,
	error::*,
	prelude::*,
	stream::*,
	token::*,
	{PResult, Parser}
};

mod common;
use common::*;

fn main() {
	util::DayInput::find::<10>().solve_with(solve);
}

/// # Definitions
/// Hiking trails increment by exactly 1 every step. Thy start from 0 and go to
/// 9. Movements (steps) can only happen in cardinal directions (on the map)
/// Trailheads start one or more hiking trails, always at height 0.
/// A trailhead score is the amount of 9-height positions that can be reached.
/// *Note*: Not paths to take, but positions of 9-height it can reach.
/// # Input
/// The input of topographic maps is a rectangle of digits 0..=9, and there are
/// no other symbols (excluding newlines on the right / EOF), apart from the
/// samples which have a `.` as an impassable tile.
/// # Problem
/// What is the sum of all the trailhead scores?
/// # Solution
/// Since you can only go in one direction (up towards 9) there is a hard limit
/// on amount of steps any search will take, though there may be many. This is
/// potentially a restriction lifted in part 2?
/// An approach is to identify the position of every trail start and trail end
/// (such that they are distinct), then iterate from every start trail and go
/// every possible cardinal direction each step. The input isn't enormous, so
/// this doesn't feel too infeasible.
fn solve(input: impl AsRef<str>) -> u64 {
	let map = common::parse_map
		.parse_next(&mut input.as_ref())
		.expect("parsable");
	debug_assert_eq!(
		input.as_ref().trim().lines().count() * map.width as usize,
		map.inner.len()
	);
	trailhead_score_sum(&map)
}

fn trailhead_score_sum(map: &TopographicMap) -> u64 {
	map.trailheads()
		// all trailhead positions
		.map(|pos| {
			/// Finds the positions of every 9-height (top) position reachable,
			/// that is reachable from 0-height (the trailhead itself).
			/// The returned values are not ensured unique in of themselves.
			fn tops_from_trailhead(
				map: &TopographicMap,
				pos: usize,
				height: u8
			) -> Box<dyn Iterator<Item = usize> + '_> {
				if height == 9 {
					Box::new(std::iter::once(pos))
				} else {
					let one_higher = height + 1;
					let map_width = map.width as usize;
					Box::new(
						map.all_dir_iter(pos, one_higher, map_width)
							.flat_map(move |valid_pos| {
								tops_from_trailhead(map, valid_pos, one_higher)
							})
					)
				}
			}
			// find score of this trailhead
			tops_from_trailhead(map, pos, 0).unique().count() as u64
			// these are summed: trailheads may share tops
		})
		.sum::<u64>()
}

#[cfg(test)]
mod tests {
	#[test]
	fn trailhead_score_sum_examples() {
		assert_eq!(super::solve(include_str!("EXAMPLE_SMALL")), 1);
		assert_eq!(super::solve(include_str!("TRAILHEAD_2")), 2);
		assert_eq!(super::solve(include_str!("TRAILHEAD_1_2")), 3);
		assert_eq!(super::solve(include_str!("TRAILHEAD_4")), 4);
		assert_eq!(super::solve(include_str!("EXAMPLE_LARGE")), 36);
		assert_eq!(super::solve(include_str!("TRAILHEAD_6")), 6);
	}

	#[ignore]
	#[test]
	fn input_solvable() {
		assert_eq!(super::solve(include_str!("../../inputs/10")), 607);
	}
}
