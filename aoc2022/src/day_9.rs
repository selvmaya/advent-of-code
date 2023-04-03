use crate::Runnable;
pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        let input = input.as_str();
        part_1_solve(input);
        part_2_solve(input);
    }
}

use egui::{Color32, Sense, Stroke};
use nom::{combinator::all_consuming, Finish};
use parse::Instruction;

mod parse;

use eframe::egui;

use self::parse::GridPos;

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
        Box::new(|_cc| {
            Box::new(MyApp {
                instructions,
                head: GridPos { x: 0, y: 0 },
                tail: GridPos { x: 1, y: 1 },
            })
        }),
    )
    .unwrap();

    // todo!("answer part 1");
    0
}

struct MyApp {
    instructions: Vec<Instruction>,
    head: GridPos,
    tail: GridPos,
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
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
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter_size = egui::vec2(250.0, 250.0);
            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            let center = res.rect.center().to_vec2();

            const SIDE: f32 = 16.0;
            let to_panel_pos = |pos: GridPos| {
                (egui::vec2(pos.x as f32 * SIDE, pos.y as f32 * SIDE) + center).to_pos2()
            };

            for x in -30..30 {
                for y in -20..20 {
                    let dot = GridPos { x, y };
                    let is_zero = dot.x == 0 && dot.y == 0;

                    let color = if is_zero {
                        Color32::DARK_RED
                    } else {
                        Color32::LIGHT_GRAY
                    };
                    painter.circle_stroke(to_panel_pos(dot), 1.0, Stroke::new(1.0, color));
                }
            }

            // paint the head
            let head_pos = to_panel_pos(self.head);
            painter.circle_stroke(head_pos, 6.0 / 2.0, Stroke::new(2.0 / 2.0, Color32::GREEN));

            // paint the tail
            let tail_pos = to_panel_pos(self.tail);
            painter.circle_stroke(tail_pos, 3.0 / 2.0, Stroke::new(2.0 / 2.0, Color32::YELLOW));

            // paint an arrow from head to tail
            painter.arrow(
                tail_pos,
                head_pos - tail_pos,
                Stroke::new(2.0 / 2.0, Color32::YELLOW),
            );
        });
        ctx.request_repaint();
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
