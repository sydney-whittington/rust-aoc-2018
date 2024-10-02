advent_of_code::solution!(21);

use std::collections::{HashMap, HashSet};

use advent_of_code::{
    instructions, parse_program, Instruction, MachineState, Opcode, OperationResult,
    Registers,
};

instructions!(Registers);

fn advance_program(state: MachineState) -> OperationResult {
    let mut registers = state.registers;

    if let Some(instruction) = state.instructions.get(&(registers[state.pointer])) {
        registers = execute_instruction(registers, *instruction);

        // advance the instruction pointer
        registers[state.pointer] += 1;

        OperationResult::Active(MachineState {
            registers,
            pointer: state.pointer,
            instructions: state.instructions,
        })
    } else {
        OperationResult::Concluded(state)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, (pointer, instructions)) = parse_program(input).unwrap();

    let mut state = MachineState {
        // determined by looking at the first number it checks against
        registers: [202209, 0, 0, 0, 0, 0],
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
        registers: [0, 0, 0, 0, 0, 0],
        pointer,
        instructions: HashMap::from_iter((0..).zip(instructions)),
    };

    let mut seen_values = HashSet::new();
    let mut last_value = 0;

    loop {
        match advance_program(state) {
            OperationResult::Active(s) => state = s,
            OperationResult::Concluded(_) => panic!("somehow halted without being set to")
        }
        if state.registers[state.pointer] == 28 {
            if !seen_values.insert(state.registers[5]) {
                return Some(last_value);
            }
            if seen_values.len() % 1000 == 0 {
                println!("seen {} unique values", seen_values.len())
            }
            last_value = state.registers[5];
        }
    }
}
