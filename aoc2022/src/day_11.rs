use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        println!("PART 1:\n{}\n", part_1_solve(&input));
        println!("PART 2:\n{}\n", part_2_solve(&input));
    }
}