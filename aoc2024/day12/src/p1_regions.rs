use crate::p1_garden::{Flower, Garden, Position};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Region {
    pub flower: Flower,
    pub positions: HashSet<Position>,
}

impl Region {
    pub fn relative_to<'a>(&'a self, garden: &'a Garden) -> RelativeRegion<'a> {
        RelativeRegion {
            region: self,
            garden,
        }
    }
    pub fn changing_to<'a>(&'a self, other: &'a Self, garden: &'a Garden) -> RegionChange<'a> {
        RegionChange {
            a: self,
            b: other,
            garden,
        }
    }
}

pub struct RelativeRegion<'a> {
    pub region: &'a Region,
    pub garden: &'a Garden,
}

#[expect(clippy::needless_lifetimes)]
impl<'a> RelativeRegion<'a> {
    pub fn x_span(&self) -> RangeInclusive<usize> {
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
    pub fn y_span(&self) -> RangeInclusive<usize> {
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
        let y_span = self.y_span();
        let max_y = *y_span.end();
        for rel_y in y_span {
            for rel_x in self.x_span() {
                let current = Position(rel_y * self.garden.width + rel_x);
                write!(
                    f,
                    "{}",
                    match self.region.positions.contains(&current) {
                        true => self.region.flower,
                        false => Flower(' '),
                    }
                )?;
            }
            if rel_y != max_y {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub struct RegionChange<'a> {
    pub a: &'a Region,
    pub b: &'a Region,
    pub garden: &'a Garden,
}

#[expect(clippy::needless_lifetimes)]
/// Draw a change between regions.
/// It should look like two boxes, one may be larger than the other (in either direction).
impl<'a> Display for RegionChange<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self.a.relative_to(self.garden);
        let b = self.b.relative_to(self.garden);
        let max_draw_y = a
            .y_span()
            .try_len()
            .expect("non-empty")
            .max(b.y_span().try_len().expect("non_empty"));
        let middle_draw_y = max_draw_y / 2;
        for draw_y in 0..max_draw_y
        {
            let try_draw = |f: &mut Formatter, r: &RelativeRegion| {
                let x_start = (r.y_span().start() + draw_y) * self.garden.width;
                // draws one region's part of a line
                for x_pos in r.x_span() {
                    let pos = Position(x_start + x_pos);
                    write!(
                        f,
                        "{}",
                        if r.region.positions.contains(&pos) {
                            r.region.flower
                        } else {
                            Flower(' ')
                        }
                    )?;
                }
                Ok(())
            };

            // draw horizontal line of region `a`
            try_draw(f, &a)?;

            // draw arrow or empty spacing
            if draw_y == middle_draw_y {
                write!(f, " --> ")?;
            } else {
                write!(f, "     ")?;
            }

            // draw horizontal line of region `b`
            try_draw(f, &b)?;

            // end drawing of line, if not on last
            if draw_y != max_draw_y {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::p1_garden::{Flower, Garden};
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn region_flowers_are_the_same() {
        let garden = include_str!("EXAMPLE").parse::<Garden>().unwrap();
        let all_regions = garden.get_regions();
        for region in all_regions.iter() {
            for position in region.positions.iter() {
                assert_eq!(
                    garden.inner.get(position.0),
                    Some(&region.flower),
                    "All of the flowers in the region must be the same."
                );
            }
        }
    }
    #[ignore]
    #[test]
    fn regions_are_distinct_flowers() {
        let garden = include_str!("EXAMPLE").parse::<Garden>().unwrap();
        let all_regions = garden.get_regions();
        let regions_grouped = all_regions.iter().into_group_map_by(|r| r.flower);
        for self_similar_regions in regions_grouped.values() {
            assert_eq!(
                self_similar_regions.len(),
                1,
                "Every region in EXAMPLE has a unique flower.\n\
                Examples where this is not the case:\n\
                {}",
                self_similar_regions
                    .iter()
                    .map(|r| r.relative_to(&garden))
                    .join(",\n")
            );
        }
    }
    #[ignore]
    #[test]
    fn spans_function_correctly() {
        let garden = include_str!("EXAMPLE").parse::<Garden>().unwrap();
        let all_regions = garden.get_regions();
        let region_map = all_regions
            .iter()
            .map(|r| (r.flower, r.relative_to(&garden)))
            .collect::<HashMap<_, _>>();

        let tests = [
            (Flower('A'), 0..=3, 0..=0),
            (Flower('B'), 1..=2, 0..=1),
            (Flower('C'), 2..=3, 1..=3),
            (Flower('D'), 1..=1, 3..=3),
            (Flower('E'), 0..=2, 3..=3),
        ];
        for (flower, x_span, y_span) in tests {
            let r = region_map.get(&flower).unwrap();
            eprintln!("Region:\n{}", r);
            assert_eq!(
                r.x_span(),
                x_span,
                "X span is the relevant range of columns for {}.",
                flower
            );
            assert_eq!(
                r.y_span(),
                y_span,
                "Y span is the relevant range of rows for {}.",
                flower
            );
            eprintln!("Region works!\n");
        }
    }
}