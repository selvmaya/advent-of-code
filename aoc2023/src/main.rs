#![allow(unused)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    // let day_input = std::fs::read_to_string("input/day_1.txt").unwrap();
    // day_1::part_1(&day_input);
    // day_1::part_2(&day_input);
    // let day_input = std::fs::read_to_string("input/day_2.txt").unwrap();
    // println!("{}", day_2::part_1(&day_input));
    // println!("{}", day_2::part_2(&day_input));
    // let day_input = std::fs::read_to_string("input/day_3.txt").unwrap();
    // println!("{}", day_3::part_1(&day_input));
    // println!("{}", day_3::part_2(&day_input));
    let day_input = std::fs::read_to_string("input/day_4.txt").unwrap();
    // println!("{}", day_4::part_1(&day_input));
    println!("{}", day_4::part_2(&day_input));
}