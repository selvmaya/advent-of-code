use crate::Runnable;

pub struct Solution;

impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        println!("Crates on top of each stack: {}", part_1_solve(&input));

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn part_1_example_works() {
        assert_eq!("CMZ", part_1_solve(TEST_INPUT));
    }
}

// "expedition" can depart as soon as the final supplies have been unloaded from the ships.
// supplies are stored in stacks of crates with a label (written as
// their starting position is our puzzle input
struct Crate {
    label: char
}
// in the input the stacks have labels (numbers 1 and up) but it doesn't seem needed
struct Stack {
    crates: Vec<Crate>
}
// crates are labeled (in input) as upper case letters with label (char)

// the ship has a cargo crane that can move crates between stacks
// the crane operator wil rearrange them in series of steps (bottom of puzzle input)
struct Crane {
    stacks: Vec<Stack>
}

// after the rearrangement, the correct crates will be at the top of each stack
// to solve part 1, we just need to know what crates end up at the top of each stack.
impl Crane {
    fn top_crate_labels(&self) -> String {
        self.stacks.iter()
            .map(|s| s.crates.first().expect("No crates in stack?"))
            .map(|c| c.label).collect::<String>()
    }
}

#[allow(unused)]
fn part_1_solve(input: &str) -> String {
    // construct crate layout (top of input)
    let split_input = input.split_once("\n\n").expect("Could not split input!");

    let stacks = vec![];
    let crane = Crane {
        stacks
    };

    // move crates around (bottom of input)


    crane.top_crate_labels()
}