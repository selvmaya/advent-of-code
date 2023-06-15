use std::fmt::Display;
use std::ops::Range;
use std::str::FromStr;

use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        println!("PART 1:\n{}\n", part_1_solve(&input));
        println!("PART 2:\n{}\n", part_2_solve(&input));
    }
}

/// CPU only has one register and two types of instructions.
/// The register 'X' has a signed integer value that starts at 1.
/// The two instructions are:
/// - "addx V" (where 'V' is a signed integer),
/// takes two cycles to complete, and changes 'X' by 'V' *at the end of the second cycle*.
/// - "noop", takes one cycle to complete and does nothing.
/// 
/// 'signal strengths' is the cycle count (starting at one, increased by instructions),
/// multiplied by the value of register 'X'.
///  
/// relevant cycles: 60th, 100th, 140th, 180th and 220th cycles (20 += 40 -> 220)
/// 
/// SOLVE: find sum of signals strengths during relevant cycles  
fn part_1_solve(input: &str) -> isize {
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut cpu = Cpu::default();
    let mut cycles_to_read_signal_strength: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut read_cycle_signal_strength = vec![];
    for instruction in instructions {
        cpu.give_instruction(instruction);
        while let CpuState::RunningInstruction(_) = &cpu.state {
            if let Some(cycle_read) = cycles_to_read_signal_strength.first() {
                if &cpu.cycle_count() == cycle_read {
                    cycles_to_read_signal_strength.remove(0);
                    read_cycle_signal_strength.push(cpu.read_signal_strength());
                }
            }
            cpu.run_cycle();
        }
    }
    read_cycle_signal_strength.into_iter().sum()
}
#[derive(Debug, PartialEq)]
enum Instruction {
    AddX(isize),
    Noop
}
impl FromStr for Instruction {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.trim();
        if let Some((_, x_str)) = line.split_once(' ') {
            let x = x_str.parse().map_err(|_| "Could not parse value as int".to_string())?;
            Ok(Self::AddX(x))
        }
        else {
            match line {
                "noop" => Ok(Self::Noop),
                _ => Err(format!("No valid value found in '{}'", line,))
            }
        }
    }
}
#[derive(Debug, PartialEq)]
enum CpuState {
    RunningInstruction(Instruction),
    ReadyForInstruction
}
type Register = isize;
struct Cpu {
    x: Register,
    total_cycle_index: usize,
    state: CpuState,
    state_cycle_count: usize
}
impl Default for Cpu {
    fn default() -> Self {
        Self { x: 1, total_cycle_index: 0, state: CpuState::ReadyForInstruction, state_cycle_count: 0 }
    }
}
impl Cpu {
    fn run_cycle(&mut self) {
        let CpuState::RunningInstruction(instruction) = &self.state else {
            panic!("Should never be able to call this function without an active instruction");
        };
        self.total_cycle_index += 1;
        self.state_cycle_count += 1;
        match instruction {
            Instruction::AddX(v) => {
                if self.state_cycle_count == 2 {
                    self.x += v;
                    self.state = CpuState::ReadyForInstruction;
                }
            },
            Instruction::Noop => {
                if self.state_cycle_count == 1 {
                    self.state = CpuState::ReadyForInstruction;
                }
            }, 
        }
    }
    fn give_instruction(&mut self, instruction: Instruction) {
        assert!(self.state == CpuState::ReadyForInstruction,
            "Should never be able to call this function with an active instruction");

        self.state = CpuState::RunningInstruction(instruction);
        self.state_cycle_count = 0;
    }
    fn cycle_count(&self) -> usize {
        self.total_cycle_index + 1
    }
    fn read_signal_strength(&self) -> isize {
        self.cycle_count() as isize * self.x
    }
}

/// X register on CPU turns out to control the horizontal position of a sprite.
/// The sprite is displayed on the CRT (scanline display).
/// The CRT displays from top to bottom, left to right.
/// The display is 40 pixels wide and 6 pixels tall.
/// The sprite is 3 pixels wide, and its position is the middle of those pixels.
/// 
/// The CRT draws a single pixel (during) each cycle.
/// A pixel is signaled "on" (lit) with the '#' character,
/// and "off" (dark) with the '.' character.
/// 
/// Since the CRT can only draw one pixel at a time,
/// it will take multiple cycles to draw the full sprite.
/// This means that while the 'X' register on the CPU doesn't move,
/// we must continue drawing the same sprite, with offsets 0-2.
/// 
/// SOLVE: What eight capital letters appear on the CRT?
fn part_2_solve(input: &str) -> String {
    let instructions: Vec<Instruction> = input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    let mut crt = Crt::default();
    let mut crt_output_buffer = vec![];
    for instruction in instructions {
        crt.cpu.give_instruction(instruction);
        while let CpuState::RunningInstruction(_) = &crt.cpu.state {
            let sprite_area = crt.cpu.x - 1 ..= crt.cpu.x + 1;
            if sprite_area.contains(&(crt.pixel_index() as isize)) {
                crt.draw_pixel();
            }

            crt.cpu.run_cycle();

            if crt.cpu.total_cycle_index % CYCLES_IN_CRT_DISPLAY == 0 {
                let output = format!("{}", crt);
                crt_output_buffer.push(output);
                crt.display = DEFAULT_DISPLAY;
                println!();
            }
        }
    }
    crt_output_buffer.join("\n")
}
#[derive(Clone, Copy)]
enum Pixel {
    Dark, // default, treated as "unknown"
    Lit,
}
impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Pixel::Dark => '.',
            Pixel::Lit => '#',
        })
    }
}
const PIXELS_IN_SCANLINE: usize = 40;
const SCANLINES_IN_CRT: usize = 6;
const CYCLES_IN_CRT_DISPLAY: usize = PIXELS_IN_SCANLINE * SCANLINES_IN_CRT;
type Scanline = [Pixel; PIXELS_IN_SCANLINE]; // 40: 0-39
type CrtDisplay = [Scanline; SCANLINES_IN_CRT]; // 6: 0-5
const DEFAULT_DISPLAY: CrtDisplay = [[Pixel::Dark; 40]; 6];
struct Crt {
    display: CrtDisplay,
    cpu: Cpu
}
impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut scanline_strs: Vec<String> = vec![];
        for scanline in self.display {
            let mut pixel_strs: Vec<String> = vec![];
            for pixel in scanline {
                pixel_strs.push(format!("{}", pixel));
            }
            scanline_strs.push(pixel_strs.join(""));
        }
        let display = scanline_strs.join("\n");
        write!(f, "{}", display)
    }
}
impl Default for Crt {
    fn default() -> Self {
        Crt {
            cpu: Cpu::default(),
            display: DEFAULT_DISPLAY,
        }
    }
}
impl Crt {
    fn crt_cycle_index(&self) -> usize {
        self.cpu.total_cycle_index % CYCLES_IN_CRT_DISPLAY
    }
    fn scanline_index(&self) -> usize {
        self.crt_cycle_index() / PIXELS_IN_SCANLINE
    }
    fn pixel_index(&self) -> usize {
        self.crt_cycle_index() % PIXELS_IN_SCANLINE
    }
    fn draw_pixel(&mut self) {
        self.display[self.scanline_index()][self.pixel_index()] = Pixel::Lit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1_solve(include_str!("day_10_sample_1.txt")), 13140);
    }
    #[test]
    fn part_2_test() {
        let output = part_2_solve(include_str!("day_10_sample_1.txt"));
        let result: Vec<&str> = output.lines().collect();
        let answer: Vec<&str> = include_str!("day_10_answer_2.txt").lines().collect();
        assert_eq!(result, answer);
    }
}