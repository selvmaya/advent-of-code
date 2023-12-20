use itertools::Itertools;
use proc_macro::TokenStream;
use std::ops::RangeInclusive;

const DAYS_INCLUDED: RangeInclusive<u8> = 13..=13;

#[proc_macro]
pub fn declare_modules(_item: TokenStream) -> TokenStream {
    DAYS_INCLUDED
        .map(|num| format!("mod day_{:0>2};", num))
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
        DAYS_INCLUDED
            .map(|num| format!("{0} => day_{0:0>2}::PARTS,", num))
            .join("\n"),
        "_ => unreachable!(\"program was asked to solve a day that was not known\")"
    )
    .parse()
    .unwrap()
}