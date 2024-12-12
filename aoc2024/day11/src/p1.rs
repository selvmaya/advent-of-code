#![doc = include_str!("../p1.md")]

#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

mod common;
use crate::common::*;

fn main() {
    util::DayInput::find::<11>().solve_with(solve);
}

/// # Problem
/// A number may change or splits into two numbers, shifting other numbers
/// (linear sequence) All the numbers change "at once" (during each update) and
/// they do not affect each other.
///
/// Their behaviour during every update follow
/// this list such that they do the first thing applicable:
/// - 0 is replaced by 1
/// - A number with an even number of digits is replaced by two numbers, whose
///   digits is made by the splitting of the digits of the old number (left
///   number is the left digits, right digits for right) (which works because
///   the number has an even amount of digits)
/// - For other numbers, the number is multiplied by 2024
///
/// The order is preserved. Splitting does not change this.
fn solve(input: impl AsRef<str>) -> u64 {
    let initial = parse_stones
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    after_n_blinks::<25>(initial).0.len() as u64
}

fn after_n_blinks<const N: u8>(mut stones: Stones) -> Stones {
    for blinks in 0..N {
        println!("Blink level: {}", blinks);
        stones = after_blink(stones);
    }
    stones
}

pub fn after_blink(stones: Stones) -> Stones {
    Stones(
        stones
            .0
            .into_iter()
            .flat_map(|stone| {
                match process(stone) {
                    (l, Some(r)) => vec![l, r],
                    (l, None) => vec![l],
                }
                .into_iter()
            })
            .collect(),
    )
}

pub fn process(stone: Stone) -> (Stone, Option<Stone>) {
    match stone.0 {
        // num is zero
        0 => (Stone(1), None),
        // if amount of digits in number is even
        n if (n.ilog10() + 1) % 2 == 0 => {
            let digit_len = n.ilog10() + 1;
            let half_digit_len = digit_len / 2;
            let cut_off: u64 = 10_u64.pow(half_digit_len);
            let left_digits = n / cut_off;
            let right_digits = n % cut_off;
            (Stone(left_digits), Some(Stone(right_digits)))
        }
        // all other numbers
        n => (Stone(n * 2024), None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Stone, Stones};

    #[test]
    fn stones_blink() {
        assert_eq!(
            after_blink(Stones(vec![Stone(12), Stone(519), Stone(141), Stone(4200)])),
            Stones(vec![
                Stone(1),
                Stone(2),
                Stone(519 * 2024),
                Stone(141 * 2024),
                Stone(42),
                Stone(0)
            ])
        )
    }

    #[test]
    fn stone_processing() {
        assert_eq!(process(Stone(12345678)), (Stone(1234), Some(Stone(5678))));
    }

    #[test]
    fn samples() {
        assert_eq!(solve("125 17"), 55312)
    }

    #[test]
    fn input_solvable() {
        assert_eq!(solve(include_str!("../../inputs/11")), 186996);
    }
}
