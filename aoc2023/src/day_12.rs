use crate::SolverFn;
use std::error::Error;

pub(crate) const PARTS: &[SolverFn] = &[/*part_1, part_2*/];

#[derive(Debug, PartialEq)]
enum KnownSpringData {
    Operational,
    Damaged,
    Unknown,
}

struct SpringData(Option<KnownSpringData>);

impl TryFrom<char> for SpringData {
    type Error = Box<dyn Error>;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(SpringData(match c {
            '.' => Some(KnownSpringData::Operational),
            '#' => Some(KnownSpringData::Damaged),
            '?' => None,
            _ => Err("")?,
        }))
    }
}
fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (data, requirements) = line.split_once(' ').unwrap();
            let data = data
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<SpringData>>();
            let requirements = requirements
                .split(',')
                .map(|digit| digit.parse().unwrap())
                .collect::<Vec<usize>>();
            // just find all combinations lol
            find_valid_combinations_count(
                data.as_slice(),
                requirements.as_slice().try_into().unwrap(),
            )
        })
        .sum::<usize>()
        .to_string()
}
fn find_valid_combinations_count(data: &[SpringData], requirements: &[usize; 3]) -> usize {
    todo!()
}

fn part_2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    //     const KNOWN_INPUT: &str = r"#.#.### 1,1,3
    // .#...#....###. 1,1,3
    // .#.###.#.###### 1,3,1,6
    // ####.#...#... 4,1,1
    // #....######..#####. 1,6,5
    // .###.##....# 3,2,1";

    const DAMAGED_INPUT: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part_1_works() {
        // assert_eq!(part_1(KNOWN_INPUT), "21");
        assert_eq!(part_1(DAMAGED_INPUT), "21");
    }

    #[test]
    #[ignore]
    fn part_2_works() {
        todo!();
        // assert_eq!(part_2(KNOWN_INPUT), "_");
    }
}