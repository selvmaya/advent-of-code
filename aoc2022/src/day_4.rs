use crate::Runnable;
use std::ops::RangeInclusive;

pub struct Solution;

impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        println!(
            "Amount of pairs where one range fully contains the other: {}",
            part_1_solve(&input)
        );
        println!(
            "Amount of pairs that have any overlap at all: {}",
            part_2_solve(&input)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn part_1_example_works() {
        assert_eq!(2, part_1_solve(TEST_INPUT));
    }
    #[test]
    fn part_2_example_works() {
        assert_eq!(4, part_2_solve(TEST_INPUT));
    }

    // cached comparison (refactoring confirmation)
    // #[test]
    // fn part_1_confirmed_result_comparison() {
    //     let result_read = fs::read_to_string("outputs/day_4_part_2.txt");
    //     match result_read {
    //         Ok(r) => {
    //             match r.parse::<u32>() {
    //                 Ok(v) => {
    //                     assert_eq!(v, part_1_solve(fs::read_to_string("inputs/day_4.txt").unwrap().as_str()))
    //                 },
    //                 Err(_) => {},
    //             }
    //         },
    //         Err(_) => {}
    //     }
    // }
    // #[test]
    // fn part_2_confirmed_result_comparison() {
    //
    // }
}

// elves have been assigned the job of cleaning up sections of the camp
// every section has a unique "ID number" (we will represent this using u32)
// each elf is assigned a range of section IDs. (range or vec?)
struct Elf {
    sections: RangeInclusive<u32>,
}

impl Elf {
    fn from_range_str(range_str: &str) -> Result<Self, String> {
        let str_range = match range_str.split_once('-') {
            Some(s_r) => [s_r.0, s_r.1],
            None => {
                return Err(format!(
                    "'{}' could not be split into ranges for an elf",
                    range_str
                ))
            }
        };
        let int_r = str_range
            .iter()
            .map(|r| r.parse().expect(""))
            .collect::<Vec<u32>>();
        Ok(Elf {
            sections: int_r[0]..=int_r[1],
        })
    }
}

// many of the section assignments on the elves overlap
// the elves pair up
struct Pair {
    elves: [Elf; 2],
}

// puzzle input is a list of the section assignments, where each line is a pair
// one elf pair per puzzle input line
impl Pair {
    fn from_str_line(line: &str) -> Result<Self, String> {
        let str_elves = match &line.split_once(',') {
            Some(s) => [s.0, s.1],
            None => {
                return Err(format!(
                    "'{}' could not be split into two string literals",
                    &line
                ))
            }
        };
        Ok(Pair {
            elves: str_elves.map(|e_s| Elf::from_range_str(e_s).unwrap()),
        })
    }
}

// solution of part 1 answers how many of the pairs have an assignment range,
// that fully covers the other (2-8 fully covers 3-7)
impl Elf {
    fn fully_contains_other(&self, other: &Self) -> bool {
        self.sections.contains(other.sections.start())
            && self.sections.contains(other.sections.end())
    }
}
impl Pair {
    fn any_full_containment(&self) -> bool {
        self.elves[0].fully_contains_other(&self.elves[1])
            || self.elves[1].fully_contains_other(&self.elves[0])
    }
}

#[allow(unused)]
fn part_1_solve(input: &str) -> u32 {
    let pairs: Vec<Pair> = input
        .lines()
        .map(|line| Pair::from_str_line(line).unwrap())
        .collect();
    let contained_overlaps: u32 = pairs.iter().filter(|&p| p.any_full_containment()).count() as u32;
    contained_overlaps
}

// in part 2 we want to know how many pairs have any overlap (5-7 partially covers 7-9),
// instead of the amount of pairs with full containment
impl Elf {
    fn any_overlap_with(&self, other: &Self) -> bool {
        for section in self.sections.clone() {
            if other.sections.contains(&section) {
                return true;
            }
        }
        false
    }
}
impl Pair {
    fn find_any_overlap(&self) -> bool {
        self.elves[0].any_overlap_with(&self.elves[1])
    }
}

#[allow(unused)]
fn part_2_solve(input: &str) -> u32 {
    let pairs: Vec<Pair> = input
        .lines()
        .map(|line| Pair::from_str_line(line).unwrap())
        .collect();
    let section_overlaps: u32 = pairs.iter().filter(|p| p.find_any_overlap()).count() as u32;
    section_overlaps
}
