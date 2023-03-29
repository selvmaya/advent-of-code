use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use camino::Utf8PathBuf;

use nom::{IResult, Finish};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{map, all_consuming};
use nom::sequence::{preceded, separated_pair};

use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        let input = input.as_str();
        part_1_solve(input);
    }
}

// find all directories with a size of less than or exactly 100_000, then calculate the sum of their sizes.
fn part_1_solve(input: &str) {
    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut root_dir = Directory { path: Utf8PathBuf::default(), parent_dir: None, sub_entries: HashMap::new() };
    let root_ref = Rc::new(RefCell::new(root_dir));
    let mut current_dir = &root_ref.clone();
    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => { }, // ignore these
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        current_dir = &root_ref.clone();
                    },
                    ".." => { // move up one folder
                        let parent: Rc<RefCell<Directory>> = current_dir.as_ref().borrow().parent_dir.clone().unwrap();
                        current_dir = &parent;
                    },
                    _ => { // change directory/path to child directory
                        let mut temp = current_dir.borrow_mut();
                        let child = temp.sub_entries
                            .entry(path.clone())
                            .or_insert(Entry::Dir(
                                Rc::new(RefCell::new(Directory { path, parent_dir: Some(current_dir.clone()), sub_entries: HashMap::new() }))
                            ));
                        let Entry::Dir(child_dir) = child else {
                            panic!("Newly created ('cd') entry was not directory?");
                        };
                        current_dir = child_dir;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => todo!(),
                Entry::File(_) => todo!(),
            }
        }
    }
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into
    )(i)
}

#[derive(Debug)]
struct Ls;
fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);
fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}
#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}
impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}
impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}
fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
struct File { path: Utf8PathBuf, size: u64 }

type DirHandle = Rc<RefCell<Directory>>;
#[derive(Debug)]
struct Directory { path: Utf8PathBuf, parent_dir: Option<DirHandle>, sub_entries: HashMap<Utf8PathBuf, Entry> }
impl Directory {
    fn total_size(&self) -> u64 {
        self.sub_entries.values().map(|entry| {
            match entry {
                Entry::Dir(sub_dir) => sub_dir.borrow().total_size(),
                Entry::File(file) => file.size,
            }
        }).sum()
    }
}
impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool { // comparison where we ignore files and assume they are correct
        self.path == other.path && self.parent_dir == other.parent_dir
    }
}

#[derive(Debug)]
enum Entry {
    Dir(DirHandle),
    File(File),
}
fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(File { path, size }),
    );
    let parse_dir = map(
        preceded(tag("dir "), parse_path),
        |path| Entry::Dir(Rc::new(RefCell::new(Directory { path, parent_dir: None, sub_entries: HashMap::new() })))
    );
    
    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}
fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

// unknown
fn part_2_solve(input: &str) {

}

#[cfg(test)]
mod tests {
    use super::*;
    mod part_1 {
        use super::*;
    }
    mod part_2 {
        use super::*;
    }
}