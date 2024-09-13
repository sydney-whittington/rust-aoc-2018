use std::collections::HashSet;

use advent_of_code::number_usize;
use enum_iterator::{all, Sequence};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list0,
    sequence::{delimited, preceded, terminated},
    IResult,
};

advent_of_code::solution!(16);

type Registers = [usize; 4];

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    input1: usize,
    input2: usize,
    output: usize,
}

#[derive(Debug)]
struct UnknownInstruction {
    opcode: usize,
    input1: usize,
    input2: usize,
    output: usize,
}

#[derive(Debug)]
struct Capture {
    before: Registers,
    instruction: UnknownInstruction,
    after: Registers,
}

#[allow(non_camel_case_types)]
#[derive(Hash, PartialEq, Eq, Sequence, Clone, Copy, Debug)]
enum Opcode {
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
    grit,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

fn execute_instruction(registers: Registers, instruction: Instruction) -> Registers {
    let mut registers = registers.clone();

    match instruction.opcode {
        Opcode::addr => {
            registers[instruction.output] =
                registers[instruction.input1] + registers[instruction.input2]
        }
        Opcode::addi => {
            registers[instruction.output] = registers[instruction.input1] + instruction.input2
        }
        Opcode::mulr => {
            registers[instruction.output] =
                registers[instruction.input1] * registers[instruction.input2]
        }
        Opcode::muli => {
            registers[instruction.output] = registers[instruction.input1] * instruction.input2
        }
        Opcode::banr => {
            registers[instruction.output] =
                registers[instruction.input1] & registers[instruction.input2]
        }
        Opcode::bani => {
            registers[instruction.output] = registers[instruction.input1] & instruction.input2
        }
        Opcode::borr => {
            registers[instruction.output] =
                registers[instruction.input1] | registers[instruction.input2]
        }
        Opcode::bori => {
            registers[instruction.output] = registers[instruction.input1] | instruction.input2
        }
        Opcode::setr => registers[instruction.output] = registers[instruction.input1],
        Opcode::seti => registers[instruction.output] = instruction.input1,
        Opcode::grit => {
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

fn possibilities(capture: &Capture) -> HashSet<Opcode> {
    let mut possibilities = HashSet::new();
    for opcode in all::<Opcode>() {
        let output = execute_instruction(
            capture.before,
            Instruction {
                opcode: opcode.clone(),
                input1: capture.instruction.input1,
                input2: capture.instruction.input2,
                output: capture.instruction.output,
            },
        );

        if output == capture.after {
            possibilities.insert(opcode);
        }
    }

    possibilities
}

fn parse_capture(i: &str) -> IResult<&str, Capture> {
    let (i, before) = terminated(
        preceded(
            tag("Before: "),
            delimited(tag("["), separated_list0(tag(", "), number_usize), tag("]")),
        ),
        newline,
    )(i)?;
    let before = before.try_into().expect("before not 4 elements");

    let (i, instruction) = terminated(separated_list0(tag(" "), number_usize), newline)(i)?;

    let (i, after) = terminated(
        preceded(
            tag("After:  "),
            delimited(tag("["), separated_list0(tag(", "), number_usize), tag("]")),
        ),
        newline,
    )(i)?;
    let after = after.try_into().expect("after not 4 elements");

    Ok((
        i,
        Capture {
            before,
            instruction: UnknownInstruction {
                opcode: instruction[0],
                input1: instruction[1],
                input2: instruction[2],
                output: instruction[3],
            },
            after,
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Capture>> {
    separated_list0(newline, parse_capture)(i)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, captures) = parser(input).unwrap();

    Some(captures.iter().filter(|&c| possibilities(c).len() >= 3).count())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_capture() {
        let capture = Capture {
            before: [3, 2, 1, 1],
            instruction: UnknownInstruction {
                opcode: 9,
                input1: 2,
                input2: 1,
                output: 2,
            },
            after: [3, 2, 2, 1],
        };
        assert_eq!(
            possibilities(&capture),
            HashSet::from([Opcode::mulr, Opcode::addi, Opcode::seti])
        )
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
