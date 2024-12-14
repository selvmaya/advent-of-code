#![doc = include_str!("../p1.md")]

use std::ops::{AddAssign, Sub};

#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    prelude::*,
    stream::*,
    token::*,
    {PResult, Parser},
};

fn main() {
    util::DayInput::find::<13>().solve_with(solve);
}

/// # Problem
/// Machines (divided by newline in input) each have one prize.
/// Prizes are located at X;Y poisitions (denoted X=N),
/// and buttons (A, B) can move the claw/targeting by some X;Y amount each.
/// For every machine, the `A` button costs 3 tokens and the `B` button costs 1
/// token. Find the minimum amount of tokens needed to win all the prizes.
/// # Solution
/// Choosing between buttons many times and pruning branches until the best
/// value is found.
fn solve(input: impl AsRef<str>) -> u64 {
    let machines = parse_machines
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    machines
        .iter()
        .filter_map(|machine| machine.optimal_cost())
        .sum()
}

fn parse_machines(input: &mut &str) -> PResult<Vec<Machine>> {
    separated(1.., parse_machine, (line_ending, line_ending)).parse_next(input)
}
fn parse_machine(input: &mut &str) -> PResult<Machine> {
    separated_pair(
        separated_pair(parse_button_offset, line_ending, parse_button_offset),
        line_ending,
        parse_prize_pos,
    )
    .map(|((a_offset, b_offset), prize_pos)| Machine {
        a_offset,
        b_offset,
        prize_pos,
    })
    .parse_next(input)
}
fn parse_button_offset(input: &mut &str) -> PResult<Vec2> {
    (
        preceded(("Button ", alt(('A', 'B')), ": X+"), dec_uint),
        preceded(", Y+", dec_uint),
    )
        .map(|(x, y)| Vec2 { x, y })
        .parse_next(input)
}
fn parse_prize_pos(input: &mut &str) -> PResult<Vec2> {
    (preceded("Prize: X=", dec_uint), preceded(", Y=", dec_uint))
        .map(|(x, y)| Vec2 { x, y })
        .parse_next(input)
}

#[derive(Debug, Clone)]
struct Machine {
    a_offset: Vec2,
    b_offset: Vec2,
    prize_pos: Vec2,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Vec2 {
    x: u64,
    y: u64,
}
impl Vec2 {
    fn magnitude(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    fn divisible_by(&self, other: Vec2) -> bool {
        other.x % self.x == 0 || other.y % self.y == 0
    }
}
type TokenCount = u64;

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Machine {
    /// No need to figure out what the actual press sequence was. The token cost
    /// is all we care about.
    /// We want to use the "biggest" (magnitude) vector as much as possible,
    /// but this might only be possible with certain offsets.
    /// We simply use the only other vector until that is possible.
    fn optimal_cost(&self) -> Option<TokenCount> {
        // checks if the range of I can be made up for by J, but does not consider if it is
        // possible with multiple combinations of I and J, just I + N J (?)
        let x_possible = {
            (self.prize_pos.x % self.a_offset.x) % self.b_offset.x == 0
                || (self.prize_pos.x % self.b_offset.x) % self.a_offset.x == 0
        };
        // todo: handle cases where it is impossible to get a prize from a machine.
        // there is a hint that it should never take >100 button presses in total to

        if self.a_offset == self.b_offset {
            // use `B` cost always, assume divisible
            return Some(self.prize_pos.x / self.b_offset.x);
        }
        let ((small, small_cost), (big, big_cost)) =
            if self.a_offset.magnitude() < self.b_offset.magnitude() {
                ((self.a_offset, 3), (self.b_offset, 1))
            } else {
                ((self.b_offset, 1), (self.a_offset, 3))
            };
        let mut walk_pos = Vec2 { x: 0, y: 0 };
        let mut cost = 0;
        let remaining = self.prize_pos - walk_pos;
        while !remaining.divisible_by(big) {
            walk_pos += small;
            cost += small_cost;
        }
        Some(cost + ((remaining.x / big.x) * big_cost))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 480);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        // 		assert_eq!(super::solve(include_str!("../../inputs/13")), todo!());
    }
}
