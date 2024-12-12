use itertools::Itertools;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, repeat, separated};
use winnow::error::StrContext;
use winnow::token::take;
use winnow::{PResult, Parser};

pub struct TopographicMap {
    /// This should have a len of `width * height`.
    pub inner: Vec<Location>,
    pub width: u8, // The rectangle isn't that big.
}
impl TopographicMap {
    pub fn trailheads(&self) -> impl Iterator<Item = usize> + '_ {
        self.inner
            .iter()
            .enumerate()
            .filter_map(|(pos, loc)| loc.0.and_then(|height| (height == 0).then_some(pos)))
    }
    pub fn all_dir_iter(
        &self,
        pos: usize,
        one_higher: u8,
        map_width: usize,
    ) -> impl Iterator<Item = usize> + '_ {
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
            match self.inner.get(potential_pos) {
                Some(Location(Some(potential_height))) => {
                    // search in 1-higher trail path direction
                    *potential_height == one_higher
                }
                _ => false,
            }
        })
    }
}

/// In the problem this is called the "position"
pub struct Location(pub Option<u8>);

pub fn parse_map(input: &mut &str) -> PResult<TopographicMap> {
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
