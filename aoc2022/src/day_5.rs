use std::str::FromStr;
use lazy_static::lazy_static;
use regex::{Regex};
use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {

    }
}

struct CrateStack {
    crates: Vec<Crate>
}

#[derive(Copy, Clone)]
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

fn stacks_after_crates_line(mut stacks: Vec<CrateStack>, line: &str) -> Vec<CrateStack> {
    lazy_static! {
        static ref RE_CRATE_PATTERN: Regex = Regex::new(r"   |\[[A-Z]\]").unwrap();
    }

    let line_crates: Vec<Option<Crate>> = RE_CRATE_PATTERN.find_iter(line)
        .map(|c| crate_from_str(c.as_str())).collect();

    for mut crate_index in 0..line_crates.len() {
        let option_crate = line_crates[crate_index]; // copy
        if let Some(c) = option_crate {
            stacks[crate_index].crates.push(c);
        }
    }
    stacks
}

struct Move {
    from_index: usize,
    count: usize,
    to_index: usize,
}
fn move_from_line(line: &str) -> Move {
    let info: Vec<&str> = line.splitn(3, ' ').collect();
    let from_index: usize = info[0].parse().unwrap();
    let count: usize = info[1].parse().unwrap();
    let to_index: usize = info[2].parse().unwrap();
    Move {
        from_index,
        count,
        to_index,
    }
}
fn stack_without_crate(mut stack: CrateStack, index: usize) -> (CrateStack, Crate) {
    let removed_crate = stack.crates.remove(index);
    (stack, removed_crate)
}
fn stack_with_crate(mut stack: CrateStack, new_crate: Crate) -> CrateStack {
    stack.crates.push(new_crate);
    stack
}
fn stacks_after_move(mut stacks: Vec<CrateStack>, line: &str) -> Vec<CrateStack> {
    // build move
    let mut my_move = move_from_line(line);
    my_move.count = stacks[my_move.from_index].crates.len(); // cap amount to take
    for _index in 0..my_move.count {
        stacks[my_move.from_index] = stack_with_crate(stacks[my_move.to_index, );
    }
    stacks
}

fn part_1_solve(input: &str) -> String {
    let parts = input.split_once("\r\n\r\n").unwrap();
    dbg!(parts);

    // create starting condition
    let mut stacks: Vec<CrateStack> = Vec::new();
    for line_of_crates in parts.0.lines() {
        stacks = stacks_after_crates_line(stacks, line_of_crates);
    }
    // operate on state using moves (lines)
    for line_of_move in parts.1.lines() {
        stacks = stacks_after_move(stacks, line_of_move);
    }

    // print the top crates of each stack (skip over empty)
    todo!()
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
    fn part_1_example_works() {
        assert_eq!("CMZ", part_1_solve(TEST_INPUT))
    }

}
