#![doc = include_str!("../p2.md")]

mod common;

use common::{Map, *};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

fn main() {
    util::DayInput::find::<8>().solve_with(solve);
}

/// # Problem
/// Same as before, but instead of calculating single spots on each side,
/// we generate lines that have antinodes for every discrete position.
/// ## Question: What is 0 and 0 are on the same line?
/// Is the line only contiguous if the antennas are next to each other?
/// # Solution
/// Iterate over every spot until we are out of bounds.
fn solve(input: impl AsRef<str>) -> u64 {
    input
        .as_ref()
        .parse::<AntinodeMap>()
        .expect("parsable")
        .antinodes
        .len() as u64
}

fn non_unique_antinodes(map: &Map) -> impl Iterator<Item = Position> + '_ {
    map.antennas.values().flat_map(|antenna_positions| {
        antenna_positions
            .iter()
            // a line for every two antennas of same the frequency
            .tuple_combinations()
            // generate points on the line using just the offset
            // (not counting any in-between values, even if they would "fit")
            .flat_map(|(a, b)| {
                fn position_iter<'a>(
                    origin: &'a Position,
                    base_offset: Offset,
                    continue_condition: impl Fn(&Position) -> bool + 'a,
                ) -> impl Iterator<Item = Position> + 'a {
                    // include antennas
                    (0..)
                        .map(move |mult| {
                            // go further away each step
                            origin.with_offset(&Offset {
                                x: base_offset.x * mult,
                                y: base_offset.y * mult,
                            })
                        })
                        // finish when would be outside bounds
                        .take_while(continue_condition)
                }

                let is_inside_bounds = |pos: &Position| {
                    (0..map.width as i64).contains(&pos.x)
                        && (0..map.height as i64).contains(&pos.y)
                };
                let distance = a.offset_from(b);
                let a = position_iter(a, -distance, is_inside_bounds);
                let b = position_iter(b, distance, is_inside_bounds);
                a.chain(b)
            })
    })
}

struct AntinodeMap {
    inner: Map,
    antinodes: HashSet<Position>,
}
impl Display for AntinodeMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // reuse rendering code
        self.inner.render_with_antinodes(&self.antinodes, f)
    }
}
//noinspection DuplicatedCode
impl FromStr for AntinodeMap {
    type Err = ErrMode<ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Map>().map(|map| Self {
            antinodes: non_unique_antinodes(&map).collect(),
            inner: map,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::common::tests::{COMPLEX_P2_ANTINODES, T_P1_ANTINODES};

    const P2_ANTINODES: [&str; 2] = [T_P1_ANTINODES, COMPLEX_P2_ANTINODES];

    #[test]
    fn displays_with_antinodes() {
        super::common::tests::displays_with_antinodes::<super::AntinodeMap>(&P2_ANTINODES);
    }

    #[test]
    fn calculates_antinodes() {
        for (sample, antinode_count) in [(T_P1_ANTINODES, 9), (COMPLEX_P2_ANTINODES, 34)] {
            let antinodes = sample.parse::<super::AntinodeMap>().unwrap().antinodes;
            assert_eq!(
                antinodes.len(),
                antinode_count,
                "There should be be the known amount of antinodes."
            );
        }
    }

    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/8")), 1235);
    }
}
