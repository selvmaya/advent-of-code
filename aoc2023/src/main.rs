#![allow(unused)]

use itertools::Itertools;
use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::ops::Range;
use std::{fs, io};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
// mod day_12;
// mod day_13;
// mod day_14;
// mod day_15;
// mod day_16;
// mod day_17;
// mod day_18;
// mod day_19;
// mod day_20;
// mod day_21;
// mod day_22;
// mod day_23;
// mod day_24;
// mod day_25;

fn main() {
    let mut days: Vec<u8> = Vec::new();
    for arg in std::env::args() {
        if let Some((from, to)) = arg.split_once("..=") {
            if let Ok(range) = from.parse().and_then(|from| to.parse().map(|to| from..=to)) {
                for day in range {
                    days.push(day);
                }
            }
        } else if let Some((from, to)) = arg.split_once("..") {
            if let Ok(range) = from.parse().and_then(|from| to.parse().map(|to| from..to)) {
                for day in range {
                    days.push(day);
                }
            }
        } else if let Ok(day) = arg.parse() {
            days.push(day);
        }
    }
    for day in days.into_iter().sorted().dedup() {
        match day {
            26.. => eprintln!("{} is not a valid number for a day.", day),
            day => {
                if let Err(e) = run_day(day) {
                    eprintln!("Error when trying to run day {}: {}", day, e);
                }
            }
        }
    }
}

pub(crate) type SolverFn = fn(&str) -> String;

struct Solution {
    input: String,
}
impl Solution {
    fn run(&self, parts: &[SolverFn]) -> io::Result<()> {
        if parts.is_empty() {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Solution contained no solvers!",
            ))
        } else {
            for (index, part) in parts.iter().enumerate() {
                println!("SOLUTION PART {}:\n{}", index + 1, part(&self.input));
            }
            Ok(())
        }
    }
}

fn run_day(day: u8) -> io::Result<()> {
    Solution {
        input: fs::read_to_string(format!("input/day_{:0>2}.txt", day))?,
    }
    .run(match day {
        1 => day_01::PARTS,
        2 => day_02::PARTS,
        3 => day_03::PARTS,
        4 => day_04::PARTS,
        5 => day_05::PARTS,
        6 => day_06::PARTS,
        7 => day_07::PARTS,
        8 => day_08::PARTS,
        9 => day_09::PARTS,
        10 => day_10::PARTS,
        11 => day_11::PARTS,
        // 12 => day_12::PARTS,
        // 13 => day_13::PARTS,
        // 14 => day_14::PARTS,
        // 15 => day_15::PARTS,
        // 16 => day_16::PARTS,
        // 17 => day_17::PARTS,
        // 18 => day_18::PARTS,
        // 19 => day_19::PARTS,
        // 20 => day_20::PARTS,
        // 21 => day_21::PARTS,
        // 22 => day_22::PARTS,
        // 23 => day_23::PARTS,
        // 24 => day_24::PARTS,
        // 25 => day_25::PARTS,
        _ => {
            unreachable!("program was asked to solve a day that was not completed")
        }
    })
}