#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    token::*,
    {PResult, Parser},
};

#[doc = include_str!("../p2.md")]
fn main() {
    util::DayInput::find::<6>().solve_with(solve);
}

/// # Problem
/// Figure out how many different ways you can make the guard walk in an infinite loop,
/// with only a single added obstacle.
/// # Solution
/// This is pretty hard, assuming we can't check every single spot and just simulate them all.
/// ## Loops and exits
/// A way to guarantee that the guard is in an infinite loop is by simulating guard movement along some path.
/// If they end up in the same spot with the same facing direction, it means they are looping.
/// The guard is *either* "infinitely looping" and will traverse the same spot twice,
/// or they have left the map.
/// We don't havé to check *every* spot, just the ones the guard could possibly reach.
/// ## Basic idea
/// This can be done with BFS. Go along the standard guard path, and for every step check if the
/// different path created by an obstacle creates an infinite loop or not, using the above method.
fn solve(input: impl AsRef<str>) -> u64 {
    todo!()
}
#[cfg(test)]
mod tests {

    #[test]
    fn input_solvable() {
        assert_eq!(super::solve(include_str!("../../inputs/6")), 0);
    }
}