pub(crate) use {part_1::part_1, part_2::part_2};

// #[cfg(test)]

mod part_1 {
    use itertools::Itertools;
    use std::num::Saturating;
    use std::ops::Add;
    use strum::{EnumIter, IntoEnumIterator};

    /// Parse as you go?
    /// Collect 2D grid of chars,
    /// Find starting point and go through pipes (cardinally connected) in each direction.
    pub(crate) fn part_1(input: &str) -> String {
        let lines = input.lines();
        let width = lines
            .clone()
            .map(|line| line.chars().count())
            .max()
            .unwrap();
        let height = lines.clone().count();
        let mut chars: Vec<char> = lines.flat_map(|line| line.chars()).collect();
        let starting_index = chars.iter().find_position(|&&c| c == 'S').unwrap().0;
        let (mut x, mut y) = (starting_index % width, starting_index / width);

        let mut grid_base = chars.as_mut_slice().chunks_mut(width).collect::<Vec<_>>();
        let grid = grid_base.as_mut_slice();

        assert_eq!(grid[y][x], 'S');

        #[derive(Debug, EnumIter, Eq, PartialEq, Copy, Clone)]
        enum Direction {
            North,
            South,
            West,
            East,
        }
        impl Direction {
            fn opposite(&self) -> Direction {
                match self {
                    Direction::North => Direction::South,
                    Direction::South => Direction::North,
                    Direction::West => Direction::East,
                    Direction::East => Direction::West,
                }
            }
            fn connections_for_char(c: &char) -> Vec<Direction> {
                match c {
                    'S' => Direction::iter().collect(),
                    '|' => vec![Self::North, Self::South],
                    '-' => vec![Self::East, Self::West],
                    'L' => vec![Self::North, Self::East],
                    'J' => vec![Self::North, Self::West],
                    '7' => vec![Self::South, Self::West],
                    'F' => vec![Self::South, Self::East],
                    '.' => vec![],
                    _ => unreachable!("Only the above characters should be handled."),
                }
            }
        }
        impl From<Direction> for (isize, isize) {
            fn from(value: Direction) -> Self {
                match value {
                    Direction::North => (0, -1),
                    Direction::South => (0, 1),
                    Direction::West => (0 - 1, 0),
                    Direction::East => (1, 0),
                }
            }
        }
        impl Add<(usize, usize)> for Direction {
            type Output = (usize, usize);
            fn add(self, rhs: (usize, usize)) -> Self::Output {
                let (x, y) = self.into();
                ((rhs.0 as isize + x) as usize, (rhs.1 as isize + y) as usize)
            }
        }

        fn pipe_connects_from(c: &char, direction: Direction) -> bool {
            Direction::connections_for_char(c).contains(&direction)
        }

        let mut pipe_length = 0;
        let mut prev: Option<Direction> = None;
        while grid[y][x] != 'S' || pipe_length == 0 {
            let c = grid[y][x];
            let directions_from_char = Direction::connections_for_char(&c);
            let directions_allowed_by_state = directions_from_char
                .iter()
                .filter(|dir| prev != Some(dir.opposite())) // don't go back
                .collect::<Vec<_>>();
            // eprintln!(
            //     "Of {:?} possible from '{}', only {:?} allowed...",
            //     directions_from_char, c, directions_allowed_by_state
            // );
            'update_pos: for dir in directions_allowed_by_state {
                let (next_x, next_y) = dir.add((x, y));
                let next_c = grid[next_y][next_x];
                if pipe_connects_from(&next_c, dir.opposite()) {
                    // eprintln!(
                    //     "Found valid connection {:?} at {:?}: char '{}'",
                    //     dir,
                    //     (next_x, next_y),
                    //     next_c
                    // );
                    prev = Some(*dir);
                    x = next_x;
                    y = next_y;
                    break 'update_pos;
                }
            }
            pipe_length += 1;
        }
        (pipe_length / 2).to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const SQUARE_LOOP: &str = r".....
.S-7.
.|.|.
.L-J.
.....";
        const MESSY_SQUARE_LOOP: &str = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        const COMPLEX_LOOP: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        #[test]
        fn simple_square_works() {
            assert_eq!(part_1(SQUARE_LOOP), "4", "Square loop is 8 long.");
        }

        #[test]
        fn messy_square_works() {
            assert_eq!(part_1(MESSY_SQUARE_LOOP), "4", "Square loop is 8 long.");
        }

        #[test]
        fn complex_works() {
            assert_eq!(part_1(COMPLEX_LOOP), "8", "Complex loop is 16 long.");
        }
    }
}

mod part_2 {
    pub(crate) fn part_2(input: &str) -> String {
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}