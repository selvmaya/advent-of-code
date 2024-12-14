#![doc = include_str!("../p2.md")]

mod common;

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
	util::DayInput::find::<10>().solve_with(solve);
}

/// # Problem
/// What is the sum of the trailhead ratings?
/// # Definitions
/// A trailhead rating is how many distinct routes from 0 to 9 through the map
/// for this trailhead. Different trailheads all have distinct routes, since
/// they start at distinct 0-height places. # Solution
/// Instead of tallying up how many valid leaf nodes (9) can be reached from
/// each trailhead, join together distinct route and sum up how many there are
/// across all trailheads. Actually, is it even necessary to have them be
/// distinct? Isn't every route inherently distinct? ## Note
/// We want to start collecting data on each journey as we go back up the tree,
/// *not* as we go down. It remains true that branch depth is still capped to 9
/// or 10.
fn solve(input: impl AsRef<str>) -> u64 {
	let map = parse_map.parse_next(&mut input.as_ref()).expect("parsable");
	sum_of_distinct_hiking_trails(&map)
}

fn sum_of_distinct_hiking_trails(map: &TopographicMap) -> u64 {
	map.trailheads()
		.map(|pos| {
			fn trails_from_height(map: &TopographicMap, pos: usize, height: u8) -> u64 {
				if height == 9 {
					1
				} else {
					let one_higher = height + 1;
					let map_width = map.width as usize;
					map.all_dir_iter(pos, one_higher, map_width)
						.map(move |valid_pos| trails_from_height(map, valid_pos, one_higher))
						.sum()
				}
			}

			trails_from_height(map, pos, 0)
		})
		.sum()
}

#[cfg(test)]
mod tests {
	#[test]
	fn trailhead_rating_sum_examples() {
		assert_eq!(super::solve(include_str!("HIKING_TRAIL_3")), 3);
		assert_eq!(super::solve(include_str!("HIKING_TRAIL_13")), 13);
		assert_eq!(super::solve(include_str!("HIKING_TRAIL_227")), 227);
		assert_eq!(super::solve(include_str!("EXAMPLE_LARGE")), 81);
	}

	#[test]
	fn input_solvable() {
		assert_eq!(super::solve(include_str!("../../inputs/10")), 1384);
	}
}
