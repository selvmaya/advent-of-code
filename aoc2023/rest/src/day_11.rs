use glue::SolverFn;
use itertools::Itertools;
use std::cmp::{max, min};

pub const PARTS: &[SolverFn] = &[part_1, part_2];

/// TAKES: an image of empty space (.) and galaxies (#)
fn part_1(input: &str) -> String {
	solve(input, 2)
}

/// TAKES: an image of empty space (.) and galaxies (#)
fn part_2(input: &str) -> String {
	solve(input, 1000000)
}

fn solve(input: &str, expansion_distance: usize) -> String {
	#[derive(PartialEq)]
	enum SpaceTile {
		Empty,
		BigEmpty,
		Galaxy,
		LabeledGalaxy(usize),
	}

	// create base space map
	let mut space = input
		.lines()
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'.' => SpaceTile::Empty,
					'#' => SpaceTile::Galaxy,
					_ => unreachable!("input does not contain any other characters"),
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	// expand
	for vertical_index in 0..space.len() {
		if space[vertical_index]
			.iter()
			.all(|tile| *tile != SpaceTile::Galaxy)
		{
			for mut tile in space[vertical_index].iter_mut() {
				*tile = SpaceTile::BigEmpty;
			}
		}
	}
	for horizontal_index in 0..space[0].len() {
		if space
			.iter()
			.all(|tiles| tiles[horizontal_index] != SpaceTile::Galaxy)
		{
			for mut tiles in space.iter_mut() {
				tiles[horizontal_index] = SpaceTile::BigEmpty;
			}
		}
	}
	// label galaxies
	for (index, galaxy_tile) in space
		.iter_mut()
		.flat_map(|tiles| tiles.iter_mut())
		.filter(|tile| **tile == SpaceTile::Galaxy)
		.enumerate()
	{
		*galaxy_tile = SpaceTile::LabeledGalaxy(index);
	}
	space
		.iter()
		.enumerate()
		.flat_map(|(y, tiles)| {
			tiles
				.iter()
				.enumerate()
				.map(move |(x, tile)| ((x, y), tile))
				.filter(|(_pos, tile)| matches!(tile, SpaceTile::LabeledGalaxy(_)))
		})
		.tuple_combinations::<(_, _)>()
		.dedup_by(|((_, a_a), (_, a_b)), ((_, b_a), (_, b_b))| a_a == b_b && a_b == b_a)
		.map(|(((a_x, a_y), _), ((b_x, b_y), _))| {
			let mut distance: usize = 0;
			let min_x = min(a_x, b_x);
			let max_x = max(a_x, b_x);
			let min_y = min(a_y, b_y);
			let max_y = max(a_y, b_y);
			for x in min_x..max_x {
				distance += match space[min_y][x] {
					SpaceTile::BigEmpty => expansion_distance,
					_ => 1,
				};
			}
			for y in (1 + min_y)..=max_y {
				distance += match space[y][max_x] {
					SpaceTile::BigEmpty => expansion_distance,
					_ => 1,
				};
			}
			distance
		})
		.sum::<usize>()
		.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

	#[test]
	fn part_1_works() {
		assert_eq!(part_1(TEST_INPUT), "374");
	}

	#[test]
	fn part_2_works() {
		assert_eq!(solve(TEST_INPUT, 10), "1030");
		assert_eq!(solve(TEST_INPUT, 100), "8410");
	}
}