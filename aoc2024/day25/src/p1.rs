#![doc = include_str!("../p1.md")]

use itertools::Itertools;
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

fn main() {
    util::DayInput::find::<25>().solve_with(solve);
}

/// # Problem
/// Some locks, some keys.
/// A key fits with a lock if columns do not overlap.
/// NOTE: They do *not* have to match exactly!
/// How many unique lock/key pairs fit together?
/// NOTE: It is not specified if locks and keys are unique,
/// but if combinations must be unique, then there is no reason to process duplicate schematics.
/// # Solution
/// If every column value in a key is less than or equal to that of a lock's,
/// the key fits. The count of a filtered iterator from combinations.
fn solve(input: impl AsRef<str>) -> u64 {
    let (locks, keys) = parse_locks_and_keys
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| {
            lock.0
                .into_iter()
                .zip(key.0)
                .all(|(l, k)| (COLUMN_SIZE as u8) - (2 + l) >= k)
        })
        .count() as u64
}
const ROW_SIZE: usize = 5;
const COLUMN_SIZE: usize = 7;

type SchematicFit = [u8; ROW_SIZE];
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Lock(SchematicFit);
impl Display for Lock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "LOCK:")?;
        writeln!(f, "{}", "#".repeat(ROW_SIZE))?;
        for increasing_height in 1..(COLUMN_SIZE - 1) {
            for x in 0..ROW_SIZE {
                let max_height_at_column = self.0[x];
                write!(
                    f,
                    "{}",
                    if increasing_height <= max_height_at_column as usize {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        write!(f, "{}", ".".repeat(ROW_SIZE))
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Key(SchematicFit);
impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "KEY:")?;
        writeln!(f, "{}", ".".repeat(ROW_SIZE))?;
        for decreasing_height in (1..(COLUMN_SIZE - 1)).rev() {
            for x in 0..ROW_SIZE {
                let max_height_at_column = self.0[x];
                write!(
                    f,
                    "{}",
                    if decreasing_height <= max_height_at_column as usize {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        write!(f, "{}", "#".repeat(ROW_SIZE))
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Schematic {
    Lock(Lock),
    Key(Key),
}
impl Display for Schematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Schematic::Lock(l) => write!(f, "{}", l),
            Schematic::Key(k) => write!(f, "{}", k),
        }
    }
}

/// # Format
/// Locks and keys divided by `line_ending`.
/// The top for locks is all `#`, for keys it is `.`. At the bottom, it is the opposite.
/// This is how we can distinquish the data.
/// All values (schematics) are 5 characters wide (row length), 7 characters tall (column length).
/// The variable part (middle, between top and bottom) is only 5 high though.
/// We assume that the height of columns is the same as the amount of filled spots per column.
fn parse_locks_and_keys(input: &mut &str) -> PResult<(HashSet<Lock>, HashSet<Key>)> {
    separated(1.., parse_schematic, (line_ending, line_ending))
        .map(|schematics: HashSet<Schematic>| {
            schematics.into_iter().fold(
                (HashSet::new(), HashSet::new()),
                |(mut locks, mut keys), next| {
                    match next {
                        Schematic::Lock(l) => locks.insert(l),
                        Schematic::Key(k) => keys.insert(k),
                    };
                    (locks, keys)
                },
            )
        })
        .parse_next(input)
}

fn parse_schematic(input: &mut &str) -> PResult<Schematic> {
    alt((
        parse_lock.map(Schematic::Lock),
        parse_key.map(Schematic::Key),
    ))
    .parse_next(input)
}
fn parse_lock(input: &mut &str) -> PResult<Lock> {
    parse_from_top(input, &Spot::Filled)
        .map(|a: Vec<[Spot; ROW_SIZE]>| Lock(convert_schematic_fit(a)))
}
fn parse_key(input: &mut &str) -> PResult<Key> {
    parse_from_top(input, &Spot::Empty)
        .map(|a: Vec<[Spot; ROW_SIZE]>| Key(convert_schematic_fit(a)))
}

fn parse_from_top(input: &mut &str, top: &Spot) -> PResult<Vec<[Spot; ROW_SIZE]>> {
    delimited(
        terminated(
            parse_row.verify(|a: &[Spot; ROW_SIZE]| a.iter().all(|b| b == top)),
            line_ending,
        ),
        repeat(COLUMN_SIZE - 2, terminated(parse_row, line_ending)),
        parse_row.verify(|a: &[Spot; ROW_SIZE]| a.iter().all(|b| b != top)),
    )
    .parse_next(input)
}
fn convert_schematic_fit(a: Vec<[Spot; ROW_SIZE]>) -> SchematicFit {
    a.into_iter().fold([0; ROW_SIZE], |mut acc, next| {
        for (i, spot) in next.into_iter().enumerate() {
            if spot == Spot::Filled {
                acc[i] += 1;
            }
        }
        acc
    })
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spot {
    Filled,
    Empty,
}
fn parse_row(input: &mut &str) -> PResult<[Spot; ROW_SIZE]> {
    repeat(ROW_SIZE, parse_spot)
        .map(|v: Vec<_>| v.try_into().unwrap())
        .parse_next(input)
}
fn parse_spot(input: &mut &str) -> PResult<Spot> {
    alt(('#'.value(Spot::Filled), '.'.value(Spot::Empty))).parse_next(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_solvable() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 3);
    }

    //    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/25")), 3451);
    }
}
