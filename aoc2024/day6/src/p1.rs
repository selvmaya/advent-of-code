#![doc = include_str!("../p1.md")]

use std::{
	collections::HashSet,
	fmt::{Display, Formatter},
	str::FromStr
};

#[allow(unused_imports)]
use winnow::{
	ascii::*,
	combinator::*,
	error::*,
	stream::AsChar,
	token::*,
	{PResult, Parser}
};

fn main() {
	util::DayInput::find::<6>().solve_with(solve);
}

/// # Problem
/// How many distinct (unique) positions will the guard visit before leaving the
/// map? # Solution
/// Simulate movement until the guard leaves the map. Keep track of every spot.
/// Give back the amount.
/// ## Simulation
/// Guard begins somewhere, with a starting direction.
/// We can get position and direction while parsing.
/// While parsing, we also get positions of every obstacle (`#`).
fn solve(input: impl AsRef<str>) -> u64 {
	let sim = input.as_ref().parse::<SimulationState>().expect("parsable");
	sim.finish(HashSet::new()).visited.len() as u64
}

struct SimulationState {
	guard:     Guard,
	obstacles: HashSet<Position>,
	bounds:    MapBounds
}
impl Display for SimulationState {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for y in 0..(self.bounds.height as i64) {
			for x in 0..(self.bounds.width as i64) {
				let pos = Position { x, y };
				write!(
					f,
					"{}",
					if self.guard.pos == pos {
						(&self.guard.dir).into()
					} else if self.obstacles.contains(&pos) {
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
impl SimulationState {
	fn finish(mut self, mut visited: HashSet<Position>) -> CompletedSimulation {
		self.step_till_completed(&mut visited);
		CompletedSimulation { sim: self, visited }
	}

	fn step_till_completed(&mut self, visited: &mut HashSet<Position>) {
		while self.guard.pos.is_inside(&self.bounds) {
			self.step_and_or_turn(visited);
		}
	}

	#[allow(unused)]
	#[cfg(test)]
	fn walk_to_next_collision(&mut self, visited: &mut HashSet<Position>) {
		let _ = self.turn_till_free();
		while self.free_step(visited) {}
	}

	#[allow(unused)]
	#[cfg(test)]
	fn free_step(&mut self, visited: &mut HashSet<Position>) -> bool {
		let possible_new_pos = self.guard.pos.move_in(&self.guard.dir);
		if self.obstacles.contains(&possible_new_pos) || !possible_new_pos.is_inside(&self.bounds) {
			false
		} else {
			self.guard.pos = possible_new_pos;
			visited.insert(self.guard.pos);
			true
		}
	}

	fn step_and_or_turn(&mut self, visited: &mut HashSet<Position>) {
		let possible_new_pos = self.turn_till_free();
		self.guard.pos = possible_new_pos;
		if self.guard.pos.is_inside(&self.bounds) {
			visited.insert(self.guard.pos);
		}
	}

	fn turn_till_free(&mut self) -> Position {
		let mut possible_new_pos;
		while {
			// try facing direction
			possible_new_pos = self.guard.pos.move_in(&self.guard.dir);
			self.obstacles.contains(&possible_new_pos)
		} {
			// face new direction
			self.guard.dir = self.guard.dir.next();
		}
		possible_new_pos
	}
}
struct Guard {
	pos: Position,
	dir: Direction
}
#[derive(Debug, Clone)]
struct MapBounds {
	width:  usize,
	height: usize
}

#[derive(Clone)]
enum MapItem {
	Empty,
	Guard(Direction),
	Obstacle
}
impl MapItem {
	fn parser(s: &mut &str) -> PResult<Self> {
		alt((
			'.'.value(Self::Empty),
			'#'.value(Self::Obstacle),
			Direction::parser.map(Self::Guard)
		))
		.parse_next(s)
	}
}
impl FromStr for SimulationState {
	type Err = ErrMode<ContextError>;

	fn from_str(mut s: &str) -> Result<Self, Self::Err> {
		let bounds = MapBounds {
			width:  s.lines().next().expect("first line").chars().count(),
			height: s.lines().count()
		};
		parse_content
			.parse_next(&mut s)
			.map(|(guard, obstacles)| SimulationState {
				guard,
				obstacles,
				bounds
			})
	}
}
fn parse_content(input: &mut &str) -> PResult<(Guard, HashSet<Position>)> {
	let all_lines: Vec<_> = separated(1.., parse_line, line_ending).parse_next(input)?;
	let (guard, obstacles) = all_lines
		.into_iter()
		.enumerate()
		.flat_map(|(y, line)| {
			line.into_iter()
				.enumerate()
				.map(move |(x, item)| ((x, y), item))
		})
		.fold(
			(None, HashSet::new()),
			|(mut guard, mut obstacles), ((x, y), item)| {
				match item {
					MapItem::Empty => {},
					MapItem::Guard(dir) => {
						guard = Some(Guard {
							pos: Position {
								x: x as i64,
								y: y as i64
							},
							dir
						})
					},
					MapItem::Obstacle => {
						if !obstacles.insert(Position {
							x: x as i64,
							y: y as i64
						}) {
							panic!("it should be impossible for this to exist")
						}
					}
				};
				(guard, obstacles)
			}
		);
	Ok((guard.expect("there should be a guard somewhere"), obstacles))
}
fn parse_line(input: &mut &str) -> PResult<Vec<MapItem>> {
	repeat(1.., MapItem::parser).parse_next(input)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
	x: i64,
	y: i64
}
impl Display for Position {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "({},{})", self.x, self.y)
	}
}
impl Position {
	fn is_inside(&self, bounds: &MapBounds) -> bool {
		(0..bounds.width as i64).contains(&self.x) && (0..bounds.height as i64).contains(&self.y)
	}

	fn move_in(&self, dir: &Direction) -> Self {
		match dir {
			// notice that positions are upside down for line counting reasons
			Direction::Up => Self {
				x: self.x,
				y: self.y - 1
			},
			Direction::Down => Self {
				x: self.x,
				y: self.y + 1
			},
			Direction::Left => Self {
				x: self.x - 1,
				y: self.y
			},
			Direction::Right => Self {
				x: self.x + 1,
				y: self.y
			}
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}
impl Display for Direction {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", char::from(self))
	}
}
impl From<&Direction> for char {
	fn from(dir: &Direction) -> Self {
		match dir {
			Direction::Up => '^',
			Direction::Down => 'v',
			Direction::Left => '<',
			Direction::Right => '>'
		}
	}
}
impl Direction {
	fn parser(s: &mut &str) -> PResult<Direction> {
		alt((
			'^'.value(Direction::Up),
			'>'.value(Direction::Right),
			'v'.value(Direction::Down),
			'<'.value(Direction::Left)
		))
		.parse_next(s)
	}

	fn next(&self) -> Direction {
		match self {
			// always turn clockwise
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up
		}
	}
}

struct CompletedSimulation {
	sim:     SimulationState,
	visited: HashSet<Position>
}
impl Display for CompletedSimulation {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for y in 0..(self.sim.bounds.height as i64) {
			for x in 0..(self.sim.bounds.width as i64) {
				let pos = Position { x, y };
				write!(
					f,
					"{}",
					if self.sim.guard.pos == pos {
						(&self.sim.guard.dir).into()
					} else if self.sim.obstacles.contains(&pos) {
						'#'
					} else if self.visited.contains(&pos) {
						'X'
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

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	// start somewhere in the middle
	const EXAMPLE_0: &str = include_str!("EXAMPLE_0");
	// hit first obstacle
	const EXAMPLE_1: &str = include_str!("EXAMPLE_1");
	const EXAMPLE_2: &str = include_str!("EXAMPLE_2");
	const EXAMPLE_3: &str = include_str!("EXAMPLE_3");
	// this one is multiple steps ahead, unlike the first four
	const EXAMPLE_4: &str = include_str!("EXAMPLE_4");
	// this one is filled in
	const EXAMPLE_5: &str = include_str!("EXAMPLE_5");

	// noinspection RsAssertEqual
	#[test]
	fn guard_movements_follow() {
		let mut sim = EXAMPLE_0.parse::<super::SimulationState>().unwrap();
		{
			let unparsed_initial = sim.to_string();
			eprintln!("Initial unparsed state:\n{}", unparsed_initial);
			assert!(
				unparsed_initial.trim() == EXAMPLE_0.trim(),
				"Parsing and unparsing should restore text.\nOriginal:\n{}",
				EXAMPLE_0
			);
			eprintln!("This is correct!\n");
		}
		let mut visited = HashSet::new();
		for next_step_example in [EXAMPLE_1, EXAMPLE_2, EXAMPLE_3] {
			sim.walk_to_next_collision(&mut visited);
			let unparsed = sim.to_string();
			eprintln!("Moving forward once:\n{}", unparsed);
			assert!(
				unparsed.trim() == next_step_example.trim(),
				"Stepping the simulation should follow examples.\nOriginal:\n{}",
				next_step_example
			);
			eprintln!("This is correct!\n");
		}

		for step in 1..=8 {
			sim.walk_to_next_collision(&mut visited);
			let unparsed_step = sim.to_string();
			eprintln!("Moving forward, step {}:\n{}", step, unparsed_step);
		}
		{
			let unparsed_4 = sim.to_string();
			assert!(
				unparsed_4.trim() == EXAMPLE_4.trim(),
				"After some moves this should be the same as example 4.\nOriginal (example 4):\n{}",
				EXAMPLE_4
			);
			eprintln!("This is correct!\n");
		}

		let completed = sim.finish(visited);
		let unparsed_completed = completed.to_string();
		eprintln!("Moved to end:\n{}", unparsed_completed);
		assert!(
			unparsed_completed.trim() == EXAMPLE_5.trim(),
			"After some moves this should be the same as example 4.\nOriginal (example 4):\n{}",
			EXAMPLE_5
		);
	}

	#[test]
	fn example_solvable() {
		assert_eq!(super::solve(EXAMPLE_0), 41);
	}

	#[test]
	fn input_solvable() {
		const INPUT: &str = include_str!("../../inputs/6");
		assert_eq!(super::solve(INPUT), 4964);
	}
}
