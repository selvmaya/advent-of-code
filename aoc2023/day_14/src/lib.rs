use anyhow::anyhow;
use glue::SolverFn;
use itertools::Itertools;

pub const PARTS: &[SolverFn] = &[part_1];

#[derive(Debug, Clone, Eq, PartialEq)]
enum Element {
	RoundRock,  // moves, contributes weight
	SquareRock, // only blocks
	Empty,      // just empty space
}
impl TryFrom<char> for Element {
	type Error = anyhow::Error;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		Ok(match c {
			'O' => Element::RoundRock,
			'#' => Element::SquareRock,
			'.' => Element::Empty,
			_ => Err(anyhow!("char '{}' not recognized as grid element", c))?,
		})
	}
}
impl From<&Element> for char {
	fn from(element: &Element) -> Self {
		match element {
			Element::RoundRock => 'O',
			Element::SquareRock => '#',
			Element::Empty => '.',
		}
	}
}

fn part_1(input: &str) -> String {
	let mut rows = input
		.lines()
		.map(|line| {
			line.chars()
				.map(|c| c.try_into())
				.collect::<Result<Vec<Element>, _>>()
				.expect("cannot parse line as row of elements")
		})
		.collect::<Vec<Vec<Element>>>();
	// assume same length rows
	for index in 0..(rows.len() - 1) {
		let next_index = index + 1;
		for column in 0..rows[next_index].len() {
			if rows[next_index][column] == Element::RoundRock
				&& rows[index][column] == Element::Empty
			{
				for (above, below) in (0..=next_index).into_iter().rev().tuple_windows() {
					if (rows[below][column]) != Element::Empty {
						break;
					}
					rows[below][column] = Element::RoundRock;
					rows[above][column] = Element::Empty;
				}
			}
		}
	}
	rows.iter()
		.rev()
		.enumerate()
		.map(|(index, row)| {
			(row.iter()
				.filter(|element| **element == Element::RoundRock)
				.count() * (1 + index)) as u64
		})
		.sum::<u64>()
		.to_string()
}

#[cfg(test)]
fn display_grid(grid: &Vec<Vec<Element>>) -> String {
	grid.iter()
		.map(|row| row.iter().map(|element| char::from(element)).join(""))
		.join("\n")
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &'static str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

	#[test]
	fn part_1_works() {
		assert_eq!(part_1(TEST_INPUT), "136")
	}
}