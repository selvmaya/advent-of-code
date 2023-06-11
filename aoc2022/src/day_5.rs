use {
    crate::Runnable,
    core::fmt,
    itertools::Itertools,
    nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::digit1,
        combinator::{all_consuming, map, map_res},
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
        Finish, IResult,
    },
};

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        // println!(
        // "Crates on top of each stack after moves: {}",
        part_1_solve(input.as_str());
        // );
    }
}

#[derive(Clone, Copy)]
struct Crate(char);
impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}
fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}
fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i) // parse for crate, then try hole, otherwise fail
}
fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(tag(" "), parse_crate_or_hole)(i)
}

struct Piles(Vec<Vec<Crate>>);
impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}
impl Piles {
    #[allow(unused)]
    fn apply_1(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(el);
        }
    }
    fn apply_2(&mut self, ins: Instruction) {
        for krate in (0..ins.quantity)
            .map(|_| self.0[ins.src].pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            self.0[ins.dst].push(krate);
        }
    }
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i) // convert from 1-indexed to 0-indexed
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}
fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

fn part_1_solve(input: &str) {
    let mut lines = input.lines();

    let crate_lines: Vec<_> = lines
        .by_ref()
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let mut piles = Piles(transpose_rev(crate_lines));
    println!("{piles:?}");

    assert!(lines.next().unwrap().is_empty());

    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        println!("{ins:?}");
        piles.apply_2(ins);
        println!("{piles:?}");
    }

    println!(
        "answer = {}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    );
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
