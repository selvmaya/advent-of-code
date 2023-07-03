use std::{cmp::Ordering, fmt::Display, str::FromStr};

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
	let mut index = 1;
	let pairs: Vec<Pair> = pairs_of_lines
		.map(|pair_of_lines| {
			// println!("PAIR STR: '{}'", pair_of_lines.split('\n').join("', '"));
			let (left, right) = pair_of_lines
				.split_once('\n')
				.expect("tried creating pair from wrong split");
			let (left, right) = (string_without_edges(left), string_without_edges(right));
			let new_pair = Pair {
				index,
				left: left.parse().expect("left element could not be parsed"),
				right: right.parse().expect("right element could not be parsed")
			};
			// println!(
			// 	"\nCompleted pair:\nLEFT - {}\nRIGHT - {}\n",
			// 	new_pair.left, new_pair.right
			// );
			index += 1;
			new_pair
		})
		.collect();

	let right_order_pairs: Vec<Pair> = pairs
		.into_iter()
		.filter(|pair| pair.left < pair.right)
		.collect();
	right_order_pairs.into_iter().map(|pair| pair.index).sum()
}
fn string_without_edges(s: &str) -> &str {
	let mut chars = s.chars();
	chars.next();
	chars.next_back();
	chars.as_str()
}
#[derive(Debug)]
struct Pair {
	index: usize,
	left:  Element,
	right: Element
}
#[derive(Debug, PartialEq, Eq)]
enum Element {
	Integer(usize),
	List(Vec<Element>)
}
impl Display for Element {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let element_str = match self {
			Element::Integer(i) => format!("{}", i),
			Element::List(l) => format!("[{}]", l.iter().join(", "))
		};
		write!(f, "{}", element_str)
	}
}
impl FromStr for Element {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		println!("- Parsing string '{}'", s);
		// outermost packet layer is always list,
		// below layers will also be collected as list(s)
		let mut elements: Vec<Element> = Vec::new();
		let mut surface_integer_chars: Vec<char> = Vec::new();

		let mut sub_chars: Vec<char> = Vec::new();
		let mut nesting_depth = 0;
		for c in s.chars() {
			match c {
				'[' => {
					if nesting_depth > 0 {
						sub_chars.push('[');
					}
					// go below
					nesting_depth += 1;
					println!("> Increased nesting depth to {}", nesting_depth);
				},
				']' => {
					if nesting_depth == 0 {
						assert!(sub_chars.is_empty());
						// assume that this will only happen at the end (early return)
						break;
					}
					// go back up recursive stack
					nesting_depth -= 1;
					if nesting_depth > 0 {
						sub_chars.push(']');
					}
					if nesting_depth == 0 && !sub_chars.is_empty() {
						// get result of substring
						elements.push(sub_chars.iter().collect::<String>().parse::<Element>()?);
						sub_chars.clear();
					}
				},
				',' => {
					if nesting_depth > 0 {
						sub_chars.push(',')
					} else if !surface_integer_chars.is_empty() {
						let new_integer = Element::Integer(
							surface_integer_chars
								.iter()
								.collect::<String>()
								.parse::<usize>()
								.map_err(|_| "Could not parse integer?".to_string())?
						);
						println!(
							"Adding integer {} to list [{}]...",
							new_integer,
							elements.iter().join(", ")
						);
						elements.push(new_integer);
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
		if !surface_integer_chars.is_empty() {
			let new_integer = Element::Integer(
				surface_integer_chars
					.iter()
					.collect::<String>()
					.parse::<usize>()
					.map_err(|_| "Could not parse integer?".to_string())?
			);
			println!(
				"Adding integer {} to list [{}]...",
				new_integer,
				elements.iter().join(", ")
			);
			elements.push(new_integer);
		}
		let elements = Element::List(elements);
		println!("; Parsed elements: {}", elements);
		Ok(elements)
	}
}
impl PartialOrd for Element {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		println!("Comparing self '{}' with other '{}'", self, other);
		let result = match (self, other) {
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
				let mut ordering = self_list.len().partial_cmp(&other_list.len());
				let smallest_len = self_list.len().min(other_list.len());
				for i in 0..smallest_len {
					let self_element = &self_list[i];
					let other_element = &other_list[i];
					match self_element.partial_cmp(other_element) {
						Some(Ordering::Equal) => {},
						alt => {
							println!(
								"Compared {} with {} and found ordering {:?}",
								self_element, other_element, alt
							);
							ordering = alt;
							break;
						}
					}
				}
				ordering
			}
		};
		println!("Result from comparison: {:?}\n", result);
		result
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
