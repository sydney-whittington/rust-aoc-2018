use std::{collections::HashMap, fmt, str::FromStr};

use advent_of_code::{instructions, number_usize, Instruction, Opcode};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::map_res,
    multi::{many0, separated_list0},
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(19);

type InstructionPointer = usize;
type Registers = [usize; 6];

#[derive(Debug)]
struct MachineState {
    registers: Registers,
    pointer: InstructionPointer,
    instructions: HashMap<usize, Instruction>,
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
enum OperationResult {
    Active(MachineState),
    Concluded(MachineState),
}

instructions!(Registers);

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
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

fn parser(i: &str) -> IResult<&str, (InstructionPointer, Vec<Instruction>)> {
    let (i, instruction_pointer) = terminated(preceded(tag("#ip "), number_usize), newline)(i)?;
    let (i, instructions) = many0(parse_instruction)(i)?;

    Ok((i, (instruction_pointer, instructions)))
}

fn advance_program(state: MachineState) -> OperationResult {
    let mut registers = state.registers;

    if let Some(instruction) = state.instructions.get(&(registers[state.pointer])) {
        // print!("{}{}", &state, &instruction);
        registers = execute_instruction(registers, *instruction);
        // println!(" {:?}", &registers);
        // advance the instruction pointer
        registers[state.pointer] += 1;

        // magic part 2 number
        if registers[state.pointer] == 35 {
            OperationResult::Concluded(MachineState {
                registers,
                pointer: state.pointer,
                instructions: state.instructions,
            })
        } else {
            OperationResult::Active(MachineState {
                registers,
                pointer: state.pointer,
                instructions: state.instructions,
            })
        }
    } else {
        OperationResult::Concluded(state)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, (pointer, instructions)) = parser(input).unwrap();

    let mut state = MachineState {
        registers: [0, 0, 0, 0, 0, 0],
        pointer,
        instructions: HashMap::from_iter((0..).zip(instructions)),
    };

    loop {
        match advance_program(state) {
            OperationResult::Active(s) => state = s,
            OperationResult::Concluded(s) => return Some(s.registers[0]),
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, (pointer, instructions)) = parser(input).unwrap();

    let mut state = MachineState {
        registers: [1, 0, 0, 0, 0, 0],
        pointer,
        instructions: HashMap::from_iter((0..).zip(instructions)),
    };

    // stolen from https://www.reddit.com/r/adventofcode/comments/a7j9zc/2018_day_19_solutions/ec3kfej/
    // because having to reverse engineer assembly wasn't exciting today
    let final_state = loop {
        match advance_program(state) {
            OperationResult::Active(s) => state = s,
            OperationResult::Concluded(s) => break s,
        }
    };

    println!("{}", &final_state);
    // sum of factors
    let n = final_state.registers[5];
    Some((1..(n+1)).filter(|x| n % x == 0).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
