use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::{Regex};
use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        println!("Crates on top of each stack after moves: {}", part_1_solve(input.as_str()));
    }
}

#[derive(Debug)]
struct CrateStack {
    crates: Vec<Crate>
}

#[derive(Copy, Clone, Debug)]
struct Crate {
    label: char
}

fn crate_from_str(s: &str) -> Option<Crate> {
    lazy_static! {
        static ref RE_LABEL: Regex = Regex::new(r"[A-Z]").unwrap();
    }

    let label = RE_LABEL.find(s)?.as_str();
    let label: char = label.chars().next()?;
    Some(Crate {
        label
    })
}

fn stacks_after_crates_line(stacks: &mut HashMap<usize, CrateStack>, line: &str) {
    lazy_static! {
        static ref RE_CRATE_PATTERN: Regex = Regex::new(r"   |\[[A-Z]\]").unwrap();
    }

    let line_crates: Vec<Option<Crate>> = RE_CRATE_PATTERN.find_iter(line)
        .map(|c| crate_from_str(c.as_str())).collect();
    for crate_key in 1..=line_crates.len() {
        // dbg!(crate_index);
        let option_crate = line_crates[crate_key - 1]; // copy
        // dbg!(option_crate);
        if let Some(c) = option_crate {
            if !stacks.contains_key(&crate_key) {
                stacks.insert(crate_key, CrateStack {crates: Vec::new()});
            }
            stacks.get_mut(&crate_key).unwrap().crates.push(c);
        }
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    source: usize,
    target: usize,
}
fn move_from_line(line: &str) -> Move {
    lazy_static! {
        static ref RE_COUNT: Regex = Regex::new(r"move [0-9]").unwrap(); // replace with unspecific number?
        static ref RE_FROM_INDEX: Regex = Regex::new(r"from [0-9]").unwrap(); // replace with unspecific number?
        static ref RE_TO_INDEX: Regex = Regex::new(r"to [0-9]").unwrap(); // replace with unspecific number?
    }
    let from = RE_FROM_INDEX.find(line).unwrap().as_str();
    let from: usize = from.split_once(' ').unwrap().1.parse().unwrap();
    let count = RE_COUNT.find(line).unwrap().as_str();
    let count: usize = count.split_once(' ').unwrap().1.parse().unwrap();
    let to = RE_TO_INDEX.find(line).unwrap().as_str();
    let to: usize = to.split_once(' ').unwrap().1.parse().unwrap();
    Move {
        source: from,
        count,
        target: to,
    }
}
fn stack_without_crate(stack: &mut CrateStack) -> Crate {
    stack.crates.remove(stack.crates.len() - 1)
}
fn stacks_after_move(stacks: &mut HashMap<usize, CrateStack>, move_line: &str) {
    // build move
    let mut my_move = move_from_line(move_line);
    let max_crates_to_take = stacks.get(&my_move.source).unwrap().crates.len();
    if max_crates_to_take < my_move.count {
        my_move.count = max_crates_to_take;
    }
    dbg!(&my_move);
    // use move
    let mut crates_taken: Vec<Crate> = Vec::new();
    {
        let mut from_stack = stacks.get_mut(&my_move.source).unwrap();
        for _ in 0..my_move.count {
            if from_stack.crates.len() > 0 {
                crates_taken.push(stack_without_crate(&mut from_stack));
            }
        }
    }
    let to_stack = stacks.get_mut(&my_move.target).unwrap();
    for crate_taken in crates_taken {
        to_stack.crates.push(crate_taken);
    }
}

fn part_1_solve(input: &str) -> String {
    let parts = match input.split_once("\r\n\r\n") {
        Some(p) => p,
        None => input.split_once("\n\n").unwrap(),
    };

    // create starting condition
    let mut stacks: HashMap<usize, CrateStack> = HashMap::new();
    for line_of_crates in parts.0.lines() {
        stacks_after_crates_line(&mut stacks, line_of_crates);
    }
    dbg!(&stacks);
    // operate on state using moves (lines)
    for line_of_move in parts.1.lines() {
        stacks_after_move(&mut stacks, line_of_move);
    }
    dbg!(&stacks);

    // print the top crates of each stack (skip over empty)
    let mut top: String = String::new();
    for stack in stacks.values() {
        let top_crate = stack.crates.first();
        dbg!(top_crate);
        if let Some(top_crate) = top_crate {
            top.push(top_crate.label);
        };
    }
    top
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
        assert_eq!("CMZ", part_1_solve(TEST_INPUT))
    }
}
