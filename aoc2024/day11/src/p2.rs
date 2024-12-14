#![doc = include_str!("../p2.md")]

mod common;

use std::collections::HashMap;

use common::*;
#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

fn main() {
    util::DayInput::find::<11>().solve_with(solve);
}

fn solve(input: impl AsRef<str>) -> u64 {
    let initial = parse_stones(&mut input.as_ref()).expect("parsable");
    len_after_blinks::<75>(initial)
}

#[derive(Hash, Eq, PartialEq)]
struct State {
    stone: Stone,
    blinks_remain: u8,
}
fn len_after_blinks<const N: u8>(stones: Stones) -> u64 {
    fn compute_branch(stone: Stone, blinks: u8, visited: &mut HashMap<State, u64>) -> u64 {
        match stone.0 {
            0 => len_from_state(
                State {
                    stone: Stone(1),
                    blinks_remain: blinks,
                },
                visited,
            ),
            // if amount of digits in number is even
            n if (n.ilog10() + 1) % 2 == 0 => {
                let digit_len = n.ilog10() + 1;
                let half_digit_len = digit_len / 2;
                let cut_off: u64 = 10_u64.pow(half_digit_len);
                let left_digits = n / cut_off;
                let right_digits = n % cut_off;
                len_from_state(
                    State {
                        stone: Stone(left_digits),
                        blinks_remain: blinks,
                    },
                    visited,
                ) + len_from_state(
                    State {
                        stone: Stone(right_digits),
                        blinks_remain: blinks,
                    },
                    visited,
                )
            }
            // all other numbers
            n => len_from_state(
                State {
                    stone: Stone(n * 2024),
                    blinks_remain: blinks,
                },
                visited,
            ),
        }
    }
    fn len_from_state(state: State, visited: &mut HashMap<State, u64>) -> u64 {
        if let Some(&prev) = visited.get(&state) {
            // previously visited branch
            prev
        } else {
            // unvisited state
            let new_result = match state.blinks_remain {
                // reached leaf for the first time
                0 => 1,
                // new branch
                blinks => compute_branch(state.stone, blinks - 1, visited),
            };
            let prev = visited.insert(state, new_result);
            assert_eq!(prev, None);
            new_result
        }
    }

    let mut visited: HashMap<State, u64> = HashMap::new();
    stones
        .0
        .into_iter()
        .map(|stone| {
            len_from_state(
                State {
                    stone,
                    blinks_remain: N,
                },
                &mut visited,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::common::{Stone, Stones};

    #[test]
    fn samples() {
        assert_eq!(
            super::len_after_blinks::<25>(Stones(vec![Stone(125), Stone(17)])),
            55312
        )
    }

    #[test]
    fn input_solvable() {
        assert_ne!(
            super::solve(include_str!("../../inputs/11")),
            613318790,
            "number is too low"
        );
        assert_eq!(
            super::solve(include_str!("../../inputs/11")),
            221683913164898,
            "number is too low"
        );
    }
}
