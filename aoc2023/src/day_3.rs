use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::collections::{BTreeMap, HashSet};
use std::ops::{Add, RangeInclusive};

/// below is a crude repackaging of Niashi24's solution,
/// from https://github.com/Niashi24/aoc2023/blob/master/src/day3.rs,
/// in an attempt to find *where* in my own solution I am letting a wrong a part number through.
mod imported {
    use regex::Regex;
    use std::collections::HashSet;
    use std::ops::Add;

    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    pub struct V2I {
        pub x: i32,
        pub y: i32,
    }
    impl V2I {
        pub fn new(x: i32, y: i32) -> V2I {
            V2I { x, y }
        }
    }
    impl Add for V2I {
        type Output = V2I;

        fn add(self, rhs: Self) -> Self::Output {
            Self::Output {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    pub struct Info {
        numbers: Vec<Number>,
        symbols: Vec<(char, V2I)>,
    }
    pub struct Number {
        value: u32,
        positions: Vec<V2I>,
    }
    impl Info {
        pub(crate) fn parse_file(input: String) -> Info {
            let syms = input
                .lines()
                .enumerate()
                .flat_map(|(row, line)| {
                    let row = row as i32;
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| !c.is_numeric() && c != &'.')
                        .map(move |(col, c)| (c, V2I::new(col as i32, row)))
                })
                .collect();

            // #[cfg(windows)]
            // const LINE_ENDING: &str = "\r\n";
            // #[cfg(not(windows))]
            const LINE_ENDING: &str = "\n";

            let num_regex = Regex::new(r"(\d+)").unwrap();
            let width = input.lines().next().unwrap().chars().count() + LINE_ENDING.len();
            // dbg!(width);
            let nums = num_regex
                .captures_iter(&input)
                .map(|m| {
                    let m = m.get(0).unwrap();
                    let (y, x) = (m.start() / width, m.start() % width);
                    // println!("({}, {})", x, y);
                    Number {
                        value: m.as_str().parse().unwrap(),
                        positions: (0..m.len())
                            .map(|dx| V2I::new((x + dx) as i32, y as i32))
                            .collect(),
                    }
                })
                .collect();
            Info {
                numbers: nums,
                symbols: syms,
            }
        }

        #[inline]
        fn near(a: &V2I, b: &V2I) -> bool {
            (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
        }

        pub(crate) fn part_1(&self) -> u32 {
            self.part_1_numbers().into_iter().sum()
        }
        pub(crate) fn part_1_numbers(&self) -> Vec<u32> {
            // Hashmap solution: O(s + n)
            let mut filled = HashSet::<V2I>::new();
            self.symbols.iter().for_each(|(_, p)| {
                for y in [-1, 0, 1] {
                    for x in [-1, 0, 1] {
                        filled.insert(V2I::new(x + p.x, y + p.y));
                    }
                }
            });
            self.numbers
                .iter()
                .filter(|x| x.positions.iter().any(|x| filled.contains(x)))
                .inspect(|x| {
                    if x.value == 96 {
                        dbg!(&x.positions);
                    }
                })
                .map(|x| x.value)
                // Naive solution: O(s * n)
                // self.numbers
                //     .iter()
                //     .filter(|x| {
                //         x.positions
                //             .iter()
                //             .any(|x| self.symbols.iter().any(|(_, p)| Self::near(x, p)))
                //     })
                //     .map(|x| x.value)
                .collect::<Vec<_>>()
        }
    }
}

pub(crate) fn part_1(input: &str) -> String {
    part_1_numbers(input).into_iter().sum::<u32>().to_string()
}
fn part_1_numbers(input: &str) -> Vec<u32> {
    #[derive(Debug, Clone)]
    struct Number {
        value: u32,
        area: Area, // includes position data
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
        // fn line_intersection(&self, other: &Area) -> bool {
        //     (self
        //         .horizontal
        //         .contains(&other.horizontal.start().saturating_add(1))
        //         || self
        //             .horizontal
        //             .contains(&other.horizontal.end().saturating_sub(1))
        //         || other
        //             .horizontal
        //             .contains(&self.horizontal.start().saturating_add(1))
        //         || other
        //             .horizontal
        //             .contains(&self.horizontal.end().saturating_sub(1)))
        // }
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
    eprintln!("VALUES AS ATTEMPTED:");
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
        .inspect(|x| {
            if x.value == 96 {
                dbg!(&x.area);
            }
        })
        .map(|number| number.value)
        .collect()
}

// pub(crate) fn part_2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use crate::day_3::imported::Info;
    use crate::day_3::*;
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

    // #[test]
    // fn solution_matches_answer() {
    //     let input = fs::read_to_string("input/day_3.txt").unwrap();
    //     assert_eq!(
    //         part_1(input.as_str()),
    //         imported::Info::parse_file(input).part_1().to_string()
    //     );
    // }

    #[test]
    fn numbers_match_up() {
        let input = fs::read_to_string("input/day_3.txt").unwrap();
        let own =
            part_1_numbers(input.as_str())
                .into_iter()
                .fold(BTreeMap::new(), |mut acc, val| {
                    *acc.entry(val).or_insert(0) += 1;
                    acc
                });
        let other = Info::parse_file(input).part_1_numbers().into_iter().fold(
            BTreeMap::new(),
            |mut acc, val| {
                *acc.entry(val).or_insert(0) += 1;
                acc
            },
        );
        let mut different: BTreeMap<u32, i32> = BTreeMap::new();
        for (val, amount) in own {
            if let Some(other_amount) = other.get(&val) {
                if *other_amount != amount {
                    eprintln!(
                        "{} had the wrong amount: own={}, other={}",
                        val, amount, other_amount
                    );
                    assert!(different.insert(val, amount - other_amount).is_none());
                }
            } else {
                assert!(different.insert(val, amount).is_none());
            }
        }
        assert_eq!(different, BTreeMap::new());
    }

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(part_2(""), "");
    // }
}