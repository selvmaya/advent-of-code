use std::{
    fmt::Display,
    iter::Map,
    str::{FromStr, Lines},
};

use winnow::{ascii::dec_uint, error::ContextError, token::one_of, Parser};

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    impl std::ops::AddAssign<Rotation> for DialPoint {
        fn add_assign(&mut self, rhs: Rotation) {
            *self = *self + rhs
        }
    }

    #[test]
    fn dial_point_additions_work() {
        let mut dial_point = DialPoint::default();
        assert_eq!(dial_point.0, 50);

        dial_point += Rotation {
            dir: Direction::Right,
            distance: 49,
        };
        assert_eq!(dial_point.0, 99, "able to add before wrap");

        dial_point += Rotation {
            dir: Direction::Right,
            distance: 1,
        };
        assert_eq!(dial_point.0, 0, "wraps to zero");

        dial_point += Rotation {
            dir: Direction::Left,
            distance: 100,
        };
        assert_eq!(dial_point.0, 0, "goes backwards and wraps, still hits zero");

        dial_point += Rotation {
            dir: Direction::Left,
            distance: 50,
        };
        assert_eq!(dial_point.0, 50, "goes backwards and wraps, goes halfway");
    }

    #[test]
    fn dial_point_addition_examples() {
        // as taken from problem text
        {
            let mut dial_point = DialPoint(11);
            dial_point += Rotation {
                dir: Direction::Right,
                distance: 8,
            };
            assert_eq!(dial_point.0, 19);

            dial_point += Rotation {
                dir: Direction::Left,
                distance: 19,
            };
            assert_eq!(dial_point.0, 0);
        }
        {
            let mut dial_point = DialPoint(0);
            dial_point += Rotation {
                dir: Direction::Left,
                distance: 1,
            };
            assert_eq!(dial_point.0, 99);

            dial_point += Rotation {
                dir: Direction::Right,
                distance: 1,
            };
            assert_eq!(dial_point.0, 0);
        }
        {
            let mut dial_point = DialPoint(5);
            dial_point += Rotation {
                dir: Direction::Left,
                distance: 10,
            };
            assert_eq!(dial_point.0, 95);

            dial_point += Rotation {
                dir: Direction::Right,
                distance: 5,
            };
            assert_eq!(dial_point.0, 0);
        }
    }

    const EXAMPLE_STR: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    const EXAMPLE_PASSWORD: Password = Password(3);

    #[test]
    fn parsing_works() {
        assert_eq!(
            EXAMPLE_STR.lines().count(),
            10,
            "example str is laid out properly"
        );
        EXAMPLE_STR
            .trim()
            .lines()
            .map(Rotation::from_str)
            .collect::<Result<Vec<Rotation>, _>>()
            .expect("able to parse");
    }

    #[test]
    fn intermediate_fold_works() {
        let parsed_representation = EXAMPLE_STR
            .parse::<Puzzle<Rotation>>()
            .expect("able to parse input first");
        eprintln!(
            "Parsed representation:\n{}\n",
            parsed_representation.0.iter().join("\n")
        );
        let computed_password = Password::from(parsed_representation);
        assert_eq!(computed_password, EXAMPLE_PASSWORD);
    }

    #[test]
    fn direct_fold_works() {
        let computed_password =
            Password::try_from(Puzzle::from(EXAMPLE_STR)).expect("able to parse while calculating");
        assert_eq!(computed_password, EXAMPLE_PASSWORD)
    }

    #[ignore]
    #[test]
    fn real_input_assumptions() {
        assert!(
            !REAL_INPUT
                .lines()
                .any(|line| Rotation::from_str(line).expect("parse entry").distance > 999),
            "no number is more than 999 (assumption)"
        );
    }

    #[ignore]
    #[test]
    fn real_input_works() {
        let computed_password =
            Password::try_from(Puzzle::from(REAL_INPUT)).expect("able to parse while calculating");
        let alternative_computed_password = Password::from(Puzzle::from_str(REAL_INPUT).expect(""));
        assert_eq!(
            computed_password, alternative_computed_password,
            "both should be the same number"
        );

        assert_eq!(computed_password.0, 1182);
    }
}

const REAL_INPUT: &str = include_str!("../../inputs/d01");

fn main() {
    println!("Starting dial point at {}.", DialPoint::default());

    let answer = Password::try_from(Puzzle::from(REAL_INPUT))
        .expect("able to parse")
        .0;

    println!("Password to open door (times reached zero): {}", answer);
}

/// A puzzle representation that can own its data or simply work through an iterator.
struct Puzzle<R, I: IntoIterator<Item = R> = Vec<R>>(I);

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

type RotationParseResult = Result<Rotation, ContextError>;

/// Thinly wrap iterator from input as puzzle representation
impl<'a> From<&'a str>
    for Puzzle<RotationParseResult, Map<Lines<'a>, fn(&'a str) -> RotationParseResult>>
{
    fn from(s: &'a str) -> Self {
        Self(s.lines().map(Rotation::from_str))
    }
}

/// Representing the output
#[derive(Debug, PartialEq, Eq)]
struct Password(usize);

/// solve for `Vec` of `Rotation`s
impl From<Puzzle<Rotation>> for Password {
    fn from(puzzle: Puzzle<Rotation>) -> Self {
        Password(
            puzzle
                .0
                .into_iter()
                .fold(
                    (0, DialPoint::default()),
                    |(times_reached_zero, dial_point), rotation| {
                        handle_rotation(times_reached_zero, dial_point, rotation)
                    },
                )
                .0,
        )
    }
}

/// Add rotation to dial point, incrementing `times_reached_zero` along the way if `dial_point.0` is zero.
fn handle_rotation(
    times_reached_zero: usize,
    dial_point: DialPoint,
    rotation: Rotation,
) -> (usize, DialPoint) {
    let new_dial_point = dial_point + rotation;
    (
        if new_dial_point.0 == 0 {
            // eprintln!("{dial_point:>2} {rotation:>#3} = {new_dial_point:>2} (times_reached_zero is now '{}')", times_reached_zero + 1);
            times_reached_zero + 1
        } else {
            // eprintln!("{dial_point:>2} {rotation:>#3} = {new_dial_point:>2}");
            times_reached_zero
        },
        new_dial_point,
    )
}

/// solve for iterator `I` of maybe-parsed `Rotation`s,
/// avoiding collecting to vec in parse step, since we fold directly.
impl<I: Iterator<Item = RotationParseResult>> TryFrom<Puzzle<RotationParseResult, I>> for Password {
    type Error = ContextError;

    fn try_from(mut puzzle: Puzzle<RotationParseResult, I>) -> Result<Self, Self::Error> {
        puzzle
            .0
            .try_fold::<(usize, DialPoint), _, Result<(usize, DialPoint), ContextError>>(
                (0, DialPoint::default()),
                |(times_reached_zero, dial_point), rotation| {
                    rotation
                        .map(|rotation| handle_rotation(times_reached_zero, dial_point, rotation))
                },
            )
            .map(|(password, _)| Password(password))
    }
}

/// Value range of `0..=99`.
#[derive(Copy, Clone)]
struct DialPoint(u8);
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
impl std::ops::Add<Rotation> for DialPoint {
    type Output = DialPoint;

    fn add(self, rhs: Rotation) -> Self::Output {
        // 210 is the same as 110, same as 10. We do not care how many times we may pass zero.
        let space_distance: u8 = (rhs.distance.rem_euclid(100)) as u8;
        DialPoint(match rhs.dir {
            Direction::Left => {
                if space_distance > self.0 {
                    // manual wrapping, starting from 100
                    100u8 - (space_distance - self.0)
                } else {
                    self.0 - space_distance
                }
            }
            // 99 + 99 < 255, we do not need to worry about running out of space
            Direction::Right => (self.0 + space_distance).rem_euclid(100),
        })
    }
}

#[derive(Copy, Clone)]
struct Rotation {
    /// Left or Right
    dir: Direction,
    /// Any positive number (range not specified)
    distance: usize,
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
enum Direction {
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

mod display {
    use std::fmt::Display;

    use crate::{Direction, Rotation};

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
