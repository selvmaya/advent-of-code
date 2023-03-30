use crate::Runnable;
pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        let input = input.as_str();
        part_1_solve(input);
        part_2_solve(input);
    }
}

use nom::{combinator::all_consuming, Finish};
use parse::Instruction;

mod parse;

use eframe::egui;

fn part_1_solve(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|line| all_consuming(Instruction::parse)(line).finish().unwrap().1)
        .collect::<Vec<_>>();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "AoC 2022 - Day 9",
        options,
        Box::new(|_cc| Box::new(MyApp { instructions }))
    ).unwrap();

    // todo!("answer part 1");
    0
}

struct MyApp {
    instructions: Vec<Instruction>,
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Instructions:");
            for ins in &self.instructions {
                let arrow = match ins.dir {
                    parse::Direction::Up => "⬆",
                    parse::Direction::Down => "⬇",
                    parse::Direction::Right => "➡",
                    parse::Direction::Left => "⬅",
                };
                ui.label(arrow.repeat(ins.dist as _));
            }
        });
    }
}

fn part_2_solve(_input: &str) {
    // todo!("answer part 2");
}

#[cfg(test)]
mod tests {
    use super::*;
    mod part_1 {
        use super::*;
        #[test]
        fn test_solver() {
            let test_result = part_1_solve(include_str!("../inputs/day_9_test.txt"));
            assert_eq!(test_result, 13)
        }
    }
}