#![doc = include_str!("../p1.md")]

use itertools::Itertools;
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
    .map(|((a_offset, b_offset), prize_pos)| Machine::new(a_offset, b_offset, prize_pos))
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
    a: Button,
    b: Button,
    prize_pos: Vec2,
}

impl Machine {
    fn new(a_offset: Vec2, b_offset: Vec2, prize_pos: Vec2) -> Self {
        Self {
            a: Button {
                offset: a_offset,
                cost: 3,
            },
            b: Button {
                offset: b_offset,
                cost: 1,
            },
            prize_pos,
        }
    }
    /// No need to figure out what the actual press sequence was. The token cost
    /// is all we care about.
    /// We want to use the "biggest" (magnitude) vector as much as possible,
    /// but this might only be possible with certain offsets.
    /// We simply use the only other vector until that is possible.
    fn optimal_cost(&self) -> Option<TokenCount> {
        // the optimal path is one that:
        // 1) is possible/valid
        // 2) uses the biggest of two vectors as much as possible
        // we can search for some amount of small vector, and see if any amount of it can
        // create us a path only completable by the big vector.
        // if no combination of only the small vector can create a path for the small vector,
        // the path is impossible.
        // We cannot have negative values of either vector,
        // and the case where that would be necessary can be assumed to not exist.
        // The bigger vector is *always* the best vector if it can reach the target,
        // while also being of the biggest magnitude. We just may also need the smaller vector.
        // There are no reasons to try combinations of A and B, only xA and yB.
        let (min, max) = [&self.a, &self.b]
            .into_iter()
            .sorted_by_key(|a| a.offset.magnitude() as u32)
            .collect_tuple()
            .unwrap();
        (0..=100)
            .find(|&count: &u64| {
                let offset = Vec2 {
                    x: min.offset.x * count,
                    y: min.offset.y * count,
                };
                self.prize_pos.x.checked_sub(offset.x).is_some_and(|x| {
                    self.prize_pos.y.checked_sub(offset.y).is_some_and(|y| {
                        let remaining = Vec2 { x, y };
                        let divisible = remaining.divisible_by(max.offset);
                        divisible
                    })
                })
            })
            .map(|min_count| {
                let max_count = (self.prize_pos.x - (max.offset.x * min_count)) / max.offset.x;
                min_count * min.cost as u64 + max_count + max.cost as u64
            })
    }
}
#[derive(Debug, Clone)]
struct Button {
    offset: Vec2,
    cost: TokenCost,
}
type TokenCost = u8;
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
        self.x % other.x == 0 && self.y % other.y == 0
    }
}
type TokenCount = u64;

//impl AddAssign for Vec2 {
//    fn add_assign(&mut self, rhs: Self) {
//        self.x += rhs.x;
//        self.y += rhs.y;
//    }
//}

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
