use itertools::Itertools;
use proc_macro::TokenStream;
use std::ops::RangeInclusive;

const REST_DAYS: RangeInclusive<u8> = 1..=13;
// const INDEPENDENT_DAYS: RangeInclusive<u8> = 14..=14;

// const TOTAL_DAYS: RangeInclusive<u8> =
// 	RangeInclusive::new(*REST_DAYS.start(), *INDEPENDENT_DAYS.end());

#[proc_macro]
pub fn use_independent_days(_item: TokenStream) -> TokenStream {
	REST_DAYS
		.map(|num| format!("use day_{:0>2};", num))
		.join("\n")
		.parse()
		.unwrap()
}

#[proc_macro]
pub fn two_digit(num: TokenStream) -> TokenStream {
	format!("{:0>2}", num.to_string()).parse().unwrap()
}

#[proc_macro]
pub fn map_to_part_functions(_item: TokenStream) -> TokenStream {
	format!(
		"match day {{{}\n{}\n}}",
		REST_DAYS
			.map(|num| format!("{0} => day_{0:0>2}::PARTS,", num))
			.join("\n"),
		"_ => unreachable!(\"program was asked to solve a day that was not known\")"
	)
	.parse()
	.unwrap()
}