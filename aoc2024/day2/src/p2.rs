use itertools::Itertools;

mod common;

#[doc = include_str!("p2.md")]
fn main() {
    util::DayInput::find::<2>().solve_with(solve);
}

fn solve(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .expect("numbers are parsable")
        })
        .filter(|level| {
            // all variations where 1 number is removed from the level, if any of these
            // variations is valid then the whole level is
            (0..level.len()).any(|skipped_index| {
                fn all_increasing<'a>(
                    level_with_skipped_element: impl Iterator<Item = &'a u32>,
                ) -> bool {
                    level_with_skipped_element
                        .tuple_windows()
                        .all(|(&a, b)| b.checked_sub(a).is_some_and(|rest| (1..=3).contains(&rest)))
                }
                let level_with_skipped_element = level
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != skipped_index)
                    .map(|(_, number)| number);

                all_increasing(level_with_skipped_element.clone())
                    || all_increasing(level_with_skipped_element.rev())
            })
        })
        .count() as u32
}

#[cfg(test)]
mod p2test {
    #[test]
    fn sample_solvable() {
        assert_eq!(super::solve(super::common::SAMPLE), 4);
    }

    // note: here was a manual snapshot test. A better solution would be to use `insta` and cache
    // results somewhere hidden?
}
