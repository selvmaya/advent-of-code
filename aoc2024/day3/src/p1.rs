mod common;

fn main() {
    util::DayInput::find::<3>().solve_with(solve);
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
