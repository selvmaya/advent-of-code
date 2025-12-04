mod shared;
use shared::{digits, Puzzle, Solution, REAL_INPUT};

fn main() {
    let parsed = REAL_INPUT.parse::<Puzzle>().expect("able to parse input");
    let solution: Solution = parsed.into();
    println!("Solution for REAL_INPUT: {}", solution.0);
}

impl From<Puzzle> for Solution {
    fn from(puzzle: Puzzle) -> Self {
        // Ranges may have "invalid" IDs.
        // Invalid IDs are IDs who have sequences of digits repeated *twice* (two times in total), with no spare digits:
        // - 99 is invalid,
        // - 8989 is invalid,
        // - 312312 is invalid.
        Solution(
            puzzle
                .0
                .iter()
                .flat_map(|r| r.0.clone())
                .filter(|&n| {
                    let digits = digits(n);
                    let res = digits.len().is_multiple_of(2)
                        && digits
                            .split_at_checked(digits.len() / 2)
                            .map(|(a, b)| a == b)
                            .expect("always in the middle");
                    if res {
                        eprintln!("{n} is an invalid ID!");
                    }
                    res
                })
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    const EXAMPLE_SOLUTION: Solution = Solution(1227775554);

    #[test]
    fn example_input_works() {
        eprintln!("Running input {EXAMPLE_INPUT}, expecting solution: {EXAMPLE_SOLUTION:?}.");
        let parsed = EXAMPLE_INPUT
            .parse::<shared::Puzzle>()
            .expect("able to parse");
        let solution: Solution = parsed.into();
        assert_eq!(solution, EXAMPLE_SOLUTION);
    }

    // #[ignore]
    #[test]
    fn real_input_works() {
        let parsed = REAL_INPUT
            .parse::<shared::Puzzle>()
            .expect("able to parse real input");
        let solution: Solution = parsed.into();
        assert_eq!(solution.0, 30599400849, "known as correct");
    }
}
