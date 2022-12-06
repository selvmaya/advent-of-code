use std::mem::take;
use crate::Runnable;

pub struct Solution;

impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        println!("Crates on top of each stack: {}", part_1_solve(&input));
        // println!("Crates on top of each stack: {}", part_2_solve(&input));
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
// supplies are stored in stacks of crates with a label (written with square brackets)
// their starting position is our puzzle input
// crates are labeled (in input) as upper case letters with label (char)
#[derive(Clone)]
struct Crate {
    label: char
}
// in the input the stacks have labels (numbers 1 and up) but it doesn't seem needed
#[derive(Clone)]
struct Stack {
    crates: Vec<Crate> // stacked from bottom to top
}
impl Stack {
    fn with_crate_on_top(&self, crate_to_add: Crate) -> Self {
        let mut new_stack = self.clone();
        new_stack.crates.push(crate_to_add);
        new_stack
    }
    fn with_crates_on_top(&self, crates: Vec<Crate>) -> Self {
        let mut new_stack = self.clone();
        for crate_to_add in crates {
            new_stack = new_stack.with_crate_on_top(crate_to_add);
        }
        new_stack
    }
    fn without_top_crate(&self) -> (Self, Crate) { // from top
        let mut new_stack = self.clone();
        let removed_crate = new_stack.crates.remove(new_stack.crates.len() - 1);
        (new_stack, removed_crate)
    }
    fn without_crates_from_top(&self, amount: &u32) -> (Self, Vec<Crate>) {
        let mut new_stack = self.clone();
        let mut removed_crates: Vec<Crate> = Vec::new();
        for _i in 0..*amount {
            let crane_move_temp = new_stack.without_top_crate();
            new_stack = crane_move_temp.0;
            removed_crates.push(crane_move_temp.1);
        }
        (new_stack, removed_crates)
    }
}

// the ship has a cargo crane that can move crates between stacks
#[derive(Clone)]
struct Crane {
    stacks: Vec<Stack>
}
impl Crane {
    fn from_str(drawing: &str) -> Self {
        let crate_lines = drawing.lines().filter(|line| line.contains("[")).collect::<Vec<&str>>();
        let platform_line = drawing.lines().filter(|line| line.contains("1")).collect::<Vec<&str>>()[0];
        let crates_2d: Vec<Vec<Crate>> = Vec::new();
        for crate_line in crate_lines {
            let crates_in_line: Vec<Crate> = Vec::new();

        }

        todo!()
    }
}
// the crane operator wil rearrange them in series of steps (bottom of puzzle input)
struct Move {
    crate_amount: u32,
    stack_source: usize,
    stack_target: usize,
}
impl Move {
    fn from_str(line: &str) -> Self {
        // get crate amount ("move x")
        // get source stack index ("from y")
        // get target stack index ("to z")
        todo!()
    }
}

// after the rearrangement, the correct crates will be at the top of each stack
// to solve part 1, we just need to know what crates end up at the top of each stack.
impl Crane {
    fn top_crate_labels(&self) -> String {
        self.stacks.iter()
            .map(|s| s.crates.first().expect("No crates in stack?"))
            .map(|c| c.label).collect::<String>()
    }
    fn after_move(self, new_move: &Move) -> Self {
        let mut new_crane = self.clone();
        let temp_crane_move = new_crane.stacks[new_move.stack_source]
            .without_crates_from_top(&new_move.crate_amount);
        new_crane.stacks[new_move.stack_source] = temp_crane_move.0;
        new_crane.stacks[new_move.stack_target] = new_crane.stacks[0].with_crates_on_top(temp_crane_move.1);
        new_crane
    }
}

#[allow(unused)]
fn part_1_solve(input: &str) -> String {
    let parts = input.split_once("\r\n\r\n").unwrap(); // might only work on windows idk

    // construct crate layout (top of input)
    let mut crane = Crane::from_str(parts.0);

    // gather and perform moves
    let moves: Vec<Move> = parts.1.lines().map(|line| Move::from_str(line)).collect();
    for new_move in moves {
        crane = crane.after_move(&new_move);
    }

    crane.top_crate_labels() // return output
}