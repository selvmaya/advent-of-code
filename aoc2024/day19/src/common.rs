use std::collections::HashSet;
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

use std::fmt::{Display, Formatter};

pub fn parse_input(input: &mut &str) -> PResult<(HashSet<StripePattern>, Vec<StripePattern>)> {
    separated_pair(parse_available, (line_ending, line_ending), parse_designs).parse_next(input)
}

fn parse_available(input: &mut &str) -> PResult<HashSet<StripePattern>> {
    separated(1.., parse_pattern, ", ").parse_next(input)
}
fn parse_designs(input: &mut &str) -> PResult<Vec<StripePattern>> {
    separated(1.., parse_pattern, line_ending).parse_next(input)
}

fn parse_pattern(input: &mut &str) -> PResult<StripePattern> {
    repeat(1.., parse_color)
        .map(StripePattern)
        .parse_next(input)
}
fn parse_color(input: &mut &str) -> PResult<StripeColor> {
    alt((
        'w'.value(StripeColor::White),
        'u'.value(StripeColor::Blue),
        'b'.value(StripeColor::Black),
        'r'.value(StripeColor::Red),
        'g'.value(StripeColor::Green),
    ))
    .parse_next(input)
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct StripePattern(pub Vec<StripeColor>);
impl StripePattern {
    pub fn single_removed_sub_patterns<'p>(
        &'p self,
        full_set: &'p HashSet<StripePattern>,
    ) -> impl Iterator<Item = StripePattern> + 'p {
        full_set.iter().filter_map(|existing| {
            // pattern order must be preserved, we can only remove from the edges
            if self.0.starts_with(&existing.0[..]) {
                let len = existing.0.len();
                let sub_inner: Vec<_> = self.0[len..].into();
                if !sub_inner.is_empty() {
                    Some(StripePattern(sub_inner)) // pattern without this start
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum StripeColor {
    /// `w`
    White,
    /// `u`
    Blue,
    /// `b`
    Black,
    /// `r`
    Red,
    /// `g`
    Green,
}

impl Display for StripeColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StripeColor::White => 'w',
                StripeColor::Blue => 'u',
                StripeColor::Black => 'b',
                StripeColor::Red => 'r',
                StripeColor::Green => 'g',
            }
        )
    }
}
