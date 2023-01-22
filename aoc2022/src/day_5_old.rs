use lazy_static::lazy_static;
use crate::Runnable;
use regex::*;

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
    #[ignore]
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
impl Crate {
    fn from_str(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[A-Z]").unwrap();
        }

        return match input.contains(' ') {
            true => None,
            false => RE.find(input).map(|c| Crate { label: c.as_str().parse().unwrap() }),
        };
    }
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
    fn separate_crate_from_top(&self) -> (Self, Crate) { // from top
        let mut new_stack = self.clone();
        let removed_crate = new_stack.crates.remove(self.crates.len() - 1);
        (new_stack, removed_crate)
    }
}

// the ship has a cargo crane that can move crates between stacks
#[derive(Clone)]
struct Crane {
    stacks: Vec<Stack>
}
impl Crane {
    fn from_str(drawing: &str) -> Self {
        lazy_static! {
            static ref RE_NUMBER: Regex = Regex::new(r"[1-9]").unwrap();
            static ref RE_CRATE: Regex = Regex::new(r"    |\[[A-Z]\]").unwrap(); // 4 spaces or crate str eg: "[X]"
        }

        // get platform numbers
        let platform: &str = drawing.lines()
            .filter(|line| line.contains('1')).collect::<Vec<&str>>()[0]; // just numbers
        let numbers: Vec<u32> = RE_NUMBER.find_iter(platform)
            .map(|m| m.as_str().parse().unwrap()).collect();

        // get crates (and empty spaces)
        let crates_lines: Vec<&str> = drawing.lines()
            .filter(|line| line.contains('[')).collect(); // all lines with at least 1 crate
        // in order to use indices at the end we need to include spaces as "empty" crates (Option::None)
        let option_crates: Vec<Vec<Option<Crate>>> = crates_lines.iter()
            .map(|line| RE_CRATE.find_iter(line)
                .map(|m| Crate::from_str(m.as_str())).collect()).collect();

        // create stacks from crates and platform numbers
        let mut stacks: Vec<Stack> = vec![Stack { crates: Vec::new() }; numbers.len()];
        for mut option_crate_line in option_crates {
            for i in 0..option_crate_line.len() - 1 {
                let option_crate: Option<Crate> = option_crate_line.remove(i);
                match option_crate {
                    None => {}, // ignore empty spaces
                    Some(c) => stacks[i].crates.push(c)
                }
            }
        }

        Crane { stacks }
    }
}
// the crane operator wil rearrange them in series of steps (bottom of puzzle input)
struct Move {
    crate_amount: u32,
    source: usize,
    target: usize,
}
impl Move {
    fn from_str(line: &str) -> Self {
        lazy_static! {
            // get crate amount ("move x")
            static ref CRATE_RE: Regex = Regex::new(r"move [1-99]").unwrap();
            // get source stack index ("from y")
            static ref SOURCE_RE: Regex = Regex::new(r"from [1-9]").unwrap();
            // get target stack index ("to z")
            static ref TARGET_RE: Regex = Regex::new(r"to [1-9]").unwrap();
            // get actual index/number from string
            static ref NUMBER_RE: Regex = Regex::new(r"[1-99]").unwrap();
        }
        let crate_str = CRATE_RE.find(line).unwrap().as_str();
        let source_str = SOURCE_RE.find(line).unwrap().as_str();
        let target_str = TARGET_RE.find(line).unwrap().as_str();
        Move {
            crate_amount: NUMBER_RE.find(crate_str).unwrap().as_str().parse().unwrap(),
            source: NUMBER_RE.find(source_str).unwrap().as_str().parse().unwrap(),
            target: NUMBER_RE.find(target_str).unwrap().as_str().parse().unwrap()
        }
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
    // fn find_stack(&self, number: &usize) -> &Stack {
    //     &self.stacks[*number - 1] // offset index
    // }
    fn after_move(&self, new_move: &Move) -> Crane {
        let mut new_crane = self.clone();
        let mut move_count = new_move.crate_amount;
        if move_count >= self.stacks[new_move.source].crates.len() as u32 {
            move_count = self.stacks[new_move.source].crates.len() as u32;
        }
        for _i in 0..move_count {
            let temp = self.stacks[new_move.source].separate_crate_from_top();
            new_crane.stacks[new_move.source] = temp.0;
            new_crane.stacks[new_move.target] = self.stacks[new_move.target].with_crate_on_top(temp.1);
        }
        new_crane
    }
}

#[allow(unused)]
fn part_1_solve(input: &str) -> String {
    println!("Input: {}", &input);
    let parts = match input.split_once("\r\n\r\n") {
        None => input.split_once("\n\n").unwrap(),
        Some(p) => p
    };

    // construct crate layout (top of input)
    let mut crane = Crane::from_str(parts.0);

    // gather and perform moves
    let moves: Vec<Move> = parts.1.lines().map(Move::from_str).collect();
    for new_move in moves {
        crane = crane.after_move(&new_move);
    }

    crane.top_crate_labels() // return output
}