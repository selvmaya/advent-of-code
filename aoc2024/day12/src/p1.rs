#![doc = include_str!("../p1.md")]

use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
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
    util::DayInput::find::<12>().solve_with(solve);
}

/// # Problem
/// Divide garden map of flowers into regions,
/// where every flower (ASCII character) connects to another if they are of the same type,
/// and are next to each other cardinally.
/// The sum of each region's area multiplied by its perimeter is the total (result).
/// ## Definitions
/// The area of regions is equal to their amount of flowers (characters).
/// The perimeter is the sum of the edge counts from the outer edge and the inner edge(s).
/// The outside of the input is also an edge, not just between different regions.
/// # Solution
/// Divide into regions of positions, then calculate perimeters & areas to sum them.
fn solve(input: impl AsRef<str>) -> u64 {
    let garden = parse_garden
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    debug_assert_ne!(garden.width, 0);
    debug_assert_ne!(garden.inner.len(), 0);
    eprintln!(
        "Garden map has a width of {} and a height of {}, with {} flowers in total.",
        garden.width,
        garden.inner.len() / garden.width,
        garden.inner.len(),
    );
    eprintln!("{}", garden);
    garden
        .get_regions()
        .into_iter()
        .map(|region| {
            let area = region.positions.len() as u64;
            let perimeter = region
                .positions
                .iter()
                .map(|pos| {
                    // amount of edges: `max` - `self-similar neighbors`
                    4 - garden
                        .bordering_pos(pos)
                        .filter(|bordering_pos| {
                            garden
                                .inner
                                .get(bordering_pos.0)
                                .is_some_and(|&flower| flower == region.flower)
                        })
                        .count()
                })
                .sum::<usize>() as u64;
            eprintln!(
                "Region of {} has an area of {} and a perimeter of {}:\n{}",
                region.flower.0,
                area,
                perimeter,
                region.relative_to(&garden)
            );
            area * perimeter
        })
        .sum()
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position(usize);
impl Position {
    fn relative_to<'a>(&'a self, garden: &'a Garden) -> RelativePosition<'a> {
        RelativePosition { pos: self, garden }
    }
}
struct RelativePosition<'a> {
    pos: &'a Position,
    garden: &'a Garden,
}
#[expect(clippy::needless_lifetimes)]
impl<'a> Display for RelativePosition<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}x,{}y)",
            //            self.pos.0,
            self.pos.0 % self.garden.width,
            self.pos.0 / self.garden.width
        )
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Flower(char);
#[derive(Clone)]
struct Region {
    flower: Flower,
    positions: HashSet<Position>,
}
impl Region {
    fn relative_to<'a>(&'a self, garden: &'a Garden) -> RelativeRegion<'a> {
        RelativeRegion {
            region: self,
            garden,
        }
    }
    fn changing_to<'a>(&'a self, other: &'a Self, garden: &'a Garden) -> RegionChange<'a> {
        RegionChange {
            a: self,
            b: other,
            garden,
        }
    }
}
struct RelativeRegion<'a> {
    region: &'a Region,
    garden: &'a Garden,
}
#[expect(clippy::needless_lifetimes)]
impl<'a> RelativeRegion<'a> {
    fn horizontal_span(&self) -> RangeInclusive<usize> {
        let (leftmost, rightmost) = self
            .region
            .positions
            .iter()
            .map(|pos| pos.0 % self.garden.width)
            .minmax()
            .into_option()
            .expect("some elements");
        leftmost..=rightmost
    }
    fn vertical_span(&self) -> RangeInclusive<usize> {
        let (topmost, bottommost) = self
            .region
            .positions
            .iter()
            .map(|pos| pos.0 / self.garden.width)
            .minmax()
            .into_option()
            .expect("some elements");
        topmost..=bottommost
    }
}
#[expect(clippy::needless_lifetimes)]
impl<'a> Display for RelativeRegion<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rel_y in self.vertical_span() {
            for rel_x in self.horizontal_span() {
                let current = Position(rel_y * self.garden.width + rel_x);
                write!(
                    f,
                    "{}",
                    match self.region.positions.contains(&current) {
                        true => self.region.flower.0,
                        false => ' ',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
struct RegionChange<'a> {
    a: &'a Region,
    b: &'a Region,
    garden: &'a Garden,
}
//#[expect(clippy::needless_lifetimes)]
//impl<'a> Display for RegionChange<'a> {
//    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//        let a = self.a.relative_to(self.garden);
//        let b = self.b.relative_to(self.garden);
//        let a_y_span = a.vertical_span();
//        let b_y_span = b.vertical_span();
//        let y_min = *a_y_span.start().min(b_y_span.start());
//        let y_max = *a_y_span.end().max(b_y_span.end());
//        for rel_y in y_min..=y_max {
//            for a_x in leftmost..=rightmost {
//                let current = Position(rel_y * self.garden.width + a_x);
//                write!(
//                    f,
//                    "{}",
//                    match self.region.positions.contains(&current) {
//                        true => self.region.flower.0,
//                        false => ' ',
//                    }
//                )?;
//            }
//            writeln!(f)?;
//        }
//        Ok(())
//    }
//}
struct Garden {
    inner: Vec<Flower>,
    width: usize,
}
impl Display for Garden {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.inner.chunks_exact(self.width) {
            writeln!(f, "{}", line.iter().map(|a| a.0).join(""))?;
        }
        Ok(())
    }
}
impl Garden {
    fn get_regions(&self) -> Vec<Region> {
        /// Recursive exploration of garden map.
        /// Go every cardinal direction, if the spot is valid and new.
        /// If nowhere to go, propagate up a collection for the valid flower.
        /// This flower connects a region, since recursive must be touching.
        /// If the flower on the upper level is different,
        /// it means an entire regions' flowers was found.
        /// This is stored, such that every one of these is unique.
        /// A vector is used because no key can be used to uniquely identify these,
        /// except an arbitrary position value.
        fn explore(
            garden: &Garden,
            this_pos: Position,
            visited: &mut HashSet<Position>,
        ) -> (Region, Vec<Region>) {
            let this_flower = *garden
                .inner
                .get(this_pos.0)
                .expect("only call this function with a valid position");
            eprintln!(
                "Looking through {} at position {}",
                this_flower.0,
                this_pos.relative_to(garden)
            );

            let (bordering_regions, others): (Vec<Region>, Vec<Vec<Region>>) = garden
                .bordering_pos(&this_pos)
                .filter(|new_pos| garden.has_pos(new_pos))
                .inspect(|pos| eprintln!("Valid new position: {}", pos.relative_to(garden)))
                .filter_map(|valid_pos| {
                    if visited.insert(valid_pos) {
                        // new position, explore branch
                        Some(explore(garden, valid_pos, visited))
                    } else {
                        // positions was already seen
                        None
                    }
                })
                .unzip();

            bordering_regions.into_iter().fold(
                (
                    Region {
                        flower: this_flower,
                        positions: HashSet::from([this_pos]),
                    },
                    others.into_iter().flatten().collect::<Vec<_>>(),
                ),
                |(mut self_similar, mut others), bordering_region| {
                    if bordering_region.flower == self_similar.flower {
                        eprintln!(
                            "Connecting {} of {} flowers at {} to {}.",
                            bordering_region.positions.len(),
                            bordering_region.flower.0,
                            bordering_region
                                .positions
                                .iter()
                                .map(|pos| pos.relative_to(garden))
                                .join(", "),
                            this_pos.relative_to(garden)
                        );
                        eprintln!("bordering...\n{}", bordering_region.relative_to(garden));
                        eprintln!("...with...\n{}", self_similar.relative_to(garden));
                        self_similar.positions.extend(bordering_region.positions);
                        eprintln!("...becomes this:\n{}", self_similar.relative_to(garden));
                    } else {
                        others.push(bordering_region);
                    }

                    (self_similar, others)
                },
            )
        }

        let (first, mut rest) = explore(self, Position(0), &mut HashSet::from([Position(0)]));
        rest.push(first);
        rest
    }
    fn bordering_pos(&self, this_pos: &Position) -> impl Iterator<Item = Position> {
        [
            // left
            this_pos.0.checked_sub(1),
            // right
            this_pos.0.checked_add(1),
            // up
            this_pos.0.checked_sub(self.width),
            // down
            this_pos.0.checked_add(self.width),
        ]
        .into_iter()
        .flatten()
        .map(Position)
    }
    fn has_pos(&self, pos: &Position) -> bool {
        self.inner.len() > pos.0
    }
}

fn parse_garden(input: &mut &str) -> PResult<Garden> {
    separated(1.., parse_line, line_ending)
        // get len of first element
        .verify_map(|v: Vec<Vec<_>>| v.first().map(|v_1| v_1.len()).map(|v_1_len| (v, v_1_len)))
        // ensure all lens are the same
        .verify(|(v, _)| v.iter().map(|v_i| v_i.len()).all_equal())
        .map(|(v, width): (Vec<Vec<_>>, _)| {
            let inner = v.into_iter().flat_map(|v_i| v_i.into_iter()).collect();
            Garden { inner, width }
        })
        .parse_next(input)
}
fn parse_line(input: &mut &str) -> PResult<Vec<Flower>> {
    repeat(1.., any.verify(|c| AsChar::is_alpha(c)).map(Flower)).parse_next(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_1() {
        assert_eq!(super::solve(include_str!("EXAMPLE")), 140);
    }
    #[ignore]
    #[test]
    fn example_2() {
        assert_eq!(super::solve(include_str!("EXAMPLE_2")), 772);
    }
    #[ignore]
    #[test]
    fn example_larger() {
        assert_eq!(super::solve(include_str!("EXAMPLE_LARGER")), 1930);
    }

    #[ignore]
    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/12")), 0);
    }
}