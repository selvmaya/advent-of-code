use crate::Runnable;
pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        let input = input.as_str();
        println!("PART 1: {}", part_1_solve(input));
        println!("PART 2: {}", part_2_solve(input));
    }
}

// imagine/simulate head of rope moving on 2D grid, and we can determine how the tail of the rope will move

// moves on 

// head and tail of rope must "always be touching" (diagonals and overlaps count)

// after simulating the rope, "you can count up all the positions the tail visited atleast once"

// SOLVE FOR: how many positions does the tail of the rope visit atleast once?
fn part_1_solve(input: &str) -> usize {
    let series_of_motions = input;
    todo!()
}
fn part_2_solve(_input: &str) -> &str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = include_str!("day_9_sample.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(13, part_1_solve(TEST_INPUT));
    }
    #[test]
    fn part_2_test() {
        assert_eq!("", part_2_solve(TEST_INPUT));
    }
}