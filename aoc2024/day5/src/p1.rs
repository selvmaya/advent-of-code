#![doc = include_str!("../p1.md")]
use itertools::Itertools;
#[allow(unused_imports)]
use winnow::{
	ascii::*,
	combinator::*,
	error::*,
	token::*,
	{PResult, Parser}
};

fn main() {
	util::DayInput::find::<5>().solve_with(solve);
}

/// # Specification
/// Some ordering rules are given, one per line.
/// These are numbers that should be before (are less than) other
/// numbers, irrespective of their decimal value.
/// Then, after a line break, some updates are given, one per line.
/// An update is a list of comma-separated numbers.
/// If an update is in order, find its middle value.
/// Updates can be assumed to be of odd length such that a middle value exists.
/// We also assume for now that ordering rules are absolute and true;
/// and do not contradict each other even for indirect links.
/// If it did have contradictions or similar, the rules would be circular
/// (chicken or egg). Both X and Y page numbers need to be present in an update
/// for their immediate ordering to have an effect, but this shouldn't make any
/// difference for other values between them. We may assume that updates do not
/// print the same page number more than once. # Solution A:
/// If we can make a way of sorting or dynamically comparing values specified in
/// the rules, the rest is trivial.
/// The problem is the implementing this is not straight-forward,
/// the comparison rules are arbitrary and only specified on a need-to-know
/// basis. It may be possible to use the need-to-know information to construct a
/// truly ordered list, though, that can be used to compare with the updates.
/// This would at least be a lot faster than checking for any specifically
/// illegal rule in the whole list.
/// The only way I can this being done correctly is by collecting all the rules,
/// then starting with the only page number that does not have a rule with
/// itself as the Y value. The problem with this alone is that multiple page
/// numbers can be mentioned only as X. If multiple numbers are mentioned only
/// as X, their relative ordering is still irrelevant. Same goes for only Y, but
/// also for all values that are not compared directly. If 1<5, 3<6,
/// and 1<6, that does not mean 1<3; for all intents and purposes they are equal
/// values. This means there are multiple valid sorting configurations for a set
/// of updates.
fn solve(input: impl AsRef<str>) -> u32 {
	let (x_smaller_than_y, updates) = parse.parse_next(&mut input.as_ref()).expect("parsable");
	let rule_map = x_smaller_than_y.into_iter().into_group_map();
	updates
		.into_iter()
		.filter_map(|update| {
			update.into_iter().try_fold(Vec::new(), |mut prev, next| {
				if rule_map
					.get(&next)
					.is_some_and(|my_ys| my_ys.iter().any(|y| prev.contains(y)))
				{
					None
				} else {
					prev.push(next);
					Some(prev)
				}
			})
		})
		.map(|update| update[update.len() / 2].0 as u32)
		.sum()
}

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
pub struct PageNumber(pub u8);

#[allow(clippy::type_complexity)]
pub fn parse(input: &mut &str) -> PResult<(Vec<(PageNumber, PageNumber)>, Vec<Vec<PageNumber>>)> {
	separated_pair(parse_rules, multispace1, parse_updates).parse_next(input)
}

fn parse_rules(input: &mut &str) -> PResult<Vec<(PageNumber, PageNumber)>> {
	separated(
		1..,
		separated_pair(
			dec_uint::<&str, u8, ContextError>.map(PageNumber),
			"|",
			dec_uint::<&str, u8, ContextError>.map(PageNumber)
		),
		line_ending
	)
	.parse_next(input)
}
fn parse_updates(input: &mut &str) -> PResult<Vec<Vec<PageNumber>>> {
	separated(1.., parse_update, line_ending).parse_next(input)
}
fn parse_update(input: &mut &str) -> PResult<Vec<PageNumber>> {
	separated(1.., dec_uint.map(PageNumber), ",").parse_next(input)
}

#[cfg(test)]
mod p1test {
	const SAMPLE: &str = include_str!("SAMPLE");

	#[test]
	fn sample_solves() {
		assert_eq!(super::solve(SAMPLE), 143);
	}

	//#[ignore]
	#[test]
	fn input_solvable() {
		assert_eq!(super::solve(include_str!("../../inputs/5")), 5091);
	}
}
