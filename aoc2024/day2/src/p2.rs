use itertools::{
	FoldWhile::{Continue, Done},
	Itertools
};

mod common;

fn main() {
	let res = solve(util::day_input::<2>());
}

fn solve(input: impl AsRef<str>) -> u32 {
	input
		.as_ref()
		.lines()
		.map(|line| {
			line.split(' ')
				.map(|n| n.parse::<u32>())
				.collect::<Result<Vec<_>, _>>()
				.expect("numbers are parsable")
		})
		.filter(|level| {
			fn test<'i>(dir: impl Iterator<Item = &'i u32>) -> bool {
				dir.tuple_windows()
					.fold_while(Some(true), |acc, (a, b)| {
						match (*a as i32 - *b as i32, acc) {
							(1..=3, prev) => Continue(prev),
							(_, Some(true)) => Continue(Some(false)),
							(_, Some(false) | None) => Done(None)
						}
					})
					.into_inner()
					.is_some()
			}
			test(level.iter()) || test(level.iter().rev())
		})
		.count() as u32
}

#[cfg(test)]
mod p2test {
	#[test]
	fn sample_solvable() {
		assert_eq!(super::solve(super::common::SAMPLE), 4)
	}
}
