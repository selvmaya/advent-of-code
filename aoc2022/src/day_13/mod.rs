use std::str::FromStr;

use itertools::Itertools;

use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
	fn run_with_input(&self, input: String) {
		println!("PART 1: {}", solve_part_1(&input));
		// solve_part_2(input);
	}
}

/// We get a distress signal. The packets are decoded out of order.
///
/// List contains pairs of packets, separated by a blank line.
///
/// Packet data consists of lists and integers (in those lists).
/// Lists start with '[', ends with ']', and contains zero or more comma-
/// separated values (integers or other lists).
/// Comparing two values, one can be called "left", and the other "right".
/// Each packet is always a list on its own line,
/// and packets come in pairs of two.
/// The first value will be left, the second value will be right.
///
/// The rules below define whether a pair of two packets are in the right order.
/// - If both values are integers, the lower value should come first.
/// - If both values are lists, then the shorter list should come first.
/// - If exactly one of the values are an integer, turn it into a list with
///   itself as the only element, then do list comparison.
/// In short: when comparing first to second, first should be Ordering::Less.
/// SOLVE: Amount of pairs that are in the right order.
fn solve_part_1(input: &str) -> usize {
	let lines_with_specific_separator = input.lines().map(|line| line.trim()).join("\n");

	let pairs_of_lines = lines_with_specific_separator.split("\n\n");
	let pairs = pairs_of_lines.map(|pair_of_lines| {
		println!("PAIR STR: {}", pair_of_lines.split('\n').join(", "));
		let (left, right) = pair_of_lines
			.split_once('\n')
			.expect("tried creating pair from wrong split");
		// dbg!(left, right);
		Pair {
			left:  left.parse().expect("left element could not be parsed"),
			right: right.parse().expect("right element could not be parsed")
		}
	});
	let right_order_pairs = pairs.filter(|pair| pair.left < pair.right);
	right_order_pairs.count()
}
#[derive(Debug)]
struct Pair {
	left:  Element,
	right: Element
}
#[derive(Debug, PartialEq, Eq)]
enum Element {
	Integer(usize),
	List(Vec<Element>)
}
impl FromStr for Element {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// outermost packet layer is always list,
		// below layers will also be collected as list(s)
		let mut elements: Vec<Element> = Vec::new();
		let mut surface_integer_chars: Vec<char> = Vec::new();

		let mut sub_chars: Vec<char> = Vec::new();
		let mut nesting_depth = 0;
		for c in s.chars() {
			match c {
				'[' => {
					// go below
					nesting_depth += 1;
				},
				']' => {
					if nesting_depth == 0 {
						break; // assume that this will only happen at the end
					}
					// go back up
					nesting_depth -= 1;
					if nesting_depth == 0 {
						assert!(!sub_chars.is_empty());
						// get result of substring
						elements.push(sub_chars.iter().collect::<String>().parse::<Element>()?);
						sub_chars.clear();
					}
				},
				',' => {
					assert!(
						!surface_integer_chars.is_empty(),
						"assume integer split has element before this point"
					);
					if nesting_depth > 0 {
						sub_chars.push(',')
					} else {
						elements.push(Element::Integer(
							surface_integer_chars
								.iter()
								.collect::<String>()
								.parse::<usize>()
								.map_err(|_| "Could not parse integer?".to_string())?
						));
						surface_integer_chars.clear();
					}
				},
				integer_char => {
					// assume integer
					if nesting_depth > 0 {
						sub_chars.push(integer_char);
					} else {
						surface_integer_chars.push(integer_char);
					}
				}
			}
		}
		Ok(Element::List(elements))
	}
}
impl PartialOrd for Element {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match (self, other) {
			// different comparison (convert before new compare)
			(Self::Integer(self_int), Self::List(_)) => {
				Self::List(vec![Self::Integer(*self_int)]).partial_cmp(other)
			},
			(Self::List(_), Self::Integer(other_int)) => {
				self.partial_cmp(&Self::List(vec![Self::Integer(*other_int)]))
			},
			// similar comparison
			(Self::Integer(self_int), Self::Integer(other_int)) => self_int.partial_cmp(other_int),
			(Self::List(self_list), Self::List(other_list)) => {
				self_list.len().partial_cmp(&other_list.len())
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn sample_1_works() {
		const SAMPLE_1_STR: &str = include_str!("sample_1.txt");
		assert_eq!(solve_part_1(SAMPLE_1_STR), 13);
	}
}
