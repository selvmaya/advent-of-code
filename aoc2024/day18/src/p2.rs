#![doc = include_str!("../p2.md")]

mod common;

use common::*;
use itertools::{FoldWhile, Itertools};
use std::collections::{HashSet, VecDeque};
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
    util::DayInput::find::<18>().solve_with(solve::<STANDARD_SIZE>);
}

fn solve<const GRAPH_SIZE: GraphDistance>(input: impl AsRef<str>) -> Pos {
    let obstacles = parse_bytes
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    let graph = UnfinalizedGraph::<GRAPH_SIZE> {
        all_obstacles: obstacles,
    };
    graph
        .find_last_placed_before_impassable()
        .expect("some place")
}

struct UnfinalizedGraph<const SIZE: GraphDistance> {
    all_obstacles: Vec<Pos>,
}
impl<const SIZE: GraphDistance> UnfinalizedGraph<SIZE> {
    fn find_last_placed_before_impassable(&self) -> Option<Pos> {
        /// BFS, early exit (when finding goal)
        fn can_find_goal_with_obstacles<const GRAPH_SIZE: GraphDistance>(
            graph: &SetGraph<GRAPH_SIZE>,
            start: Pos,
            goal: Pos,
        ) -> bool {
            let mut frontier = VecDeque::from([start]);
            let mut visited = HashSet::from([start]);
            while let Some(current) = frontier.pop_front() {
                if current == goal {
                    return true;
                }

                for valid_neighboor in graph.neighboors(&current) {
                    // is this a new spot?
                    if visited.insert(valid_neighboor) {
                        frontier.push_back(valid_neighboor);
                    }
                }
            }
            false
        }

        let start = Pos { x: 0, y: 0 };
        let goal = Pos { x: SIZE, y: SIZE };
        let (first_70, remaining) = self.all_obstacles.split_at(SIZE as usize);
        let (first_obstacle_blocking, _) = remaining
            .iter()
            .fold_while(
                (None, SetGraph(HashSet::from_iter(first_70.iter().copied()))),
                |(_, mut obstacles), next /* beginning with the 70th */| {
                    let did_not_contain = obstacles.0.insert(*next);
                    debug_assert!(did_not_contain);
                    if can_find_goal_with_obstacles::<SIZE>(&obstacles, start, goal) {
                        FoldWhile::Continue((None, obstacles))
                    } else {
                        FoldWhile::Done((Some(*next), obstacles))
                    }
                },
            )
            .into_inner();
        first_obstacle_blocking
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_solvable() {
        assert_eq!(solve::<6>(include_str!("EXAMPLE")).to_string(), "6,1");
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(
            solve::<STANDARD_SIZE>(include_str!("../../inputs/18")).to_string(),
            "22,50"
        );
    }
}
