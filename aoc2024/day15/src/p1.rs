#![doc = include_str!("../p1.md")]

use itertools::Itertools;
use std::collections::HashSet;
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
    util::DayInput::find::<15>().solve_with(solve);
}

/// # Problem
/// Simulate a robot moving around a warehouse of (immovable) walls and (movable) boxes.
/// What is the sum of the "GPS coordinates" for each box?
/// # Definitions
/// ## GPS coodinate
/// XY coodinate from the top, but compressed to a single value:
/// `f(x,y) = 100x + 1y`
/// ## XY coordinate
/// Amount of spaces from (0 would mean overlapping) the edges of the map.
/// X: spaces from left
/// Y: spaces from top
/// ## Warehouse & robot
/// A set of immovable and movable obstacles, along with a robot.
/// The robot can move boxes (`O`), but cannot move walls (`#`).
/// A box can be moved but cannot enter a wall. If a robot move would move a box into the wall,
/// it does not happen.
/// There are also empty spaces in the warehouse (`.`).
fn solve(input: impl AsRef<str>) -> u64 {
    let (mut warehouse, moves) = parse.parse_next(&mut input.as_ref()).expect("parsable");
    for robot_move in moves {
        warehouse.process_move(&robot_move)
    }
    warehouse
        .boxes
        .iter()
        .map(|box_pos| {
            100 * (1 + (box_pos.0 / warehouse.width)) as u64 + (box_pos.0 % warehouse.width) as u64
        })
        .sum()
}

impl Warehouse {
    fn process_move(&mut self, dir: &Direction) {
        if let Some(walk_pos) = self.robot.checked_move(self.width, dir) {
            if !self.walls.contains(&walk_pos) {
                // no walls, we may push or just move
                if !self.boxes.contains(&walk_pos) {
                    // no boxes, we can move
                    self.robot = walk_pos;
                } else {
                    // we may be able to push
                    let mut target_pos = walk_pos.checked_move(self.width, dir);
                    while target_pos.is_some_and(|pos| self.boxes.contains(&pos)) {
                        target_pos = walk_pos.checked_move(self.width, dir);
                    }
                    if let Some(end_pos) = target_pos.filter(|pos| self.walls.contains(pos)) {
                        self.boxes.insert(end_pos);
                        self.boxes.remove(&walk_pos);
                        self.robot = walk_pos;
                    }
                }
            }
        }
    }
}
impl Pos {
    fn checked_move(&self, width: usize, dir: &Direction) -> Option<Self> {
        match dir {
            Direction::Up => self.0.checked_sub(width),
            Direction::Down => self.0.checked_add(width),
            Direction::Left => self.0.checked_sub(1),
            Direction::Right => self.0.checked_add(1),
        }
        .map(Pos)
    }
}

fn parse(input: &mut &str) -> PResult<(Warehouse, Vec<Direction>)> {
    separated_pair(parse_warehouse, (line_ending, line_ending), parse_moves).parse_next(input)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn parse_moves(input: &mut &str) -> PResult<Vec<Direction>> {
    repeat(1.., opt(parse_direction))
        .map(|v: Vec<_>| v.into_iter().flatten().collect())
        .parse_next(input)
}
fn parse_direction(input: &mut &str) -> PResult<Direction> {
    alt((
        '^'.value(Direction::Up),
        'v'.value(Direction::Down),
        '<'.value(Direction::Left),
        '>'.value(Direction::Right),
    ))
    .parse_next(input)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos(usize);
/// Map walls (edges) are implicit, not part of data
struct Warehouse {
    boxes: HashSet<Pos>,
    walls: HashSet<Pos>,
    robot: Pos,
    width: usize,
}
#[derive(Copy, Clone, Eq, PartialEq)]
enum MapItem {
    Wall,
    Box,
    Robot,
    Empty,
}
fn parse_warehouse(input: &mut &str) -> PResult<Warehouse> {
    delimited(
        (parse_horizontal_edge, line_ending),
        separated(1.., parse_horizontal_inner, line_ending).map(|v: Vec<Vec<_>>| {
            debug_assert!(v.iter().map(|v| v.len()).all_equal()); // warn:
                                                                  // this crashes while parsing
            let width = v[0].len();
            let (boxes, interior_walls, robot) = v
                .into_iter()
                .enumerate()
                .flat_map(|(y, xs)| xs.into_iter().map(move |(x, variant)| (y * x, variant)))
                .fold(
                    (HashSet::new(), HashSet::new(), None),
                    |(mut boxes, mut walls, mut robot), (pos, variant)| {
                        match variant {
                            MapItem::Wall => {
                                walls.insert(Pos(pos));
                            }
                            MapItem::Box => {
                                boxes.insert(Pos(pos));
                            }
                            MapItem::Robot => {
                                debug_assert_eq!(robot, None);
                                robot = Some(Pos(pos));
                            }
                            _ => unreachable!("empty is discarded earlier"),
                        }
                        (boxes, walls, robot)
                    },
                );
            Warehouse {
                boxes,
                walls: interior_walls,
                robot: robot.expect("should be a robot somewhere"),
                width,
            }
        }),
        parse_horizontal_edge,
    )
    .parse_next(input)
}
fn parse_horizontal_inner(input: &mut &str) -> PResult<Vec<(usize, MapItem)>> {
    trace(
        "parsing middle line",
        (
            delimited(
                '#',
                trace(
                    "reading map items (inner)",
                    repeat(0.., parse_item).map(|v: Vec<_>| {
                        v.into_iter()
                            .enumerate()
                            .filter(|&(_, variant)| variant != MapItem::Empty)
                            .collect()
                    }),
                ),
                '#',
            ),
            trace("leave early", not((line_ending, line_ending))),
        ),
    )
    .map(|(a, _)| a)
    .parse_next(input)
}
fn parse_item(input: &mut &str) -> PResult<MapItem> {
    (
        alt((
            '#'.value(MapItem::Wall),
            'O'.value(MapItem::Box),
            '@'.value(MapItem::Robot),
            '.'.value(MapItem::Empty),
        )),
        not(line_ending),
    )
        .map(|(a, ..)| a)
        .parse_next(input)
}
fn parse_horizontal_edge<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., '#').parse_next(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 104);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/15")), 0);
    }
}
