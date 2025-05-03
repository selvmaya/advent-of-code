#![doc = include_str!("../p1.md")]

use itertools::Itertools;
use std::cmp::Ordering;
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

const STANDARD_WIDTH: Scalar = 101;
const STANDARD_HEIGHT: Scalar = 103;
fn main() {
    util::DayInput::find::<14>().solve_with(solve::<STANDARD_WIDTH, STANDARD_HEIGHT>);
}

/// # Problem
/// Robots are in a (grid) space.
/// They move, and do not interact with each other.
/// When walking past the edge (defined by problem), they wrap around ("teleport").
/// After 100 moves, what quadrant of the space has the least robots?
/// # Solution
/// To wrap around, modulus can be applied per-component,
/// and this works as long as positions are 0-indexed.
/// Quadrants can be used to semantically group-map robots by their position.
/// This is done using component comparison.
fn solve<const W: Scalar, const H: Scalar>(input: impl AsRef<str>) -> u64 {
    let mut space = Space::<W, H> {
        robots: parse_input
            .parse_next(&mut input.as_ref())
            .expect("parsable"),
    };
    const STANDARD_MOVES: usize = 100;
    space.move_robots::<STANDARD_MOVES>();
    space.safety_factor()
}

type Scalar = i16;
struct Space<const W: Scalar, const H: Scalar> {
    robots: Vec<Robot>,
}
impl<const WIDTH: Scalar, const HEIGHT: Scalar> Space<WIDTH, HEIGHT> {
    //    const WIDTH: Scalar = W;
    //    const HEIGHT: Scalar = H;
    fn move_robots<const MOVES: usize>(&mut self) {
        eprintln!("Space BEFORE move(s):\n{}", self);
        for _i in 0..MOVES {
            // say, 100 times
            for robot in self.robots.iter_mut() {
                // per robot
                robot.pos.x = (robot.pos.x + robot.vel.x).rem_euclid(WIDTH);
                robot.pos.y = (robot.pos.y + robot.vel.y).rem_euclid(HEIGHT);
            }
        }
        eprintln!("Space after {} move(s):\n{}", MOVES, self);
        //	    eprintln!("Robots after move(s):\n{}\n", self.as_robots());
    }
    fn safety_factor(&self) -> u64 {
        eprintln!("Calculating safety:\n{}\n", self.as_quads());
        let middle_x = const { (HEIGHT - 1) / 2 };
        let middle_y = const { (HEIGHT - 1) / 2 };
        self.robots
            .iter()
            .map(|robot| robot.pos)
            .map(
                |robot_pos| match (robot_pos.x.cmp(&middle_x), robot_pos.y.cmp(&middle_y)) {
                    (Ordering::Less, Ordering::Less) => Quadrant::TopLeft,
                    (Ordering::Greater, Ordering::Less) => Quadrant::TopRight,
                    (Ordering::Less, Ordering::Greater) => Quadrant::BottomLeft,
                    (Ordering::Greater, Ordering::Greater) => Quadrant::BottomRight,
                    (Ordering::Equal, _) | (_, Ordering::Equal) => Quadrant::MiddleEdge,
                },
            )
            .counts()
            .into_iter()
            .filter_map(|(quadrant, len)| match quadrant {
                Quadrant::MiddleEdge => None,
                _ => Some(len as u64),
            })
            .product()
    }
	#[allow(unused)]
	fn as_quads(&self) -> QuadSpace<WIDTH, HEIGHT> {
        QuadSpace(self)
    }
	#[allow(unused)]
	fn as_robots(&self) -> RobotList {
        RobotList(&self.robots)
    }
}
impl<const WIDTH: Scalar, const HEIGHT: Scalar> Display for Space<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let robots_by_pos = self.robots.iter().map(|r| r.pos).counts();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = Position { x, y };
                write!(
                    f,
                    "{}",
                    match robots_by_pos.get(&pos) {
                        None => '.',
                        Some(&n) => char::from_digit(n as u32, 16).unwrap(),
                    }
                )?;
            }
            if y != HEIGHT - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
#[allow(unused)]
struct QuadSpace<'s, const W: Scalar, const H: Scalar>(&'s Space<W, H>);
impl<const WIDTH: Scalar, const HEIGHT: Scalar> Display for QuadSpace<'_, WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let robots_by_pos = self.0.robots.iter().map(|r| r.pos).counts();
        for y in 0..HEIGHT {
            if y != (HEIGHT - 1) / 2 {
                for x in 0..WIDTH {
                    if x != (WIDTH - 1) / 2 {
                        let pos = Position { x, y };
                        write!(
                            f,
                            "{}",
                            match robots_by_pos.get(&pos) {
                                None => '.',
                                Some(&n) => char::from_digit((n as u32).clamp(0, 16), 16).unwrap(),
                            }
                        )?;
                    } else {
                        write!(f, " ")?;
                    }
                }
            }
            if y != HEIGHT - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
#[allow(unused)]
struct RobotList<'r>(&'r Vec<Robot>);
impl Display for RobotList<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for robot in self.0 {
            writeln!(
                f,
                "p={},{} v={},{}",
                robot.pos.x, robot.pos.y, robot.vel.x, robot.vel.y
            )?;
        }
        Ok(())
    }
}

// for robots -> group map of robots by key (quadrant variant)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    // note: middle is ignored, despite odd width & height
    MiddleEdge,
}

#[derive(Clone)]
struct Robot {
    pos: Position,
    vel: Velocity,
}
/// Note: Area known to be 101 wide and 103 tall
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: Scalar,
    y: Scalar,
}
/// Note: components seem to be -101 to 101 ish?
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Velocity {
    x: Scalar,
    y: Scalar,
}
fn parse_input(input: &mut &str) -> PResult<Vec<Robot>> {
    separated(1.., parse_robot, line_ending).parse_next(input)
}
fn parse_robot(input: &mut &str) -> PResult<Robot> {
    separated_pair(parse_pos, " ", parse_vel)
        .map(|(pos, vel)| Robot { pos, vel })
        .parse_next(input)
}
fn parse_pos(input: &mut &str) -> PResult<Position> {
    preceded(
        "p=",
        separated_pair(dec_int, ",", dec_int).map(|(x, y)| Position { x, y }),
    )
    .parse_next(input)
}
fn parse_vel(input: &mut &str) -> PResult<Velocity> {
    preceded(
        "v=",
        separated_pair(dec_int, ",", dec_int).map(|(x, y)| Velocity { x, y }),
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_solvable() {
        use crate::Scalar;
        const EXAMPLE_WIDTH: Scalar = 11;
        const EXAMPLE_HEIGHT: Scalar = 7;
        assert_eq!(
            super::solve::<EXAMPLE_WIDTH, EXAMPLE_HEIGHT>(include_str!("EXAMPLE")),
            12
        );
    }

//        #[ignore]
    #[test]
    fn input_solvable() {
        use super::{STANDARD_HEIGHT, STANDARD_WIDTH};
        assert_ne!(
            super::solve::<STANDARD_WIDTH, STANDARD_HEIGHT>(include_str!("../../inputs/14")),
            230357106,
            "too high"
        );
        assert_eq!(
            super::solve::<STANDARD_WIDTH, STANDARD_HEIGHT>(include_str!("../../inputs/14")),
            0,
            "unknown"
        );
    }
}