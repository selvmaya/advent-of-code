use itertools::Itertools;
use std::cmp::max;
use std::collections::{BTreeMap, HashSet};
use std::ops::RangeInclusive;

pub(crate) fn part_1(input: &str) -> String {
    #[derive(Debug)]
    struct Area {
        horizontal: RangeInclusive<usize>,
        vertical: RangeInclusive<usize>,
    }
    impl Area {
        fn contains(&self, x: &usize, y: &usize) -> bool {
            self.horizontal.contains(x) && self.vertical.contains(y)
        }
    }
    #[derive(Debug)]
    struct Number {
        value: u32,
        area: Area, // includes position data
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
            line_with_numbers
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

// pub(crate) fn part_2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use crate::day_3::*;

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

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(part_2(""), "");
    // }
}