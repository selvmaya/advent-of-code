#![allow(dead_code)]

use std::collections::HashMap;

use camino::Utf8PathBuf;

use id_tree::{Tree, Node};
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
        part_2_solve(input);
    }
}

// find all directories with a size of less than or exactly 100_000, then calculate the sum of their sizes.
fn part_1_solve(input: &str) {
    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let (dir_tree, root_id) = construct_dir_tree(lines);

    // find all directories in tree with a size <=100_000, then sum them together
    let sum_of_small_ish_directories = dir_tree
        .traverse_pre_order(&root_id).expect("could not traverse tree?")
        .map(|dir_n| recursive_dir_size(&dir_tree, dir_n))
        .filter(|&size| size <= 100_000)
        .sum::<u64>();
    println!("Sum of directories under size 100_000: {sum_of_small_ish_directories}");
}

fn construct_dir_tree(lines: impl Iterator<Item = Line>) -> (Tree<Directory>, id_tree::NodeId) {
    let mut dir_tree: Tree<Directory> = Tree::new();
    let root_id = dir_tree.insert(Node::new(Directory { path: "/".into(), files: HashMap::new() }), id_tree::InsertBehavior::AsRoot).unwrap();
    let mut current_id = root_id.clone();
    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => { }, // ignore these
                Command::Cd(path) => match path.as_str() {
                    "/" => { // only happens once (first line)
                        current_id = root_id.clone(); // shouldn't really be necessary in this input?
                    },
                    ".." => { // move up one folder
                        current_id = dir_tree
                        .get(&current_id).expect("node exists in tree")
                        .parent().expect("node has a parent")
                            .clone();
                    },
                    _ => { // change directory/path to child directory
                        current_id = dir_tree.insert(
                            Node::new(Directory {
                                path: path.clone(),
                                files: HashMap::new()
                            }), id_tree::InsertBehavior::UnderNode(&current_id)
                        ).expect("insertion should work");
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => { }, // we create directories when we cd into them, no need to do it now
                Entry::File(file) => { // add file to current directory
                    let current_dir = dir_tree.get_mut(&current_id).expect("current directory exists").data_mut();
                    if let Some(_old_value) = current_dir.files.insert(file.path.clone(), file) {
                        panic!("Inserted file that already existed?");
                    }
                },
            },
        }
    }
    (dir_tree, root_id)
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

#[derive(Debug)]
struct Directory { path: Utf8PathBuf, files: HashMap<Utf8PathBuf, File> }
impl Directory {
    fn file_size_sum(&self) -> u64 {
        self.files.values().map(|file| file.size).sum()
    }
}
fn recursive_dir_size(tree: &Tree<Directory>, node: &Node<Directory>) -> u64 {
    let size_of_files_at_level = node.data().file_size_sum();
    let size_of_file_under_level = node.children()
    .iter()
    .map(|child_id| recursive_dir_size(tree, tree.get(child_id).expect("child did not exist?")))
    .sum::<u64>();
size_of_files_at_level + size_of_file_under_level
}

#[derive(Debug)]
enum Entry {
    Dir(Directory),
    File(File),
}
fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(File { path, size }),
    );
    let parse_dir = map(
        preceded(tag("dir "), parse_path),
        |path| Entry::Dir(Directory { path, files: HashMap::new() })
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

// find the smallest directory that would free up enough space for NEEDED_MIN_DISK_SPACE
fn part_2_solve(input: &str) {
    let lines = input
            .lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
        
    let (dir_tree, root_id) = construct_dir_tree(lines);
    
    const TOTAL_SPACE: u64 = 70_000_000;
    const NEEDED_FREE_SPACE: u64 = 30_000_000;
    let used_space = recursive_dir_size(&dir_tree, dir_tree.get(&root_id).expect("root missing?"));
    let free_space = TOTAL_SPACE.checked_sub(used_space).expect("used space exceeded total space?");
    let min_space_to_free = NEEDED_FREE_SPACE.checked_sub(free_space).expect("free space exceeded needed space?");

    let smallest_fit_dir_size = dir_tree
        .traverse_pre_order(&root_id).expect("could not traverse tree?")
        .map(|dir_n| recursive_dir_size(&dir_tree, dir_n))
        .filter(|&s| s >= min_space_to_free)
        .min().expect("No value found in iterator?");
    println!("Size of smallest directory that gives enough space for update: {smallest_fit_dir_size}");
}