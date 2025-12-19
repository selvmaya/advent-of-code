mod shared;

use shared::*;
use std::str::FromStr;
use winnow::{error::ContextError, prelude::*, Result as WResult};

fn main() {
    let parsed: Puzzle = EXAMPLE_INPUT.parse().expect("able to parse");
    let solution: Solution = parsed.into();
    println!("Solution: {:?}", solution);
}

#[derive(Debug, PartialEq, Eq)]
struct Solution;
impl From<Puzzle> for Puzzle {
    fn from(puzzle: Puzzle) -> Self {
        todo!("unimplemented solver")
    }
}
#[derive(Debug)]
struct Puzzle;
impl FromStr for Puzzle {
    type Err = todo!();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("unimplemented parsing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{EXAMPLE_INPUT, REAL_INPUT};

    const EXAMPLE_ANSWER: Solution = todo!("not given");

    #[test]
    fn example_input_works() {
        let parsed: Puzzle = EXAMPLE_INPUT.parse().expect("able to parse");
        let solution: Solution = parse.into();
        assert_eq!(
            solution, EXAMPLE_ANSWER,
            "calculated should match expected answewr"
        );
    }

    // #[ignore]
    #[test]
    fn real_input_works() {
        let parsed: Puzzle = REAL_INPUT.parse().expect("able to parse");
        let solution: Solution = parse.into();
        panic!("GOT NEW ANSWER: {:?}", solution);
        // const KNOWN_ANSWER: Solution = todo!("unknown");
        // assert_eq!(solution, KNOWN_ANSWER, "we already know the expected answer");
    }
}
