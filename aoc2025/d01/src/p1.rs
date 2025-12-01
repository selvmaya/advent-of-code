use shared::*;
use winnow::error::ContextError;

mod shared;

fn main() {
    println!("Starting dial point at {}.", DialPoint::default());

    let answer = Password::try_from(Puzzle::from(REAL_INPUT))
        .expect("able to parse")
        .0;

    println!("Password to open door (times reached zero): {}", answer);
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

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

    #[test]
    fn manual_parsing_works() {
        assert_eq!(
            EXAMPLE_STR.lines().count(),
            10,
            "example str is laid out properly"
        );
        shared::EXAMPLE_STR
            .trim()
            .lines()
            .map(Rotation::from_str)
            .collect::<Result<Vec<Rotation>, _>>()
            .expect("able to parse");
    }

    pub const EXAMPLE_PASSWORD: Password = Password(3);

    #[test]
    fn intermediate_fold_works() {
        let parsed_representation = shared::EXAMPLE_STR
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
        let computed_password = Password::try_from(Puzzle::from(shared::EXAMPLE_STR))
            .expect("able to parse while calculating");
        assert_eq!(computed_password, EXAMPLE_PASSWORD)
    }

    // #[ignore]
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
