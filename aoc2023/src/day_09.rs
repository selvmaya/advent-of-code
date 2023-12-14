use crate::SolverFn;

pub(crate) const PARTS: &[SolverFn] = &[part_1::part_1, part_2::part_2];

#[cfg(test)]
const TEST_INPUT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

mod part_1 {
    use itertools::Itertools;

    type Number = i64;

    /// A "history" is a sequence of numbers.
    /// Traverse downwards in differences (derivatives?) between values in a history.
    /// Add another zero at the bottom (a derivative value must be a some point equal zero),
    /// then add the last level above to the derivative value (0 at first),
    /// and traverse back up:
    /// 1 3 6
    ///  3 3
    ///   0
    /// ..becomes..
    /// 1 3 6 9
    ///  3 3 3
    ///   0 0
    /// Here, 9 is the future value we are looking for.
    /// Sum up the predicted/future value for each history (line of input lines).
    pub(super) fn part_1(input: &str) -> String {
        input
            .lines()
            .map(|line| {
                traverse(
                    line.split_ascii_whitespace()
                        .map(|s| s.parse::<Number>().unwrap())
                        .collect(),
                )
            })
            .sum::<Number>()
            .to_string()
    }

    fn traverse(mut sequence: Vec<Number>) -> Number {
        let lower_sequence = sequence
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();
        sequence.pop().unwrap()
            + if lower_sequence.iter().all(|num| *num == 0) {
                0
            } else {
                traverse(lower_sequence)
            }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use super::*;

        #[test]
        fn sample_works() {
            assert_eq!(part_1(TEST_INPUT), "114");
        }
    }
}

mod part_2 {
    use itertools::Itertools;

    type Number = i64;

    /// Just like part 1, but instead of adding a zero to the end we at it at the front.
    pub(super) fn part_2(input: &str) -> String {
        input
            .lines()
            .map(|line| {
                eprintln!(" --- History: {}", line);
                traverse(
                    line.split_ascii_whitespace()
                        .map(|s| s.parse::<Number>().unwrap())
                        .collect(),
                )
            })
            .sum::<Number>()
            .to_string()
    }

    fn traverse(mut sequence: Vec<Number>) -> Number {
        let lower_sequence = sequence
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();
        sequence.first().unwrap()
            - if lower_sequence.iter().all(|num| *num == 0) {
                0
            } else {
                traverse(lower_sequence)
            }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use super::*;

        #[test]
        fn sample_works() {
            assert_eq!(part_2(TEST_INPUT), "2");
        }
    }
}