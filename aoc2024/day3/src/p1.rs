mod common;
mod p1fake;

fn main() {
	let res = solve(util::day_input::<3>());
	println!("Result: {}", res);
}

fn solve(input: impl AsRef<str>) -> u32 {
	todo!()
}

#[cfg(test)]
mod p1test {
	#[test]
	fn sample_solves() {
		assert_eq!(super::solve(super::common::SAMPLE), todo!());
	}
}
