#![doc = include_str!("../p1.md")]

use itertools::Itertools;
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
    util::DayInput::find::<10>().solve_with(solve);
}

/// # Definitions
/// Hiking trails increment by exactly 1 every step. Thy start from 0 and go to 9.
/// Movements (steps) can only happen in cardinal directions (on the map)
/// Trailheads start one or more hiking trails, always at height 0.
/// A trailhead score is the amount of 9-height positions that can be reached.
/// *Note*: Not paths to take, but positions of 9-height it can reach.
/// # Input
/// The input of topographic maps is a rectangle of digits 0..=9, and there are no other symbols
/// (excluding newlines on the right / EOF), apart from the samples which have a `.` as an
/// impassable tile.
/// # Problem
/// What is the sum of all the trailhead scores?
/// # Solution
/// Since you can only go in one direction (up towards 9) there is a hard limit on amount of
/// steps any search will take, though there may be many. This is potentially a restriction
/// lifted in part 2?
/// An approach is to identify the position of every trail start and trail end (such that they
/// are distinct), then iterate from every start trail and go every possible cardinal direction
/// each step. The input isn't enormous, so this doesn't feel too infeasible.
fn solve(input: impl AsRef<str>) -> u64 {
    let map = parse_map.parse_next(&mut input.as_ref()).expect("parsable");
    debug_assert_eq!(
        input.as_ref().trim().lines().count() * map.width as usize,
        map.inner.len()
    );
    map.trailhead_score_sum()
}

struct TopographicMap {
    /// This should have a len of `width * height`.
    inner: Vec<Location>,
    width: u8, // The rectangle isn't that big.
}
impl TopographicMap {
    fn trailhead_score_sum(&self) -> u64 {
        self.inner
            .iter()
            .enumerate()
            .filter_map(|(pos, loc)| loc.0.and_then(|height| (height == 0).then_some(pos)))
            // all trailhead positions
            .map(|pos| {
                /// Finds the positions of every 9-height (top) position reachable,
                /// that is reachable from 0-height (the trailhead itself).
                /// The returned values are not ensured unique in of themselves.
                fn tops_from_trailhead(
                    map: &TopographicMap,
                    pos: usize,
                    height: u8,
                ) -> Box<dyn Iterator<Item = usize> + '_> {
                    if height == 9 {
                        Box::new(std::iter::once(pos))
                    } else {
                        let one_higher = height + 1;
                        let map_width = map.width as usize;
                        Box::new(
                            [
                                pos.checked_sub(map_width), // up
                                pos.checked_add(map_width), // down
                                pos.checked_sub(1) // left
                                        .filter(|new_pos| new_pos / map_width == pos / map_width),
                                pos.checked_add(1) // right
                                    .filter(|new_pos| new_pos / map_width == pos / map_width),
                            ]
                            .into_iter()
                            .flatten()
                            .filter(move |&potential_pos| {
                                match map.inner.get(potential_pos) {
                                    Some(Location(Some(potential_height))) => {
                                        // search in 1-higher trail path direction
                                        *potential_height == one_higher
                                    }
                                    _ => false,
                                }
                            })
                            .flat_map(move |valid_pos| {
                                tops_from_trailhead(map, valid_pos, one_higher)
                            }),
                        )
                    }
                }
                // find score of this trailhead
                tops_from_trailhead(self, pos, 0).unique().count() as u64
                // these are summed: trailheads may share tops
            })
            .sum::<u64>()
    }
}
/// In the problem this is called the "position"
struct Location(Option<u8>);
fn parse_map(input: &mut &str) -> PResult<TopographicMap> {
    separated(
        1..,
        parse_line
            .context(StrContext::Label("line of digits"))
            .verify(|line: &Vec<_>| !line.is_empty()),
        line_ending,
    )
    .verify(|lines: &Vec<_>| {
        lines
            .iter()
            .map(|line: &Vec<Location>| line.len())
            .all_equal()
    })
    .map(|lines: Vec<Vec<_>>| {
        let map_width = lines.first().unwrap().len() as u8;
        let inner = lines.into_iter().flatten().collect();
        TopographicMap {
            inner,
            width: map_width,
        }
    })
    .parse_next(input)
}
fn parse_line(input: &mut &str) -> PResult<Vec<Location>> {
    repeat(
        1..,
        parse_location.context(StrContext::Label("location height as digit")),
    )
    .parse_next(input)
}
fn parse_location(input: &mut &str) -> PResult<Location> {
    fn parse_digit(input: &mut &str) -> PResult<u8> {
        take(1u8).parse_to().parse_next(input)
    }
    alt((parse_digit.map(Some), '.'.value(None)))
        .map(Location)
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn trailhead_score_sum_examples() {
        assert_eq!(super::solve(include_str!("EXAMPLE_SMALL")), 1);
        assert_eq!(super::solve(include_str!("TRAILHEAD_2")), 2);
        assert_eq!(super::solve(include_str!("TRAILHEAD_1_2")), 3);
        assert_eq!(super::solve(include_str!("TRAILHEAD_4")), 4);
        assert_eq!(super::solve(include_str!("EXAMPLE_LARGE")), 36);
        assert_eq!(super::solve(include_str!("TRAILHEAD_6")), 6);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/10")), 607);
    }
}