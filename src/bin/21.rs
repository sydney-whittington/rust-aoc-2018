advent_of_code::solution!(21);

use std::collections::HashMap;

use advent_of_code::{
    instructions, parse_program, Instruction, MachineState, Opcode, OperationResult, Registers,
};

instructions!(Registers);

fn advance_program(state: MachineState) -> OperationResult {
    let mut registers = state.registers;

    if let Some(instruction) = state.instructions.get(&(registers[state.pointer])) {
        print!("{}{}", &state, &instruction);
        registers = execute_instruction(registers, *instruction);
        println!(" {:?}", &registers);
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

    dbg!(&pointer, &instructions);

    let mut state = MachineState {
        registers: [2, 0, 0, 0, 0, 0],
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
