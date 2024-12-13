#![doc = include_str!("../p1.md")]

#[allow(unused_imports)]
use {p1_garden::*, p1_regions::*};

mod p1_garden;
mod p1_regions;

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
    let garden = input.as_ref().parse::<Garden>().expect("parsable");
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
                region.flower,
                area,
                perimeter,
                region.relative_to(&garden)
            );
            area * perimeter
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_square() {
        assert_eq!(super::solve(include_str!("SQUARE")), 4 * 8);
    }
    #[ignore]
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
