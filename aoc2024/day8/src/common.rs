use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    ops::Neg,
    str::FromStr,
};

use itertools::Itertools;
use winnow::stream::AsChar;
#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
impl Position {
    pub fn offset_from(&self, other: &Self) -> Offset {
        Offset {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    pub fn with_offset(&self, offset: &Offset) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Offset {
    pub x: i64,
    pub y: i64,
}
impl Neg for Offset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub antennas: HashMap<char, Vec<Position>>,
}
// basic rendering
impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position {
                    x: x as i64,
                    y: y as i64,
                };
                write!(
                    f,
                    "{}",
                    match self
                        .antennas
                        .iter()
                        .find(|(_, antennas)| antennas.iter().any(|&antenna| antenna == pos))
                    {
                        Some((&c, _)) => c,
                        None => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl FromStr for Map {
    type Err = ErrMode<ContextError>;

    fn from_str(mut input: &str) -> Result<Self, Self::Err> {
        fn parse_antenna_positions(input: &mut &str) -> PResult<HashMap<char, Vec<Position>>> {
            separated(
                1..,
                take_while(1.., ('.', '#', AsChar::is_alphanum)),
                line_ending,
            )
            .parse_next(input)
            .map(|all: Vec<&str>| {
                all.into_iter()
                    .enumerate()
                    .flat_map(|(y, line)| {
                        line.chars()
                            .enumerate()
                            .filter(|(_, c)| c.is_alphanum())
                            .map(move |(x, c)| {
                                (
                                    c,
                                    Position {
                                        x: x as i64,
                                        y: y as i64,
                                    },
                                )
                            })
                    })
                    .into_group_map()
            })
        }

        let width = input.lines().next().expect("first line").chars().count();
        let height = input.lines().count();
        parse_antenna_positions
            .parse_next(&mut input)
            .map(|data| Self {
                width,
                height,
                antennas: data,
            })
    }
}

// advanced rendering (also shows antinodes)
impl Map {
    pub fn render_with_antinodes(
        &self,
        antinodes: &HashSet<Position>,
        f: &mut Formatter,
    ) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position {
                    x: x as i64,
                    y: y as i64,
                };
                write!(
                    f,
                    "{}",
                    match self
                        .antennas
                        .iter()
                        .find(|(_, antennas)| antennas.iter().any(|&antenna| antenna == pos))
                    {
                        Some((&c, _)) => c,
                        None =>
                            if antinodes.iter().any(|&antinode| antinode == pos) {
                                '#'
                            } else {
                                '.'
                            },
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use std::{
        collections::HashSet,
        fmt::{Debug, Display},
        str::FromStr,
    };

    use itertools::Itertools;

    // note: all `ANTINODE` assume an antinode implementation, which is specific to
    // each part in order of appearance
    // in p1.md:
    pub const COMPLEX: &str = include_str!("COMPLEX");
    pub const BASIC_P1_ANTINODES: &str = include_str!("BASIC_P1_ANTINODES");
    pub const INTERMEDIATE_P1_ANTINODES_1: &str = include_str!("INTERMEDIATE_P1_ANTINODES_1");
    pub const INTERMEDIATE_P1_ANTINODES_2: &str = include_str!("INTERMEDIATE_P1_ANTINODES_2");
    pub const COMPLEX_P1_ANTINODES: &str = include_str!("COMPLEX_P1_ANTINODES");
    // in p2.md:
    pub const T_P1_ANTINODES: &str = include_str!("T_P2_ANTINODES");
    pub const COMPLEX_P2_ANTINODES: &str = include_str!("COMPLEX_P2_ANTINODES");

    const ALL_SAMPLES: [&str; 7] = [
        COMPLEX,
        BASIC_P1_ANTINODES,
        INTERMEDIATE_P1_ANTINODES_1,
        INTERMEDIATE_P1_ANTINODES_2,
        COMPLEX_P1_ANTINODES,
        T_P1_ANTINODES,
        COMPLEX_P2_ANTINODES,
    ];

    #[test]
    fn parses_antenna_types() {
        for (sample, antenna_type_count) in [
            (COMPLEX, 2),
            (BASIC_P1_ANTINODES, 1),
            (INTERMEDIATE_P1_ANTINODES_1, 1),
            (INTERMEDIATE_P1_ANTINODES_2, 2),
            (COMPLEX_P1_ANTINODES, 2),
            (T_P1_ANTINODES, 1),
            (COMPLEX_P2_ANTINODES, 2),
        ] {
            let map = sample.parse::<super::Map>().unwrap();
            assert_eq!(
                map.antennas.keys().count(),
                antenna_type_count,
                "\nThe amount of antenna types have to be right.\nSource \
				 sample:\n{}\nGenerated:\n{}",
                sample,
                map
            );
        }
    }

    #[test]
    fn parses_antenna_total() {
        for (sample, antenna_count_total) in [
            (COMPLEX, 7),
            (BASIC_P1_ANTINODES, 2),
            (INTERMEDIATE_P1_ANTINODES_1, 3),
            (INTERMEDIATE_P1_ANTINODES_2, 4),
            (COMPLEX_P1_ANTINODES, 7),
            (T_P1_ANTINODES, 3),
            (COMPLEX_P2_ANTINODES, 7),
        ] {
            let map = sample.parse::<super::Map>().unwrap();
            let antennas = map.antennas.values().flatten().collect::<Vec<_>>();
            let antenna_set = antennas.iter().collect::<HashSet<_>>();
            assert_eq!(
                antennas.len(),
                antenna_set.len(),
                "\nThere should be no duplicate positions."
            );
            assert_eq!(
                antenna_set.len(),
                antenna_count_total,
                "\nThe total amount of antennas have to be right.\nSource \
				 sample:\n{}\nGenerated:\n{}",
                sample,
                map
            );
        }
    }

    #[test]
    fn antennas_are_unique() {
        for sample in ALL_SAMPLES {
            let map = sample.parse::<super::Map>().unwrap();
            assert!(
                map.antennas.values().all_unique(),
                "\nAntennas cannot be overlapping.\nSample:\n{}\nGenerated:\n{}",
                sample,
                map
            );
        }
    }

    #[test]
    fn positions_are_inside_bounds() {
        for sample in ALL_SAMPLES {
            let map = sample.parse::<super::Map>().unwrap();
            for pos in map.antennas.values().flatten() {
                assert!(
                    (0..map.width as i64).contains(&pos.x),
                    "Position must be inside bounds. x was {}, width is {}.",
                    pos.x,
                    map.width
                );
                assert!(
                    (0..map.height as i64).contains(&pos.y),
                    "Position must be inside bounds. y was {}, height is {}.",
                    pos.y,
                    map.height
                );
            }
        }
    }

    // noinspection RsAssertEqual
    pub fn displays_with_antinodes<T: FromStr<Err: Debug> + Display>(data: &[&str]) {
        for target in data {
            let map = target.parse::<T>().unwrap();
            let displayed = map.to_string();
            assert!(
                displayed.trim() == target.trim(),
                "\nThese should look the same.\nTARGET:\n{}\nACTUAL:\n{}",
                target,
                displayed
            );
        }
    }
}
