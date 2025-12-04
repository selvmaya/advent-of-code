mod shared;
use itertools::Itertools;
use shared::{digits, Puzzle, Solution, REAL_INPUT};

fn main() {
    let parsed = REAL_INPUT.parse::<Puzzle>().expect("able to parse input");
    let solution: Solution = parsed.into();
    println!("Solution for REAL_INPUT: {}", solution.0);
}

impl From<Puzzle> for Solution {
    fn from(puzzle: Puzzle) -> Self {
        // Ranges may have "invalid" IDs.
        // Invalid IDs are IDs who have sequences of digits repeated (for p2, any amount of times), with no spare digits:
        // - 99 is invalid,
        // - 8989 is invalid,
        // - 312312 is invalid.
        // - 121212 is invalid. (*new for p2*)
        Solution(
            puzzle
                .0
                .iter()
                .flat_map(|r| r.0.clone())
                .filter(|&n| {
                    let digits = digits(n);
                    let max_sequence_len = digits.len().div_euclid(2);
                    let res = (1..=max_sequence_len)
                        .any(|sequence_len| digits.chunks(sequence_len).all_equal());
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
    const EXAMPLE_SOLUTION: Solution = Solution(4174379265);

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
        assert_eq!(solution.0, 46270373595, "known correct for input");
    }
}
