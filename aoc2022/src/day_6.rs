use itertools::Itertools;

use crate::Runnable;

pub(crate) struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        part_1_solve(input.as_str());
        part_2_solve(input.as_str());
    }
}

// find first "start-of-packet" but remember how many characters it took to find it.
fn part_1_solve(input: &str) {
    println!("answer (part 1) = {}", find_marker(input, 4).unwrap());
}

fn part_2_solve(input: &str) {
    println!("answer (part 2) = {}", find_marker(input, 14).unwrap())
}

fn find_marker(input: &str, sequence_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(sequence_size)
        .position(|window| window.iter().unique().count() == sequence_size)
        .map(|pos| pos + sequence_size)
}

#[cfg(test)]
mod tests {
    use super::find_marker;
    mod part_1 {
        use super::find_marker;
        use test_case::test_case;
        #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
        #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
        #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
        #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
        #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
        fn test_find_marker(index: usize, input: &str) {
            assert_eq!(Some(index), find_marker(input, 4));
        }
    }
    mod part_2 {
        use super::find_marker;
        use test_case::test_case;
        #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
        #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
        #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
        #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
        #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
        fn test_find_marker(index: usize, input: &str) {
            assert_eq!(Some(index), find_marker(input, 14));
        }
    }
}