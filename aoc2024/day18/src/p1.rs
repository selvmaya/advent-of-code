#![doc = include_str!("../p1.md")]

use common::*;
use std::collections::{hash_map, HashMap, VecDeque};
use winnow::Parser;

mod common;

pub const STANDARD_COUNT: usize = 1024;
fn main() {
    util::DayInput::find::<18>().solve_with(solve::<STANDARD_SIZE, STANDARD_COUNT>);
}

/// # Problem
/// From input we get some obstacles that may be placed on some 2D map.
/// The first 1024 obstacles are placed.
/// How many steps is the fastest route from the top left to the bottom right?
fn solve<const GRAPH_SIZE: GraphDistance, const BYTE_COUNT: usize>(input: impl AsRef<str>) -> u64 {
    let all_obstacles = parse_bytes
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    let graph = SetGraph::<GRAPH_SIZE>::from_obstacles::<BYTE_COUNT>(all_obstacles);
    eprintln!("Graph:\n{}", graph);
    graph.find_shortest_path_through()
}

trait ExactSearch {
    fn from_obstacles<const OBSTACLE_COUNT: usize>(
        all_obstacles: impl IntoIterator<Item = Pos>,
    ) -> Self;
    fn find_shortest_path_through(&self) -> u64;
}
impl<const SIZE: GraphDistance> ExactSearch for SetGraph<SIZE> {
    fn from_obstacles<const OBSTACLE_COUNT: usize>(
        all_obstacles: impl IntoIterator<Item = Pos>,
    ) -> Self {
        SetGraph(all_obstacles.into_iter().take(OBSTACLE_COUNT).collect())
    }

    fn find_shortest_path_through(&self) -> u64 {
        fn find_paths<const GRAPH_SIZE: GraphDistance>(
            graph: &SetGraph<GRAPH_SIZE>,
            start: Pos,
            goal: Pos,
        ) -> HashMap<Pos, Option<Pos>> {
            let mut frontier = VecDeque::from([start]);
            let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::from([(start, None)]);

            // create optimal path for every position
            while let Some(current) = frontier.pop_front() {
                if current == goal {
                    break;
                }

                for valid_neighboor in graph.neighboors(&current) {
                    if let hash_map::Entry::Vacant(new_entry) = came_from.entry(valid_neighboor) {
                        new_entry.insert(Some(current));
                        frontier.push_back(valid_neighboor);
                    }
                }
            }
            came_from
        }

        let start = Pos { x: 0, y: 0 };
        let goal = Pos { x: SIZE, y: SIZE };
        let came_from = find_paths(self, start, goal);

        let mut prev = &goal; // assume accessible
        let mut travel_distance = 0;
        while let Some(Some(next_backwards_pos)) = came_from.get(prev) {
            prev = next_backwards_pos;
            travel_distance += 1;
        }

        travel_distance
    }
}

#[cfg(test)]
mod tests {
    use crate::common::STANDARD_SIZE;
    use crate::STANDARD_COUNT;

    #[test]
    fn example_solvable() {
        // example exit is 6,6
        assert_eq!(super::solve::<6, 12>(include_str!("EXAMPLE")), 22);
    }

    #[test]
    fn input_solvable() {
        assert_eq!(
            super::solve::<STANDARD_SIZE, STANDARD_COUNT>(include_str!("../../inputs/18")),
            246
        );
    }
}
