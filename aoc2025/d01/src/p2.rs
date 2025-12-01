mod shared;
use shared::*;
use winnow::error::ContextError;

/// Like previous, but every roll past 0 must count as hitting it,
/// therefor we need to do the inner math slightly differently.
fn main() {
    let password = Password::try_from(Puzzle::from(REAL_INPUT)).expect("able to parse");

    println!("Password is {}", password.0);
}

fn handle_rotation(
    times_passed_zero: usize,
    dial_point: DialPoint,
    rotation: Rotation,
) -> (usize, DialPoint) {
    let full_cycles = rotation.distance.div_euclid(100);
    let cycle_remainder = rotation.distance.rem_euclid(100) as u8;
    match rotation.dir {
        // we must handle underflow correctly
        Direction::Left => match dial_point.0.cmp(&cycle_remainder) {
            // we move below and past zero
            std::cmp::Ordering::Less if dial_point.0 != 0 => (
                times_passed_zero + full_cycles + 1,
                DialPoint(100 - (cycle_remainder - dial_point.0)),
            ),
            // we move below zero, but we were just there
            std::cmp::Ordering::Less => (
                times_passed_zero + full_cycles,
                DialPoint(100 - (cycle_remainder - dial_point.0)),
            ),
            // we land on zero
            std::cmp::Ordering::Equal => (times_passed_zero + full_cycles + 1, DialPoint(0)),
            // we stay some amount above zero
            std::cmp::Ordering::Greater => (
                times_passed_zero + full_cycles,
                DialPoint(dial_point.0 - cycle_remainder),
            ),
        },
        // no overflow: 99 + 99 < 255
        Direction::Right => match dial_point.0 + cycle_remainder {
            // we move beyond 100 (therefor past zero)
            100.. => (
                times_passed_zero + full_cycles + 1,
                DialPoint(dial_point.0 + cycle_remainder - 100),
            ),
            // we do not move beyond 100
            1..=99 => (
                times_passed_zero + full_cycles,
                DialPoint(dial_point.0 + cycle_remainder),
            ),
            // we do not move (assume overflow impossible)
            0 => panic!("unexpected edge case"), // (times_passed_zero + full_cycles + 1, dial_point),
        },
    }
}

/// Requires a vector of parsed rotations.
impl From<Puzzle<Rotation>> for Password {
    fn from(vec: Puzzle<Rotation>) -> Self {
        Password(
            vec.0
                .into_iter()
                .fold(
                    (0, DialPoint::default()),
                    |(times_passed_zero, dial_point), rotation| {
                        handle_rotation(times_passed_zero, dial_point, rotation)
                    },
                )
                .0,
        )
    }
}
/// Direct folding, without needed to allocate while parsing.
impl<I: Iterator<Item = RotationParseResult>> TryFrom<Puzzle<RotationParseResult, I>> for Password {
    type Error = ContextError;

    fn try_from(mut iter: Puzzle<RotationParseResult, I>) -> Result<Self, Self::Error> {
        iter.0
            .try_fold(
                (0, DialPoint::default()),
                |(times_passed_zero, dial_point), rotation| {
                    rotation
                        .map(|rotation| handle_rotation(times_passed_zero, dial_point, rotation))
                },
            )
            .map(|(times_passed_zero, _)| Password(times_passed_zero))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use itertools::Itertools;

    use super::*;

    const EXAMPLE_PASSWORD: Password = Password(6);

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

        assert_eq!(
            computed_password, alternative_computed_password,
            "both should be the same number"
        );

        assert_eq!(computed_password.0, 6907);
    }

    #[test]
    fn outside_sample_works() {
        const SAMPLE: &str = "R9\nL8\nL26\nR45\nR40\nL45\nR13\nL20\nL8\nR5\nL390\nR47\nR38\nL22\nL26\nL22\nL19\nL25\nL21\nL15\nR19\nR3\nR33\nR46\nL9\nR48\nL21\nR13\nR10\nL20\nR48\nL43\nR44\nR35\nR23\nR86\nL21\nL7\nR26\nL23\nL50\nL34\nR22\nL14\nR899\nR21\nL47\nR16\nL14\nL1";
        const SAMPLE_PASSWORD: Password = Password(29);

        let computed_password = Password::try_from(Puzzle::from(SAMPLE)).expect("able to parse");
        let alternate_computed_password =
            Password::from(Puzzle::from_str(SAMPLE).expect("able to parse")); // parsed to vector first before computing
        assert_eq!(
            computed_password, alternate_computed_password,
            "should be equal"
        );

        assert_eq!(computed_password, SAMPLE_PASSWORD, "should be this value?");
    }
}
