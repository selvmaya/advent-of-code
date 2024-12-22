#![doc = include_str!("../p1.md")]

use itertools::Itertools;
use std::fmt::{Display, Formatter};
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
    util::DayInput::find::<21>().solve_with(solve);
}

/// # Problem
/// Find the sum of the complexities for each code.
///
/// ## Definitions
/// ### Complexity of a code
/// Length of the shortest sequence (outermost) from a code.
/// ### Code
/// The (pin) number in each line,
/// which can be entered into a *numeric* keypad by using a chain of three robots.
/// ### Controlling robots
/// All three robots can be moved with a directional pad, but only the "outermost"
/// robot can be directly controlled.
/// A robot can move one button at a time in a cardinal direction.
/// It *cannot* move to a spot where there is no button.
/// Control of where the robot is/points at, is done with *directional* keypads.
/// The directional pad is four directinal buttons (downwards being in the middle/south),
/// and one 'pass press' button, meaning that the robot passes a press from its pad to the next
/// pad in the chain (robot directional pad *or* numeric keypad).
/// Until that happens, the next thing in line is not touched.
/// This works because the "currently controlled" (best thought of as the outermost) thing is
/// "inherently" being controlled by our inputs: We control the outermost keypad with no other
/// movements needed, and this works at every level if we can abstract the system that makes
/// keypresses become keypresses further down into a trivial thing.
/// ### Layers (inner/outer)
/// The innermost keypad is the numeric keypad (controlling a "door").
/// The "middle" keypads are the directional pads that control two robots.
/// The outermost keypad is the directional pad we get to perform 'atomically',
/// or without further partial moves (also controlling a robot).
/// 0 - Door keypad
/// 1 - Robot 1
/// 2 - Robot 2
/// 3 - Robot 3 (controlled inherently/by us)
/// ## Observations
/// ### Symmetries
/// Some symmetric or equal series of movements are strictly less convenient to make than others,
/// assuming that they are possible. This includes things like moving down-right compared to
/// up-right: `up` is two movements away from `right`, `down` is only one away from `right`.
/// ### Non-useful moves
/// We can go back and forth or in circles, but that does not move the next thing in line.
/// ### Stacks of cost
/// It might be possible that the cost of some outermost move is worth it,
/// at least compared to what we might have needed to do in the inner layers.
///
/// # Solution
/// For each code:
/// Find the sortest sequence of button presses on the outermost directional keypad,
/// then find out the complexity of this sequence.
/// The complexities are summed to a number.
/// ## Approach
/// ### Simplicity
/// We probably do not want to implement layer-specific functions,
/// and instead should reuse the same core decision search algorithm.
/// ### Search leveling
/// This decision search may benefit from being DFS in that many same-level searches that get to
/// the same place is more time is just not relevant, and such it can be skipped.
/// Like-wise a branch can improve another's leafs if it gets to the same place,
/// but in fewer steps. DFS would however *need* some special terminating condition,
/// which can be hard to ensure correct. BFS will eliminate loop-like branches trivially.
/// ### State reuse
/// If robot 3 can move from button `x` to button `y` on robot 2's keypad, then the same move for
/// robot 2 to move on robot 1's keypad has the same cost/benefit.
/// The door keypad is different in that it is numeric and differently shaped,
/// but the act of moving coordinates is in principle very similar,
/// even if the meaning of pushing the buttons at those coordinates is different.
/// ### Modeling
/// Express the problem in explicit state invariants, before any false leaps may be attempted.
/// It is easier to implement a more optimal solution using a partial solution.
/// ### Movements
/// A cost from button `x` to button `y` is known ahead of time, it is static.
/// There is no need to calculate this, and it can be encoded with the models.
/// ### Per-level cost evaluation
/// The minimum cost can be known ahead of time, which can create an optimal sequence always.
/// It can always be created because you can always move in some L shape,
/// and that L shape is optimal since it reuses the direction before turning.
/// This being optimal (simplest) for one level, automatically makes it simplest for all
/// levels above it (outer). These optimal paths can be generated when needed, with some depth.
/// ### Searching across levels
/// We need to know how to (legally) move between buttons on each keypad, and value-based costs
/// do not provide this on their own. The value they create is merely measuring the cost of
/// moving inside a single level (where the outermost level has no cost).
/// This can be used to find a series of moves that are optimal, before actually creating them.
/// Knowing the optimal path for one level does not create the outcome we require,
/// we still need to search for combinations of costs and compare approaches.
/// An observation to have here is that since knowing the innermost/last level is trivial,
/// it means we only really need to implement searches for all the other levels.
/// This eliminates the need for a search that handles anything other than *directional* pads.
fn solve(input: impl AsRef<str>) -> u64 {
    let codes = parse_sequences
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    codes
        .into_iter()
        .map(|code| {
            let sequence = code.generate_optimal_sequence();
            sequence.0.len() as u64 * code.to_number() as u64
        })
        .sum()
}

struct Sequence(Vec<DirButton>);
impl Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().join(""))
    }
}

/// I assume all codes are three digits long?
/// It is that long for my input and for the examples.
struct Code([Num; 3]);
impl Code {
    fn generate_optimal_sequence(&self) -> Sequence {
        todo!()
    }
    fn to_number(&self) -> u32 {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, val)| 10u32.pow(i as u32) * u32::from(val))
            .sum()
    }
}
impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}A: {}",
            self.0.iter().join(""),
            self.generate_optimal_sequence()
        )
    }
}

fn parse_sequences(input: &mut &str) -> PResult<Vec<Code>> {
    separated(1.., parse_sequence, ('A', line_ending)).parse_next(input)
}
fn parse_sequence(input: &mut &str) -> PResult<Code> {
    (parse_num, parse_num, parse_num)
        .map(|(a, b, c)| Code([a, b, c]))
        .parse_next(input)
}
fn parse_num(input: &mut &str) -> PResult<Num> {
    let mut num_parser = dispatch! {any;
        '1' => empty.value(Num::One),
        '2' => empty.value(Num::Two),
        '3' => empty.value(Num::Three),
        '4' => empty.value(Num::Four),
        '5' => empty.value(Num::Five),
        '6' => empty.value(Num::Six),
        '7' => empty.value(Num::Seven),
        '8' => empty.value(Num::Eight),
        '9' => empty.value(Num::Nine),
        '0' => empty.value(Num::Zero),
        _ => fail,
    };
    num_parser.parse_next(input)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Num {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}
impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u32::from(self))
    }
}
impl From<&Num> for u32 {
    fn from(value: &Num) -> Self {
        match value {
            Num::One => 1,
            Num::Two => 2,
            Num::Three => 3,
            Num::Four => 4,
            Num::Five => 5,
            Num::Six => 6,
            Num::Seven => 7,
            Num::Eight => 8,
            Num::Nine => 9,
            Num::Zero => 0,
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum NumButton {
    Num(Num),
    Accept,
}
impl TravelCost for NumButton {
    fn a_to_b_cost(&self, other: &Self) -> KeypadDistance {
        // this could be generated using some simple inductive rules
        match self {
            NumButton::Num(Num::One) => match other {
                NumButton::Num(Num::One) => 0,
                NumButton::Num(Num::Two) => 1,
                NumButton::Num(Num::Three) => 2,
                NumButton::Num(Num::Four) => 1,
                NumButton::Num(Num::Five) => 2,
                NumButton::Num(Num::Six) => 3,
                NumButton::Num(Num::Seven) => 2,
                NumButton::Num(Num::Eight) => 3,
                NumButton::Num(Num::Nine) => 4,
                NumButton::Num(Num::Zero) => 2,
                NumButton::Accept => 3,
            },
            NumButton::Num(Num::Two) => match other {
                NumButton::Num(Num::One) => 1,
                NumButton::Num(Num::Two) => 0,
                NumButton::Num(Num::Three) => 1,
                NumButton::Num(Num::Four) => 2,
                NumButton::Num(Num::Five) => 1,
                NumButton::Num(Num::Six) => 2,
                NumButton::Num(Num::Seven) => 3,
                NumButton::Num(Num::Eight) => 2,
                NumButton::Num(Num::Nine) => 3,
                NumButton::Num(Num::Zero) => 1,
                NumButton::Accept => 2,
            },
            NumButton::Num(Num::Three) => match other {
                NumButton::Num(Num::One) => 2,
                NumButton::Num(Num::Two) => 1,
                NumButton::Num(Num::Three) => 0,
                NumButton::Num(Num::Four) => 3,
                NumButton::Num(Num::Five) => 2,
                NumButton::Num(Num::Six) => 1,
                NumButton::Num(Num::Seven) => 4,
                NumButton::Num(Num::Eight) => 3,
                NumButton::Num(Num::Nine) => 2,
                NumButton::Num(Num::Zero) => 2,
                NumButton::Accept => 1,
            },
            NumButton::Num(Num::Four) => match other {
                NumButton::Num(Num::One) => 1,
                NumButton::Num(Num::Two) => 2,
                NumButton::Num(Num::Three) => 3,
                NumButton::Num(Num::Four) => 0,
                NumButton::Num(Num::Five) => 1,
                NumButton::Num(Num::Six) => 2,
                NumButton::Num(Num::Seven) => 1,
                NumButton::Num(Num::Eight) => 2,
                NumButton::Num(Num::Nine) => 3,
                NumButton::Num(Num::Zero) => 3,
                NumButton::Accept => 4,
            },
            NumButton::Num(Num::Five) => match other {
                NumButton::Num(Num::One) => 2,
                NumButton::Num(Num::Two) => 1,
                NumButton::Num(Num::Three) => 2,
                NumButton::Num(Num::Four) => 1,
                NumButton::Num(Num::Five) => 0,
                NumButton::Num(Num::Six) => 1,
                NumButton::Num(Num::Seven) => 2,
                NumButton::Num(Num::Eight) => 1,
                NumButton::Num(Num::Nine) => 2,
                NumButton::Num(Num::Zero) => 2,
                NumButton::Accept => 3,
            },
            NumButton::Num(Num::Six) => match other {
                NumButton::Num(Num::One) => 3,
                NumButton::Num(Num::Two) => 2,
                NumButton::Num(Num::Three) => 1,
                NumButton::Num(Num::Four) => 2,
                NumButton::Num(Num::Five) => 1,
                NumButton::Num(Num::Six) => 0,
                NumButton::Num(Num::Seven) => 3,
                NumButton::Num(Num::Eight) => 2,
                NumButton::Num(Num::Nine) => 1,
                NumButton::Num(Num::Zero) => 3,
                NumButton::Accept => 2,
            },
            NumButton::Num(Num::Seven) => match other {
                NumButton::Num(Num::One) => 2,
                NumButton::Num(Num::Two) => 3,
                NumButton::Num(Num::Three) => 4,
                NumButton::Num(Num::Four) => 1,
                NumButton::Num(Num::Five) => 2,
                NumButton::Num(Num::Six) => 3,
                NumButton::Num(Num::Seven) => 0,
                NumButton::Num(Num::Eight) => 1,
                NumButton::Num(Num::Nine) => 2,
                NumButton::Num(Num::Zero) => 4,
                NumButton::Accept => 5,
            },
            NumButton::Num(Num::Eight) => match other {
                NumButton::Num(Num::One) => 3,
                NumButton::Num(Num::Two) => 2,
                NumButton::Num(Num::Three) => 3,
                NumButton::Num(Num::Four) => 2,
                NumButton::Num(Num::Five) => 1,
                NumButton::Num(Num::Six) => 2,
                NumButton::Num(Num::Seven) => 1,
                NumButton::Num(Num::Eight) => 0,
                NumButton::Num(Num::Nine) => 1,
                NumButton::Num(Num::Zero) => 3,
                NumButton::Accept => 4,
            },
            NumButton::Num(Num::Nine) => match other {
                NumButton::Num(Num::One) => 4,
                NumButton::Num(Num::Two) => 3,
                NumButton::Num(Num::Three) => 2,
                NumButton::Num(Num::Four) => 3,
                NumButton::Num(Num::Five) => 2,
                NumButton::Num(Num::Six) => 1,
                NumButton::Num(Num::Seven) => 2,
                NumButton::Num(Num::Eight) => 1,
                NumButton::Num(Num::Nine) => 0,
                NumButton::Num(Num::Zero) => 4,
                NumButton::Accept => 3,
            },
            NumButton::Num(Num::Zero) => match other {
                NumButton::Num(Num::One) => 2,
                NumButton::Num(Num::Two) => 1,
                NumButton::Num(Num::Three) => 2,
                NumButton::Num(Num::Four) => 3,
                NumButton::Num(Num::Five) => 2,
                NumButton::Num(Num::Six) => 2,
                NumButton::Num(Num::Seven) => 4,
                NumButton::Num(Num::Eight) => 3,
                NumButton::Num(Num::Nine) => 4,
                NumButton::Num(Num::Zero) => 0,
                NumButton::Accept => 1,
            },
            NumButton::Accept => match other {
                NumButton::Num(Num::One) => 3,
                NumButton::Num(Num::Two) => 2,
                NumButton::Num(Num::Three) => 1,
                NumButton::Num(Num::Four) => 4,
                NumButton::Num(Num::Five) => 3,
                NumButton::Num(Num::Six) => 2,
                NumButton::Num(Num::Seven) => 5,
                NumButton::Num(Num::Eight) => 4,
                NumButton::Num(Num::Nine) => 3,
                NumButton::Num(Num::Zero) => 1,
                NumButton::Accept => 0,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum DirButton {
    Dir(Dir),
    Accept,
}
impl Display for DirButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DirButton::Dir(Dir::Up) => '^',
                DirButton::Dir(Dir::Down) => 'v',
                DirButton::Dir(Dir::Left) => '<',
                DirButton::Dir(Dir::Right) => '>',
                DirButton::Accept => 'A',
            }
        )
    }
}
impl TravelCost for DirButton {
    fn a_to_b_cost(&self, other: &Self) -> KeypadDistance {
        // note: this could be a hashmap, but that would have more paths.
        // This is also ensured to be complete/exhaustive
        match (self, other) {
	        // non-move, for completeness
            (DirButton::Accept, DirButton::Accept)
            | (DirButton::Dir(Dir::Up), DirButton::Dir(Dir::Up))
            | (DirButton::Dir(Dir::Down), DirButton::Dir(Dir::Down))
            | (DirButton::Dir(Dir::Left), DirButton::Dir(Dir::Left))
            | (DirButton::Dir(Dir::Right), DirButton::Dir(Dir::Right))
            => 0,
	        // up-down
	        (DirButton::Dir(Dir::Up), DirButton::Dir(Dir::Down))
		    | (DirButton::Dir(Dir::Down), DirButton::Dir(Dir::Up))
	        // left-down
	        | (DirButton::Dir(Dir::Left), DirButton::Dir(Dir::Down))
	        | (DirButton::Dir(Dir::Down), DirButton::Dir(Dir::Left))
	        // right-down
	        | (DirButton::Dir(Dir::Right), DirButton::Dir(Dir::Down))
	        | (DirButton::Dir(Dir::Down), DirButton::Dir(Dir::Right))
	        // a-up
	        | (DirButton::Accept, DirButton::Dir(Dir::Up))
	        | (DirButton::Dir(Dir::Up), DirButton::Accept)
	        // a-right
	        | (DirButton::Accept, DirButton::Dir(Dir::Right))
	        | (DirButton::Dir(Dir::Right), DirButton::Accept)
	        => 1,
	        // left-right
	        (DirButton::Dir(Dir::Left), DirButton::Dir(Dir::Right))
	        | (DirButton::Dir(Dir::Right), DirButton::Dir(Dir::Left))
	        // left-up
	        | (DirButton::Dir(Dir::Left), DirButton::Dir(Dir::Up))
	        | (DirButton::Dir(Dir::Up), DirButton::Dir(Dir::Left))
	        // up-right
	        | (DirButton::Dir(Dir::Up), DirButton::Dir(Dir::Right))
	        | (DirButton::Dir(Dir::Right), DirButton::Dir(Dir::Up))
	        // a-down
	        | (DirButton::Accept, DirButton::Dir(Dir::Down))
	        | (DirButton::Dir(Dir::Down), DirButton::Accept)
	        => 2,
	        // a-left
            (DirButton::Accept, DirButton::Dir(Dir::Left))
            | (DirButton::Dir(Dir::Left), DirButton::Accept)
            => 3,
        }
    }
}

type KeypadDistance = u8;
trait TravelCost {
    fn a_to_b_cost(&self, other: &Self) -> KeypadDistance;
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 126384);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/21")), 0);
    }
}
