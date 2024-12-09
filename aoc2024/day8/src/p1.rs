mod common;

use common::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use winnow::error::{ContextError, ErrMode};

#[doc = include_str!("../p1.md")]
fn main() {
    util::DayInput::find::<8>().solve_with(solve);
}

/// # Problem
/// Given a map of antennas of different frequencies, how many antinodes are there?
/// Antinodes are defined/created by antennas of the same frequency,
/// and they occur on the line created by the two antennas,
/// with the distance to each side being equal to the distance between each antenna.
/// # Solution
/// Creating a HashMap of every antenna frequency and their every (antenna) location,
/// calculate every possible antinode location, filtering out any that would go outside the map.
/// I think the difficulty comes from parsing this into usable data.
fn solve(input: impl AsRef<str>) -> u64 {
    input
        .as_ref()
        .parse::<AntinodeMap>()
        .expect("parsable")
        .antinodes
        .len() as u64
}

fn non_unique_antinodes(map: &Map) -> impl Iterator<Item = Position> + '_ {
    map.antennas.iter().flat_map(|(_c, antenna_positions)| {
        antenna_positions
            .iter()
            // create a line between every antenna of same frequency
            .tuple_combinations()
            .flat_map(|(a, b)| {
                let offset = a.offset_from(b);
                [a.with_offset(&-offset), b.with_offset(&offset)]
            })
            // make sure these are in the right positions
            .filter(|pos| {
                (0..map.width as i64).contains(&pos.x) && (0..map.height as i64).contains(&pos.y)
            })
    })
}
struct AntinodeMap {
    inner: Map,
    antinodes: HashSet<Position>,
}
impl Display for AntinodeMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

    use super::common::tests::{
        BASIC_P1_ANTINODES, COMPLEX, COMPLEX_P1_ANTINODES, INTERMEDIATE_P1_ANTINODES_1,
        INTERMEDIATE_P1_ANTINODES_2,
    };

    const P1_ANTINODES: [&str; 4] = [
        BASIC_P1_ANTINODES,
        INTERMEDIATE_P1_ANTINODES_1,
        INTERMEDIATE_P1_ANTINODES_2,
        COMPLEX_P1_ANTINODES,
    ];

    #[test]
    fn calculates_antinodes() {
        for (sample, antinode_count) in [
            (COMPLEX, 14),
            (BASIC_P1_ANTINODES, 2),
            (INTERMEDIATE_P1_ANTINODES_1, 4),
            (INTERMEDIATE_P1_ANTINODES_2, 4),
            (COMPLEX_P1_ANTINODES, 14),
        ] {
            let antinodes = sample.parse::<super::AntinodeMap>().unwrap().antinodes;
            assert_eq!(
                antinodes.len(),
                antinode_count,
                "There should be be the known amount of antinodes."
            );
        }
    }

    #[test]
    fn antinodes_can_be_hidden() {
        for sample_with_hidden_antinode in
            [COMPLEX, INTERMEDIATE_P1_ANTINODES_2, COMPLEX_P1_ANTINODES]
        {
            let map = sample_with_hidden_antinode
                .parse::<super::AntinodeMap>()
                .unwrap();
            let hash_chars = map.to_string().chars().filter(|&c| c == '#').count();
            assert_ne!(
                hash_chars,
                map.antinodes.len(),
                "Antennas can appear on top of antibody locations.\n\
            Sample where this is the case:\n{}\n\
            What is rendered:\n{}",
                sample_with_hidden_antinode,
                map
            );
        }
    }

    #[test]
    fn displays_with_antinodes() {
        super::common::tests::displays_with_antinodes::<super::AntinodeMap>(&P1_ANTINODES);
    }

    #[test]
    fn input_solvable() {
        const INPUT: &str = include_str!("../../inputs/8");
        eprintln!("Map:\n{}", INPUT.parse::<super::AntinodeMap>().unwrap());
        assert_eq!(super::solve(INPUT), 392);
    }
}
