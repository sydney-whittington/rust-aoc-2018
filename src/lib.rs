pub mod template;

// Use this file to add helper functions and additional modules.

use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use std::{
    fmt::{self},
    str::FromStr,
};

use enum_iterator::Sequence;
use itertools::Itertools;
use nom::character::complete::{alpha1, newline, space1};
use nom::multi::{many0, separated_list0};
use nom::sequence::terminated;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, i32, multispace0},
    combinator::map_res,
    sequence::{preceded, separated_pair},
    IResult,
};

// https://blog.adamchalmers.com/nom-chars/
pub fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(i)
}

pub fn number_usize(i: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(i)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate<T> {
    pub left: T,
    pub top: T,
}

impl Coordinate<usize> {
    pub fn adjacents(&self) -> Vec<Coordinate<usize>> {
        Vec::from([
            Coordinate {
                left: self.left - 1,
                top: self.top - 1,
            },
            Coordinate {
                left: self.left,
                top: self.top - 1,
            },
            Coordinate {
                left: self.left + 1,
                top: self.top - 1,
            },
            Coordinate {
                left: self.left - 1,
                top: self.top,
            },
            Coordinate {
                left: self.left + 1,
                top: self.top,
            },
            Coordinate {
                left: self.left - 1,
                top: self.top + 1,
            },
            Coordinate {
                left: self.left,
                top: self.top + 1,
            },
            Coordinate {
                left: self.left + 1,
                top: self.top + 1,
            },
        ])
    }
}

pub fn coord_parse(i: &str) -> IResult<&str, Coordinate<u32>> {
    let (i, (left, top)) = separated_pair(
        preceded(multispace0, number),
        tag(","),
        preceded(multispace0, number),
    )(i)?;
    Ok((i, Coordinate { left, top }))
}

pub fn coord_parse_usize(i: &str) -> IResult<&str, Coordinate<usize>> {
    let (i, (left, top)) = separated_pair(
        preceded(multispace0, number_usize),
        tag(","),
        preceded(multispace0, number_usize),
    )(i)?;
    Ok((i, Coordinate { left, top }))
}

// this needs a different one because it's center-referenced instead of corner
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CoordinateSigned {
    pub x: i32,
    pub y: i32,
}

pub fn coord_signed_parse(i: &str) -> IResult<&str, CoordinateSigned> {
    let (i, (x, y)) = separated_pair(
        preceded(multispace0, i32),
        tag(", "),
        preceded(multispace0, i32),
    )(i)?;
    Ok((i, CoordinateSigned { x, y }))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Output<T>(pub T, pub T);

// since every result type has to be formattable and our tuple isn't
impl fmt::Display for Output<usize> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Display for Output<u32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub input1: usize,
    pub input2: usize,
    pub output: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {} {}",
            self.opcode, self.input1, self.input2, self.output
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Hash, PartialEq, Eq, Sequence, Clone, Copy, Debug)]
pub enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

impl FromStr for Opcode {
    type Err = ();

    fn from_str(input: &str) -> Result<Opcode, Self::Err> {
        match input {
            "addr" => Ok(Opcode::addr),
            "addi" => Ok(Opcode::addi),
            "mulr" => Ok(Opcode::mulr),
            "muli" => Ok(Opcode::muli),
            "banr" => Ok(Opcode::banr),
            "bani" => Ok(Opcode::bani),
            "borr" => Ok(Opcode::borr),
            "bori" => Ok(Opcode::bori),
            "setr" => Ok(Opcode::setr),
            "seti" => Ok(Opcode::seti),
            "gtir" => Ok(Opcode::gtir),
            "gtri" => Ok(Opcode::gtri),
            "gtrr" => Ok(Opcode::gtrr),
            "eqir" => Ok(Opcode::eqir),
            "eqri" => Ok(Opcode::eqri),
            "eqrr" => Ok(Opcode::eqrr),
            _ => Err(()),
        }
    }
}

#[macro_export]
// implements execute_instruction for the given register type
macro_rules! instructions {
    ($x:ty) => {
        fn execute_instruction(registers: $x, instruction: Instruction) -> Registers {
            let mut registers = registers;

            match instruction.opcode {
                Opcode::addr => {
                    registers[instruction.output] =
                        registers[instruction.input1] + registers[instruction.input2]
                }
                Opcode::addi => {
                    registers[instruction.output] =
                        registers[instruction.input1] + instruction.input2
                }
                Opcode::mulr => {
                    registers[instruction.output] =
                        registers[instruction.input1] * registers[instruction.input2]
                }
                Opcode::muli => {
                    registers[instruction.output] =
                        registers[instruction.input1] * instruction.input2
                }
                Opcode::banr => {
                    registers[instruction.output] =
                        registers[instruction.input1] & registers[instruction.input2]
                }
                Opcode::bani => {
                    registers[instruction.output] =
                        registers[instruction.input1] & instruction.input2
                }
                Opcode::borr => {
                    registers[instruction.output] =
                        registers[instruction.input1] | registers[instruction.input2]
                }
                Opcode::bori => {
                    registers[instruction.output] =
                        registers[instruction.input1] | instruction.input2
                }
                Opcode::setr => registers[instruction.output] = registers[instruction.input1],
                Opcode::seti => registers[instruction.output] = instruction.input1,
                Opcode::gtir => {
                    registers[instruction.output] =
                        (instruction.input1 > registers[instruction.input2]) as usize
                }
                Opcode::gtri => {
                    registers[instruction.output] =
                        (registers[instruction.input1] > instruction.input2) as usize
                }
                Opcode::gtrr => {
                    registers[instruction.output] =
                        (registers[instruction.input1] > registers[instruction.input2]) as usize
                }
                Opcode::eqir => {
                    registers[instruction.output] =
                        (instruction.input1 == registers[instruction.input2]) as usize
                }
                Opcode::eqri => {
                    registers[instruction.output] =
                        (registers[instruction.input1] == instruction.input2) as usize
                }
                Opcode::eqrr => {
                    registers[instruction.output] =
                        (registers[instruction.input1] == registers[instruction.input2]) as usize
                }
            }

            registers
        }
    };
}

#[derive(Debug)]
pub struct MachineState {
    pub registers: Registers,
    pub pointer: InstructionPointer,
    pub instructions: HashMap<usize, Instruction>,
}

impl fmt::Display for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ip={} ", self.registers[self.pointer])?;
        write!(
            f,
            "[{}] ",
            self.registers.iter().map(|i| i.to_string()).join(", ")
        )
    }
}
pub enum OperationResult {
    Active(MachineState),
    Concluded(MachineState),
}

pub type InstructionPointer = usize;
pub type Registers = [usize; 6];

pub fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, opcode) = terminated(map_res(alpha1, Opcode::from_str), space1)(i)?;
    let (i, instruction) = terminated(separated_list0(tag(" "), number_usize), newline)(i)?;
    if instruction.len() != 3 {
        dbg!(&i);
    }

    Ok((
        i,
        Instruction {
            opcode,
            input1: instruction[0],
            input2: instruction[1],
            output: instruction[2],
        },
    ))
}

pub fn parse_program(i: &str) -> IResult<&str, (InstructionPointer, Vec<Instruction>)> {
    let (i, instruction_pointer) = terminated(preceded(tag("#ip "), number_usize), newline)(i)?;
    let (i, instructions) = many0(parse_instruction)(i)?;

    Ok((i, (instruction_pointer, instructions)))
}
