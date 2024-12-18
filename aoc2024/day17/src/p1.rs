#![doc = include_str!("../p1.md")]

use itertools::Itertools;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};
use std::ops::BitXorAssign;
#[allow(unused_imports)]
use winnow::{
    ascii::*,
    combinator::*,
    error::*,
    prelude::*,
    stream::*,
    token::*,
    {PResult, Parser},
};

#[derive(Debug, IntoPrimitive, TryFromPrimitive, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
enum ThreeBitValue {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct LiteralOperand(ThreeBitValue);
impl From<LiteralOperand> for ArbitraryUInt {
    fn from(value: LiteralOperand) -> Self {
        u8::from(value.0).into()
    }
}

#[derive(IntoPrimitive, TryFromPrimitive, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
enum TwoBitValue {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}
#[derive(Copy, Clone, Eq, PartialEq)]
enum ComboOperand {
    /// 0..=3: Literal values
    Literal(TwoBitValue),
    /// 4: Value of register `A`
    RegA,
    /// 5: Value of register `B`
    RegB,
    /// 6: Value of register `C`
    RegC,
    /// 7: "reserved and does not appear in valid programs" (can be ignored?)
    Reserved,
}
impl From<ThreeBitValue> for ComboOperand {
    fn from(value: ThreeBitValue) -> Self {
        match value {
            n @ (ThreeBitValue::Zero
            | ThreeBitValue::One
            | ThreeBitValue::Two
            | ThreeBitValue::Three) => Self::Literal(
                u8::from(n)
                    .try_into()
                    .expect("0..=3 fits inside TwoBitValue"),
            ),
            ThreeBitValue::Four => Self::RegA,
            ThreeBitValue::Five => Self::RegB,
            ThreeBitValue::Six => Self::RegC,
            ThreeBitValue::Seven => Self::Reserved,
        }
    }
}

/// # OpCode values
/// All arithmetic operations, as well as XOR, all have "closure", especially XOR,
/// because for all intents and purposes the outputs of these can be stored in arbitrary registers.
///
/// In order, 0..=7.
#[repr(u8)]
#[derive(Debug, TryFromPrimitive, Copy, Clone, Eq, PartialEq)]
enum OpCode {
    /// division with register `A` and a combo operand `O` : `A / 2.pow(O) -> A`
    Adv = 0,
    /// bitwise XOR of `B` and a literal operand `L`: `B^L -> B`
    Bxl = 1,
    /// combo operand `O` modulo `8` (keeping lowest 3 bits): `O mod 8 -> B`
    Bst = 2,
    /// noop if `A==0`, else set ins_ptr `I` (jump) to literal `L`: `if A==0 { L -> I }`
    /// if this happens, `I` is not incremented.
    /// jumps can be odd, meaning that `ins_ptr` isn't necessarily an even number
    Jnz = 3,
    /// bitwise XOR of `B` and `C` stored to `B` (still reads operand): `B^C -> B`
    Bxc = 4,
    /// combo `O` modulus 8, printed: `print(O mod 8)`
    Out = 5,
    /// division like `Adv` but written to `B` (src is still `A`): `A / 2.pow(O) -> B`
    Bdv = 6,
    /// division like `Adv` but written to `C` (src is still `A`): `A / 2.pow(O) -> C`
    Cdv = 7,
}
impl From<ThreeBitValue> for OpCode {
    fn from(value: ThreeBitValue) -> Self {
        Into::<u8>::into(value)
            .try_into()
            .expect("same variant count")
    }
}
type ArbitraryUInt = usize;
#[derive(Default)]
struct Register(ArbitraryUInt);
#[derive(Default)]
struct Program(Vec<ThreeBitValue>);
#[derive(Default)]
struct ProgramOutput(Vec<ThreeBitValue>);
impl Display for ProgramOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|&v| u8::from(v)).join(","))
    }
}
#[derive(Default)]
struct InstructionPtr(ArbitraryUInt);
impl InstructionPtr {
    fn increment(&mut self) {
        self.0 += 2;
    }
    fn set(&mut self, literal: LiteralOperand) {
        self.0 = literal.0 as ArbitraryUInt
    }
}
#[derive(Default)]
struct Computer {
    a: Register,
    b: Register,
    c: Register,
    program: Program,
    ins_ptr: InstructionPtr,
    output: ProgramOutput,
}
impl Display for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // print instruction pointer above program
        writeln!(
            f,
            "         {}v-- IPTR={}",
            "  ".repeat(self.ins_ptr.0),
            self.ins_ptr.0
        )?;
        // print program
        writeln!(
            f,
            "Program: {}",
            self.program.0.iter().map(|&v| u8::from(v)).join(",")
        )?;
        // print state of registers
        writeln!(f, "Register A: {}", self.a.0)?;
        writeln!(f, "Register B: {}", self.b.0)?;
        write!(f, "Register C: {}", self.c.0)
    }
}
impl Computer {
    fn calc_combo(&self, operand: ThreeBitValue) -> ArbitraryUInt {
        match ComboOperand::from(operand) {
            ComboOperand::Literal(n) => u8::from(n) as ArbitraryUInt,
            ComboOperand::RegA => self.a.0,
            ComboOperand::RegB => self.b.0,
            ComboOperand::RegC => self.c.0,
            ComboOperand::Reserved => unreachable!("is not supposed to appear for valid programs"),
        }
    }
    fn run(mut self) -> Self {
        eprintln!("Starting program with state:\n{}\n", self);
        while let Some((opcode, operand_value)) = self.next_instruction() {
            // not in order, but grouped in relation
            match opcode {
                // conditional jump
                OpCode::Jnz if self.a.0 != 0 => self.ins_ptr.set(LiteralOperand(operand_value)),
                // all other branches
                non_jmp @ (OpCode::Jnz
                | OpCode::Adv
                | OpCode::Bdv
                | OpCode::Cdv
                | OpCode::Bxl
                | OpCode::Bxc
                | OpCode::Bst
                | OpCode::Out) => {
                    self.ins_ptr.increment();
                    match non_jmp {
                        OpCode::Jnz => { /* do nothing */ }
                        // division
                        OpCode::Adv => {
                            self.a.0 /= 2usize.pow(self.calc_combo(operand_value) as u32)
                        }
                        OpCode::Bdv => {
                            self.b.0 = self.a.0 / 2usize.pow(self.calc_combo(operand_value) as u32)
                        }
                        OpCode::Cdv => {
                            self.c.0 = self.a.0 / 2usize.pow(self.calc_combo(operand_value) as u32)
                        }
                        // bitwise XOR
                        OpCode::Bxl => self.b.0 ^= usize::from(LiteralOperand(operand_value)),
                        OpCode::Bxc => self.b.0.bitxor_assign(self.c.0),
                        // modulo 8
                        OpCode::Bst => self.b.0 = self.calc_combo(operand_value) % 8,
                        OpCode::Out => self.output.0.push(
                            u8::try_from(self.calc_combo(operand_value) % 8usize)
                                .ok()
                                .and_then(|v| v.try_into().ok())
                                .expect("v % 8 < 8"),
                        ),
                    };
                }
            }
            eprintln!("{}", self);
        }
        self
    }

    fn next_instruction(&self) -> Option<(OpCode, ThreeBitValue)> {
        self.program.0.get(self.ins_ptr.0).and_then(|&x| {
            self.program
                .0
                .get(self.ins_ptr.0 + 1)
                .map(|&y| (OpCode::from(x), y))
        })
    }
}

fn main() {
    util::DayInput::find::<17>().solve_with(solve);
}

fn parse_computer(input: &mut &str) -> PResult<Computer> {
    separated_pair(
        (
            terminated(parse_register, line_ending),
            terminated(parse_register, line_ending),
            terminated(parse_register, line_ending),
        ),
        line_ending,
        parse_program,
    )
    .map(|((a, b, c), program)| Computer {
        a,
        b,
        c,
        program,
        ins_ptr: InstructionPtr(0),
        output: ProgramOutput(Vec::new()),
    })
    .parse_next(input)
}
fn parse_register(input: &mut &str) -> PResult<Register> {
    preceded(
        ("Register ", alpha1, ": "),
        dec_uint.map(|n: ArbitraryUInt| Register(n)),
    )
    .parse_next(input)
}
fn parse_program(input: &mut &str) -> PResult<Program> {
    preceded(
        "Program: ",
        separated(
            0..,
            dec_uint.try_map(|n: u8| ThreeBitValue::try_from(n)),
            ',',
        ),
    )
    .map(|p: Vec<ThreeBitValue>| Program(p))
    .parse_next(input)
}

fn solve(input: impl AsRef<str>) -> ProgramOutput {
    eprintln!("Running with input:\n{}", input.as_ref());
    let computer = parse_computer
        .parse_next(&mut input.as_ref())
        .expect("parsable");
    eprintln!("Parsed!\n");
    computer.run().output
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<Vec<ArbitraryUInt>> for Program {
        fn from(value: Vec<ArbitraryUInt>) -> Self {
            Program(
                value
                    .into_iter()
                    .map(|v| (v as u8).try_into().expect("assume correct input"))
                    .collect(),
            )
        }
    }
    impl From<Vec<ArbitraryUInt>> for ProgramOutput {
        fn from(value: Vec<ArbitraryUInt>) -> Self {
            ProgramOutput(
                value
                    .into_iter()
                    .map(|v| (v as u8).try_into().expect("assume correct input"))
                    .collect(),
            )
        }
    }

    /// If register C contains 9, the program 2,6 would set register B to 1.
    #[test]
    fn register_write() {
        assert_eq!(
            Computer {
                c: Register(9),
                program: Program::from(vec![2, 6]),
                ..Default::default()
            }
            .run()
            .b
            .0,
            1
        );
    }

    /// If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn register_with_program_output() {
        assert_eq!(
            Computer {
                a: Register(10),
                program: Program::from(vec![5, 0, 5, 1, 5, 4]),
                ..Default::default()
            }
            .run()
            .output
            .0,
            ProgramOutput::from(vec![0, 1, 2]).0
        );
    }

    /// If register A contains 2024,
    /// the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn register_write_with_program_output() {
        let computer = Computer {
            a: Register(2024),
            program: Program::from(vec![0, 1, 5, 4, 3, 0]),
            ..Default::default()
        }
        .run();
        assert_eq!(
            computer.output.0,
            ProgramOutput::from(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]).0
        );
        assert_eq!(computer.a.0, 0);
    }

    /// If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn register_write_2() {
        assert_eq!(
            Computer {
                b: Register(29),
                program: Program::from(vec![1, 7]),
                ..Default::default()
            }
            .run()
            .b
            .0,
            26
        );
    }

    /// If register B contains 2024 and register C contains 43690,
    /// the program 4,0 would set register B to 44354.
    //    #[ignore]
    #[test]
    fn multi_register_write() {
        assert_eq!(
            Computer {
                b: Register(2024),
                c: Register(43690),
                program: Program::from(vec![4, 0]),
                ..Default::default()
            }
            .run()
            .b
            .0,
            44354
        );
    }

    #[test]
    fn example_solvable() {
        const EXAMPLE: &str = include_str!("EXAMPLE");
        assert_eq!(
            solve(EXAMPLE).to_string(),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }

    #[test]
    fn input_solvable() {
        assert_eq!(
            solve(include_str!("../../inputs/17")).to_string(),
            "4,3,7,1,5,3,0,5,4".to_string()
        );
    }
}
