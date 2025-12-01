use std::{fmt::Display, str::FromStr};

use winnow::{ascii::dec_uint, error::ContextError, prelude::*, token::one_of};

#[cfg(test)]
pub const EXAMPLE_STR: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
pub const REAL_INPUT: &str = include_str!("../../inputs/d01");

/// A puzzle representation that can own its data or simply work through an iterator.
pub struct Puzzle<R, I: IntoIterator<Item = R> = Vec<R>>(pub I);

/// Owned puzzle representation (vec)
impl FromStr for Puzzle<Rotation, Vec<Rotation>> {
    type Err = ContextError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .lines()
            .map(|line| line.trim().parse::<Rotation>())
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

pub type RotationParseResult = Result<Rotation, ContextError>;

/// Thinly wrap iterator from input as puzzle representation
impl<'a> From<&'a str>
    for Puzzle<
        RotationParseResult,
        std::iter::Map<std::str::Lines<'a>, fn(&'a str) -> RotationParseResult>,
    >
{
    fn from(s: &'a str) -> Self {
        Self(s.lines().map(Rotation::from_str))
    }
}

/// Representing the output
#[derive(Debug, PartialEq, Eq)]
pub struct Password(pub usize);

/// Value range of `0..=99`.
#[derive(Copy, Clone)]
pub struct DialPoint(pub u8);
impl Display for DialPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.0.to_string())
    }
}
/// Starts at 50.
impl Default for DialPoint {
    fn default() -> Self {
        Self(50)
    }
}

#[derive(Copy, Clone)]
pub struct Rotation {
    /// Left or Right
    pub dir: Direction,
    /// Any positive number (range not specified)
    pub distance: usize,
}
fn parse_rotation(input: &mut &str) -> winnow::Result<Rotation> {
    (parse_direction, dec_uint)
        .map(|(dir, distance)| Rotation { dir, distance })
        .parse_next(input)
}
impl FromStr for Rotation {
    type Err = ContextError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        parse_rotation(&mut s)
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}
fn parse_direction(input: &mut &str) -> winnow::Result<Direction> {
    one_of(['L', 'R'])
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .parse_next(input)
}

pub mod display {
    use std::fmt::Display;

    use super::{Direction, Rotation};

    impl Display for Rotation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if f.alternate() {
                write!(f, "{:#}", self.dir)?;
                f.pad(&format!("{}", self.distance))
            } else {
                write!(f, "{}", self.dir)?;
                f.pad(&format!("{}", self.distance))
            }
        }
    }

    impl Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if f.alternate() {
                f.pad(match self {
                    Direction::Left => "-",
                    Direction::Right => "+",
                })
            } else {
                f.pad(match self {
                    Direction::Left => "L",
                    Direction::Right => "R",
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn real_input_assumptions() {
        assert!(
            !REAL_INPUT
                .lines()
                .any(|line| Rotation::from_str(line).expect("parse entry").distance > 999),
            "no number is more than 999 (assumption)"
        );
    }
}
