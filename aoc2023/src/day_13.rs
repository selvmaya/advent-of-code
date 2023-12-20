use crate::SolverFn;
use itertools::{EitherOrBoth, Itertools};
use std::error::Error;
use std::fmt::Display;

pub(crate) const PARTS: &[SolverFn] = &[part_1 /*, part_2*/];

fn try_find_point_of_mirror<S: Display + PartialEq>(correct_layout_lines: &[S]) -> Option<usize> {
    // eprintln!(
    //     "Interpreted downwards:\n{}",
    //     correct_layout_lines.iter().join("\n")
    // );
    let mut seen = Vec::new();
    let mut point_of_mirror = None;
    for (i, (a, b)) in correct_layout_lines.iter().tuple_windows().enumerate() {
        // eprintln!("Scanning for mirror on column {}: {}, next: {}", i, a, b);
        if a == b {
            // eprintln!("Found potential mirror spot (equal parts)...");
            if seen
                .iter()
                .rev()
                .zip_longest(correct_layout_lines[(i + 2)..].iter())
                .enumerate()
                .all(|(j, either_or_both)| match either_or_both {
                    EitherOrBoth::Both(a, b) => {
                        // eprintln!("Comparing {} distance {}: {}=={}", i, j + 1, a, b);
                        // eprintln!("Equal? {}", *a == b);
                        *a == b
                    }
                    _ => true,
                })
            {
                point_of_mirror = Some(i);
                break;
            }
        }
        seen.push(a);
    }
    point_of_mirror
}
fn part_1(input: &str) -> String {
    input
        .lines()
        .join("\n")
        .split("\n\n")
        .map(|pattern| {
            // eprintln!("\nNew pattern!\n");
            if let Some(mirror_row) = {
                let lines = pattern.lines().collect::<Vec<_>>();
                try_find_point_of_mirror(&lines)
            } {
                // eprintln!("Mirror found on row {}.", mirror_row);
                (mirror_row + 1) * 100
            } else if let Some(mirror_column) = {
                // eprintln!("\nNot found in rows, trying columns...");
                // need to repackage first
                let mut data = pattern
                    .lines()
                    .map(|line| line.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                let oriented_data = (0..data[0].len())
                    .map(|x| {
                        (0..data.len())
                            .map(|y| data[y][x])
                            .rev()
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>();
                try_find_point_of_mirror(&oriented_data)
            } {
                // eprintln!("Mirror found on column {}.", mirror_column);
                mirror_column + 1
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

// fn part_2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    const TEST_INPUT: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT), "405");
    }

    #[test]
    #[ignore]
    fn part_2_works() {
        todo!();
        // assert_eq!(part_2(KNOWN_INPUT), "_");
    }
}