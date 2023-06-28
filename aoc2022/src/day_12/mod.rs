use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
	fn run_with_input(&self, input: String) {
		println!("PART 1: {}", part_1_solve(&input));
		// println!("PART 2: {}", part_2_solve(&input));
	}
}
/// Puzzle input is a heightmap, where heights are given by characters "a" to
/// "z", where "z" is the highest.
/// Also included is current location on map, indicated by character "S",
/// and the location with the best signal "E".
/// Current position "S" has elevation "a", best signal "E" has elevation "z".
///
/// To reach the best location "E", we can move one character/square up, down
/// left or right. We can however only move one up per square, eg. "a" to "b",
/// but we can drop more than one if needed, eg. "g" to "b".
///
/// SOLVE: Fewest steps required to move from current position to the location
/// with the best signal
fn part_1_solve(input: &str) -> usize {
	let heightmap = Heightmap::from(input);

	naive_maze_solution(heightmap).len()
}
type Path = Vec<StepDirection>;
fn naive_maze_solution(heightmap: Heightmap) -> Path {
	let starting_pos = heightmap.starting_pos;

	let mut path = Path::new();
	let mut current_pos = starting_pos;
	let mut current = &heightmap.map[starting_pos.y][starting_pos.x];
	'path: loop {
		for dir in StepDirection::all_dirs() {
			let new_pos = current_pos.after_step(&dir);
			let new = &heightmap.map[new_pos.y][new_pos.x];
			if new.height() > current.height() {
				path.push(dir);
				if new.0 == 'E' {
					break 'path;
				}
				current_pos = new_pos;
				current = new;
				continue 'path;
			}
		}
		panic!("No direction found! Position: {:?}", current_pos);
	}
	path
}
enum StepDirection {
	Up,
	Down,
	Left,
	Right
}
impl StepDirection {
	const fn all_dirs() -> [Self; 4] { [Self::Up, Self::Down, Self::Left, Self::Right] }
}
#[derive(PartialEq, PartialOrd)]
struct CharPosition(char);
impl CharPosition {
	fn height(&self) -> u8 {
		const OFFSET: u8 = b'a';
		(match self.0 {
			'S' => 'a',
			'E' => 'z',
			c => c
		} as u8) - OFFSET
	}
}
#[derive(Default, Clone, Copy, Debug)]
struct Position2D {
	x: usize,
	y: usize
}
impl Position2D {
	fn after_step(&self, dir: &StepDirection) -> Self {
		let mut x: isize = self.x as isize;
		let mut y: isize = self.y as isize;
		match dir {
			StepDirection::Up => y += 1,
			StepDirection::Down => y -= 1,
			StepDirection::Left => x -= 1,
			StepDirection::Right => x += 1
		}
		if x < 0 {
			x = 0;
			println!("Forced 'x' to be zero.");
		} else if y < 0 {
			y = 0;
			println!("Forced 'y' to be zero.");
		}
		Position2D {
			x: x as usize,
			y: y as usize
		}
	}
}
struct Heightmap {
	map:           Vec<Vec<CharPosition>>, // 2D map
	starting_pos:  Position2D,
	_best_location: Position2D
}
impl From<&str> for Heightmap {
	fn from(s: &str) -> Self {
		let mut starting_pos = Position2D::default();
		let mut best_location = Position2D::default();
		Heightmap {
			map: {
				let mut lines_of_positions = vec![];
				for (y, line) in s.lines().enumerate() {
					let mut positions = vec![];
					for (x, c) in line.chars().enumerate() {
						if c == 'S' {
							starting_pos = Position2D { x, y };
						} else if c == 'E' {
							best_location = Position2D { x, y };
						}
						positions.push(CharPosition(c));
					}
					lines_of_positions.push(positions);
				}
				lines_of_positions
			},
			starting_pos,
			_best_location: best_location
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::day_12::part_1_solve;

	const SAMPLE_1: &str = include_str!("sample_1.txt");
	#[test]
	fn part_1_test() {
		assert_eq!(part_1_solve(SAMPLE_1), 31);
	}
}
