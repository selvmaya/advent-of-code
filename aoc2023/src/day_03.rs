use crate::SolverFn;
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::collections::{BTreeMap, HashSet};
use std::ops::{Add, RangeInclusive};

pub(crate) const PARTS: &[SolverFn] = &[part_1, part_2];

fn part_1(input: &str) -> String {
    #[derive(Debug, Clone)]
    struct Number {
        value: u32,
        area: Area, // position data can be extracted from this
    }
    #[derive(Debug, Clone)]
    struct Area {
        horizontal: RangeInclusive<usize>,
        vertical: RangeInclusive<usize>,
    }
    impl Area {
        fn contains(&self, x: &usize, y: &usize) -> bool {
            self.horizontal.contains(x) && self.vertical.contains(y)
        }
        fn horizontally_contains(&self, other: &Area) -> bool {
            self.horizontal.start() <= other.horizontal.start()
                && self.horizontal.end() >= other.horizontal.end()
        }
    }

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| !c.is_ascii_digit() && *c != '.')
                .map(move |(x, _c)| (x, y))
        })
        .collect::<Vec<_>>();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line_with_numbers = line
                .chars()
                .map(|c| if c.is_ascii_digit() { c } else { ' ' })
                .collect::<String>();
            let all_substring_matches = line_with_numbers
                .split_ascii_whitespace()
                .unique()
                .flat_map(|digits| {
                    line.match_indices(digits).map(|(x, number)| Number {
                        value: number.parse().expect("is parsable number"),
                        area: Area {
                            horizontal: (x.saturating_sub(1))
                                ..=(x.saturating_add(number.chars().count())),
                            vertical: (y.saturating_sub(1))..=(y.saturating_add(1)),
                        },
                    })
                })
                .collect::<Vec<_>>();
            all_substring_matches
                .iter()
                .filter(|num| {
                    // filter out numbers found in line that are a subsequence of another
                    !all_substring_matches.iter().any(|other_num| {
                        let other_string = other_num.value.to_string();
                        let self_string = num.value.to_string();
                        other_string.chars().count() > self_string.chars().count()
                            && other_string.contains(&self_string)
                            && other_num.area.horizontally_contains(&num.area)
                    })
                })
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<Number>>()
        .into_iter()
        .filter(|number| symbols.iter().any(|(x, y)| number.area.contains(x, y)))
        .map(|number| number.value)
        .sum::<u32>()
        .to_string()
}

fn part_2(input: &str) -> String {
    #[derive(Debug, Clone)]
    struct Number {
        value: u32,
        area: Area, // position data can be extracted from this
    }
    #[derive(Debug, Clone)]
    struct Area {
        horizontal: RangeInclusive<usize>,
        vertical: RangeInclusive<usize>,
    }
    impl Area {
        fn contains(&self, x: &usize, y: &usize) -> bool {
            self.horizontal.contains(x) && self.vertical.contains(y)
        }
        fn horizontally_contains(&self, other: &Area) -> bool {
            self.horizontal.start() <= other.horizontal.start()
                && self.horizontal.end() >= other.horizontal.end()
        }
    }

    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line_with_numbers = line
                .chars()
                .map(|c| if c.is_ascii_digit() { c } else { ' ' })
                .collect::<String>();
            let all_substring_matches = line_with_numbers
                .split_ascii_whitespace()
                .unique()
                .flat_map(|digits| {
                    line.match_indices(digits).map(|(x, number)| Number {
                        value: number.parse().expect("is parsable number"),
                        area: Area {
                            horizontal: (x.saturating_sub(1))
                                ..=(x.saturating_add(number.chars().count())),
                            vertical: (y.saturating_sub(1))..=(y.saturating_add(1)),
                        },
                    })
                })
                .collect::<Vec<_>>();
            all_substring_matches
                .iter()
                .filter(|num| {
                    // filter out numbers found in line that are a subsequence of another
                    !all_substring_matches.iter().any(|other_num| {
                        let other_string = other_num.value.to_string();
                        let self_string = num.value.to_string();
                        other_string.chars().count() > self_string.chars().count()
                            && other_string.contains(&self_string)
                            && other_num.area.horizontally_contains(&num.area)
                    })
                })
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<Number>>();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '*') // gear symbol
                .map(move |(x, _c)| (x, y))
        })
        .filter_map(|(x, y)| {
            numbers
                .iter()
                .filter(|number| number.area.contains(&x, &y))
                .collect_tuple()
        })
        .map(|(a, b)| a.value * b.value)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::day_03::*;
    use std::collections::btree_map::Entry;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn part_1_works() {
        assert_eq!(
            part_1(
                r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "4361"
        );
    }

    #[test]
    fn part_2_works() {
        assert_eq!(
            part_2(
                r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "467835"
        );
    }
}