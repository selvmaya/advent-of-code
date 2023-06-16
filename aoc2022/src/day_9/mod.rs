use std::cmp;
use std::fmt::Display;

use itertools::Itertools;

use crate::Runnable;
pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        let input = input.as_str();
        println!("PART 1: {}", part_1_solve(input));
        println!("PART 2: {}", part_2_solve(input));
    }
}

/// imagine/simulate head of rope moving on 2D grid, and we can determine how the tail of the rope will move
///
/// rope is short: head and tail of rope will "always be touching" (diagonals and overlaps count) on the grid
///
/// tail must be touching, so if head moves in a way that would force a diagonal for tail, it will do it,
/// while head only moves cardinally in its own right
///
/// the movement of the rope's *head* is the puzzle input
///
/// after simulating the rope, "you can count up all the positions the *tail* visited atleast once"
///
/// SOLVE: how *many* positions does the *tail* of the rope visit *atleast once*?
fn part_1_solve(input: &str) -> usize {
    let series_of_head_motions = input
        .lines()
        .map(|l| Motion::try_from(l.trim()).expect("unable to create motion from line input"));
    // println!("Motions: [{}]", series_of_head_motions.clone().join(", "));
    let mut record: Vec<Step> = vec![Step {
        head: Head { pos: STARTING_POS },
        tail: Tail { pos: STARTING_POS }, /* dir: Direction::Right, previous: Vec::new() */
    }];
    for motion in series_of_head_motions {
        for _ in 0..motion.repeats {
            record.push(record.last().unwrap().after_dir(&motion.dir));
        }
    }
    // println!("--- RECORD ---\n{}", record.iter().join("\n\n---\n"));
    record.iter().map(|step| &step.tail.pos).unique().count()
}
#[derive(PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => 'L',
                Direction::Right => 'R',
                Direction::Up => 'U',
                Direction::Down => 'D',
            }
        )
    }
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'L' => Self::Left,
            'R' => Self::Right,
            'U' => Self::Up,
            'D' => Self::Down,
            _ => return Err("passed char could not be converted to direction"),
        })
    }
}
const STARTING_POS: Position = Position { x: 0, y: 0 };
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    fn after_dir(&self, dir: &Direction) -> Self {
        Self {
            x: {
                match dir {
                    Direction::Left => self.x - 1,
                    Direction::Right => self.x + 1,
                    _ => self.x,
                }
            },
            y: {
                match dir {
                    Direction::Up => self.y + 1,
                    Direction::Down => self.y - 1,
                    _ => self.y,
                }
            },
        }
    }
    /// diagonals are counted
    fn is_next_to(&self, other: &Position) -> bool {
        other.x.abs_diff(self.x) < 2 && other.y.abs_diff(self.y) < 2
    }
}
struct Motion {
    dir: Direction,
    repeats: usize,
}
impl Display for Motion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} : {})", self.dir, self.repeats)
    }
}
impl TryFrom<&str> for Motion {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (dir, repeats) = s
            .split_once(' ')
            .ok_or(format!("'{}' did not match expected format of 3 chars.", s))?;
        if dir.chars().count() != 1 {
            return Err(format!("'{}' had more than one char", dir));
        }
        let dir = dir.chars().next().unwrap();
        Ok(Motion {
            dir: Direction::try_from(dir)?,
            repeats: repeats.parse().map_err(|_| "could not parse str to u32")?,
        })
    }
}
#[derive(Debug, Clone)]
struct Head {
    pos: Position,
}
impl Head {
    fn moved_dir(&self, dir: &Direction) -> Self {
        Self {
            pos: self.pos.after_dir(dir),
        }
    }
}
#[derive(Debug, Clone)]
struct Tail {
    pos: Position,
}
impl Tail {
    fn moved_step_to_head(&self, head: &Head, head_prev_move_dir: &Direction) -> Self {
        if self.pos.is_next_to(&head.pos) {
            return self.clone();
        }
        let prev_head_pos = head.pos.after_dir(&head_prev_move_dir.opposite());
        Self { pos: prev_head_pos }
    }
}
#[derive(Clone)]
struct Step {
    head: Head,
    tail: Tail, /* dir: Direction, previous: Vec<Tail> */
}
impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = cmp::min(
            STARTING_POS.x,
            cmp::min(self.head.pos.x, self.tail.pos.x) - 1,
        );
        let max_x = cmp::max(
            STARTING_POS.x,
            cmp::max(self.head.pos.x, self.tail.pos.x) + 1,
        );
        let min_y = cmp::min(
            STARTING_POS.y,
            cmp::min(self.head.pos.y, self.tail.pos.y) - 1,
        );
        let max_y = cmp::max(
            STARTING_POS.y,
            cmp::max(self.head.pos.y, self.tail.pos.y) + 1,
        );
        let mut lines: Vec<String> = vec![];
        for y in min_y..=max_y {
            let mut line = vec![];
            for x in min_x..=max_x {
                let pos = Position { x, y };
                line.push(match pos {
                    _ if pos == self.head.pos => 'H',
                    _ if pos == self.tail.pos => 'T',
                    _ if pos == STARTING_POS => 's',
                    // _ if self.previous.iter().any(|tail| tail.pos == pos) => '#',
                    _ => '.',
                })
            }
            lines.push(line.iter().collect());
        }
        let print = lines.join("\n");
        // let dir_display = self.dir.to_string();
        // let print = [dir_display, print].join("\n\n");
        write!(f, "{}", print)
    }
}
impl Step {
    fn after_dir(&self, dir: &Direction) -> Self {
        let head = self.head.moved_dir(dir);
        // let mut current = self.previous.clone();
        // current.push(self.tail.clone());
        Self {
            tail: self.tail.moved_step_to_head(&head, dir),
            head,
            // dir: dir.clone(),
            // previous: current
        }
    }
}

/// SOLVE: How many positions does the tail of a rope *of length 10* visit *atleast* once?
fn part_2_solve(input: &str) -> usize {
    let motions = input
        .lines()
        .map(|l| Motion::try_from(l).expect("could not parse line as_str motion"));

    let mut snake = LinkedTail::spawn_recursive(9);
    let mut tails_positions: Vec<Vec<Position>> = vec![];
    for motion in motions {
        for _ in 0..motion.repeats {
            let new_head_pos = snake.pos.after_dir(&motion.dir);
            snake.move_recursive(&new_head_pos);
            println!("\n\n--- SNAKE ---\n\n{}", snake);

            let new_positions = snake
                .collection_recursive()
                .iter()
                .map(|lt| lt.pos.clone())
                .collect();
            tails_positions.push(new_positions);
        }
    }

    let mut tail_histories: Vec<Vec<&Position>> = vec![];
    for e in 0..tails_positions.first().unwrap().len() {
        let mut tail_history: Vec<&Position> = vec![];
        for time_slice in &tails_positions {
            let piece_of_tails_history = &time_slice[e];
            tail_history.push(piece_of_tails_history);
        }
        tail_histories.push(tail_history);
    }

    for history in tail_histories.clone() {
        println!("POSITIONS: {}", history.iter().unique().count());
    }

    tail_histories
        .last()
        .expect("a last tail")
        .iter()
        .unique()
        .count()
}
struct LinkedTail {
    pos: Position,
    next: Option<Box<LinkedTail>>,
}
impl Display for LinkedTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tails = self.collection_recursive();
        let positions: Vec<&Position> = tails.into_iter().map(|t| &t.pos).collect();
        let min_x = cmp::min(0, positions.iter().map(|pos| pos.x).min().unwrap() - 1);
        let max_x = cmp::max(0, positions.iter().map(|pos| pos.x).max().unwrap() + 1);
        let min_y = cmp::min(0, positions.iter().map(|pos| pos.y).min().unwrap() - 1);
        let max_y = cmp::max(0, positions.iter().map(|pos| pos.y).max().unwrap() + 1);
        let mut lines: Vec<String> = vec![];
        for y in min_y..=max_y {
            let mut line = vec![];
            for x in min_x..=max_x {
                let current_pos = Position { x, y };
                line.push(match &current_pos {
                    pos if pos == *positions.first().unwrap() => 'T',
                    pos if pos == *positions.last().unwrap() => 'H',
                    pos if positions.contains(&pos) => 'X',
                    pos if pos == &STARTING_POS => 's',
                    // _ if self.previous.iter().any(|tail| tail.pos == pos) => '#',
                    _ => '.',
                })
            }
            lines.push(line.iter().collect());
        }
        let print = lines.join("\n");
        write!(f, "{}", print)
    }
}
impl LinkedTail {
    fn spawn_recursive(length_remaining: usize) -> LinkedTail {
        if length_remaining == 0 {
            return LinkedTail {
                pos: STARTING_POS,
                next: None,
            };
        }
        LinkedTail {
            pos: STARTING_POS,
            next: Some(Box::new(LinkedTail::spawn_recursive(length_remaining - 1))),
        }
    }
    // fn bottom_value(&self) -> &LinkedTail {
    //     match &self.next {
    //         Some(n) => n.bottom_value(),
    //         None => self,
    //     }
    // }
    fn move_recursive(&mut self, new_pos: &Position) {
        if &self.pos == new_pos {
            return; // no need to move to a position we are already at
        }
        if let Some(n) = self.next.as_deref_mut() {
            if !n.pos.is_next_to(new_pos) {
                n.move_recursive(&self.pos);
            }
        }
        self.pos = new_pos.clone();
    }
    fn collection_recursive(&self) -> Vec<&LinkedTail> {
        let mut vec = match self.next.as_ref() {
            Some(n) => n.collection_recursive(),
            None => Vec::new(),
        };
        vec.push(self);
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_1: &str = include_str!("sample_1.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(13, part_1_solve(TEST_INPUT_1));
    }
    #[test]
    fn next_to_works() {
        const POS_1: Position = Position { x: 1, y: 0 };
        const POS_2: Position = Position { x: 0, y: 1 };
        const POS_3: Position = Position { x: 1, y: 1 };
        const POS_4: Position = Position { x: -1, y: -1 };
        assert!(STARTING_POS.is_next_to(&POS_1));
        assert!(STARTING_POS.is_next_to(&POS_2));
        assert!(STARTING_POS.is_next_to(&POS_3));
        assert!(STARTING_POS.is_next_to(&POS_4));

        assert!(POS_1.is_next_to(&POS_2));
        assert!(POS_1.is_next_to(&POS_3));
        assert!(!POS_3.is_next_to(&POS_4));

        const POS_5: Position = Position { x: 2, y: 0 };
        const POS_6: Position = Position { x: 0, y: 2 };
        const POS_7: Position = Position { x: 2, y: 1 };
        const POS_8: Position = Position { x: 2, y: 2 };
        const POS_9: Position = Position { x: -2, y: -2 };
        assert!(!STARTING_POS.is_next_to(&POS_5));
        assert!(!STARTING_POS.is_next_to(&POS_6));
        assert!(!STARTING_POS.is_next_to(&POS_7));
        assert!(!STARTING_POS.is_next_to(&POS_8));
        assert!(!STARTING_POS.is_next_to(&POS_9));

        assert!(POS_1.is_next_to(&POS_5));
    }

    const TEST_INPUT_2: &str = include_str!("sample_2.txt");
    #[test]
    fn part_2_test() {
        assert_eq!(36, part_2_solve(TEST_INPUT_2));
    }
}
