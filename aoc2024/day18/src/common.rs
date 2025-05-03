use std::collections::HashSet;
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

pub const STANDARD_SIZE: GraphDistance = 70;

pub fn parse_bytes(input: &mut &str) -> PResult<Vec<Pos>> {
    separated(1.., parse_byte, line_ending).parse_next(input)
}
fn parse_byte(input: &mut &str) -> PResult<Pos> {
    separated_pair(dec_uint, ',', dec_uint)
        .map(|(x, y)| Pos { x, y })
        .parse_next(input)
}

/// u8 because axis is 0..=70
pub type GraphDistance = u8;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: GraphDistance,
    pub y: GraphDistance,
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// SIZE is the square bounds, inclusive (70 contains 0..=70).
pub trait Grid<const SIZE: GraphDistance> {
    fn contains_obstacle(&self, pos: &Pos) -> bool;
    fn neighboors(&self, pos: &Pos) -> impl Iterator<Item = Pos> + '_ {
        [
            // up
            pos.y.checked_sub(1).map(|y| Pos { x: pos.x, y }),
            // down
            pos.y.checked_add(1).map(|y| Pos { x: pos.x, y }),
            // left
            pos.x.checked_sub(1).map(|x| Pos { x, y: pos.y }),
            // right
            pos.x.checked_add(1).map(|x| Pos { x, y: pos.y }),
        ]
        .into_iter()
        .flatten()
        .filter(|possible| possible.x <= SIZE && possible.y <= SIZE)
        .filter(|pos| !self.contains_obstacle(pos))
    }
    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=SIZE {
            for x in 0..=SIZE {
                write!(
                    f,
                    "{}",
                    if self.contains_obstacle(&Pos { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct SetGraph<const SIZE: GraphDistance>(pub HashSet<Pos>);
impl<const SIZE: GraphDistance> Grid<SIZE> for SetGraph<SIZE> {
    fn contains_obstacle(&self, pos: &Pos) -> bool {
        self.0.contains(pos)
    }
}
impl<const SIZE: GraphDistance> Display for SetGraph<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}
