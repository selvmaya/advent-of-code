use std::fmt::Display;
use std::ops::Range;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        println!("PART 1: {}", part_1_solve(&input));
        println!("PART 2: {}", part_2_solve(&input));
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
    let mut cpu = Cpu { x: 1, total_cycle_count: 1, state: CpuState::ReadyForInstruction, state_cycle_count: 0 };
    let mut cycles_to_read_signal_strength: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut read_cycle_signal_strength = vec![];
    for instruction in instructions {
        cpu.give_instruction(instruction);
        while let CpuState::RunningInstruction(_) = &cpu.state {
            if let Some(cycle_read) = cycles_to_read_signal_strength.first() {
                if &cpu.total_cycle_count == cycle_read {
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
    total_cycle_count: usize,
    state: CpuState,
    state_cycle_count: usize
}
impl Cpu {
    fn run_cycle(&mut self) {
        let CpuState::RunningInstruction(instruction) = &self.state else {
            panic!("Should never be able to call this function without an active instruction");
        };
        self.total_cycle_count += 1;
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
    fn read_signal_strength(&self) -> isize {
        self.total_cycle_count as isize * self.x // todo: check if cast may be invalid
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
    let mut cpu = Cpu { x: 1, total_cycle_count: 1, state: CpuState::ReadyForInstruction, state_cycle_count: 0 };
    let mut crt = Crt::default();
    let mut crt_output_buffer = vec![];
    let mut new_sprite_pos = None;
    for instruction in instructions {
        cpu.give_instruction(instruction);
        while let CpuState::RunningInstruction(_) = &cpu.state {
            crt.draw_next_pixel(new_sprite_pos);
            const CRT_DISPLAY_CYCLE_RANGE: Range<u8> = 0..CYCLES_IN_CRT_DISPLAY;
            if !CRT_DISPLAY_CYCLE_RANGE.contains(&crt.display_cycles) {
                let output = format!("{}", crt);
                crt_output_buffer.push(output);
                crt = Crt { last_sprite_pos: crt.last_sprite_pos, ..Default::default() };
            } else if crt.display_cycles / PIXELS_IN_SCANLINE == 1 {
                crt.sprite_cycles = 0;
            }

            { // UPDATE DEBUG INFO (step by step)
                println!("CRT --- display_cycle: {}; sprite_x: {}; sprite_cycles: {}", crt.display_cycles, crt.last_sprite_pos, crt.sprite_cycles);
                let scanline_index = crt.display_cycles / PIXELS_IN_SCANLINE;
                let active_scanline = crt.display[scanline_index as usize];
                let scanline_display = {
                    let mut scanline_display = active_scanline.iter().map(|pixel| format!("{}", pixel)).collect::<String>();
                    for offset in -1..=1 {
                        const INDEX_RANGE: Range<i8> = 0..(PIXELS_IN_SCANLINE as i8);
                        let index = (crt.last_sprite_pos + offset) as usize;
                        if INDEX_RANGE.contains(&crt.last_sprite_pos) {
                            let mut chars: Vec<char> = scanline_display.chars().collect();
                            chars[index] = if chars[index] == '#' { '█' } else { '▒' };
                            scanline_display = chars.into_iter().collect();
                        }
                    }
                    scanline_display
                };
                println!("LINE--| {}", scanline_display);
                println!("CPU --- cycle: {}; state: {:?}, x: {}", cpu.total_cycle_count, cpu.state, cpu.x);
            }

            cpu.run_cycle();
            new_sprite_pos = if cpu.state == CpuState::ReadyForInstruction {
                let pos = cpu.x as i8;
                const DRAW_RANGE: RangeInclusive<i8> = -1..=(PIXELS_IN_SCANLINE as i8);
                if !DRAW_RANGE.contains(&pos) {
                    panic!("draw position is {}, cpu.x register is {}", &pos, &cpu.x);
                }
                Some(pos)
            } else {
                None
            };
            
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
            Pixel::Dark => '_',
            Pixel::Lit => '#',
        })
    }
}
const PIXELS_IN_SCANLINE: u8 = 40;
const SCANLINES_IN_CRT: u8 = 6;
const CYCLES_IN_CRT_DISPLAY: u8 = PIXELS_IN_SCANLINE * SCANLINES_IN_CRT;
type Scanline = [Pixel; PIXELS_IN_SCANLINE as usize]; // 40: 0-39
type CrtDisplay = [Scanline; SCANLINES_IN_CRT as usize]; // 6: 0-5
const DEFAULT_DISPLAY: CrtDisplay = [[Pixel::Dark; 40]; 6];
struct Crt {
    display: CrtDisplay,
    display_cycles: u8, // 240: 0-239
    last_sprite_pos: i8, // 42: (-1)-40, can be 1 outside display pixels (0-39)
    sprite_cycles: usize,
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
            display: DEFAULT_DISPLAY,
            display_cycles: 0,
            last_sprite_pos: 1,
            sprite_cycles: 0,
        }
    }
}
impl Crt {
    fn draw_next_pixel(&mut self, new_sprite_position: Option<i8>) {
        if let Some(new_position) = new_sprite_position {
            const VALID_SPRITE_POSITIONS: RangeInclusive<i8> = -1..=(PIXELS_IN_SCANLINE as i8);
            assert!(VALID_SPRITE_POSITIONS.contains(&new_position));

            self.sprite_cycles = 0;
            self.last_sprite_pos = new_position;
        }

        let draw_signed_offset = match self.last_sprite_pos {
            ..=-1 => 1,
            40.. => -1,
            0 => i8::clamp(self.sprite_cycles as i8, 0, 1),
            39 => i8::clamp(-1 + self.sprite_cycles as i8, -1, 0),
            _ => i8::clamp(-1 + self.sprite_cycles as i8, -1, 1)
        };

        let pixel_index = (self.last_sprite_pos + draw_signed_offset) as u8;
        let scanline_index = self.display_cycles / PIXELS_IN_SCANLINE; // integer division is quotient
        self.display[scanline_index as usize][pixel_index as usize] = Pixel::Lit;

        // update cycle count
        self.sprite_cycles += 1;
        self.display_cycles += 1;
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
        let result = part_2_solve(include_str!("day_10_sample_1.txt"));
        println!("RESULT:\n{}\n", &result);
        let answer = include_str!("day_10_answer_2.txt");
        println!("EXPECTED:\n{}\n", &answer);
        assert_eq!(result, answer);
    }
}