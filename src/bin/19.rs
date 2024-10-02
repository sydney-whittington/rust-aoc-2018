use std::collections::HashMap;

use advent_of_code::{
    instructions, parse_program, Instruction, MachineState, Opcode, OperationResult, Registers,
};

advent_of_code::solution!(19);

instructions!(Registers);

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
    let (_, (pointer, instructions)) = parse_program(input).unwrap();

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
    let (_, (pointer, instructions)) = parse_program(input).unwrap();

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
    Some((1..(n + 1)).filter(|x| n % x == 0).sum())
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
        // no part two test cases
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(true);
    }
}
