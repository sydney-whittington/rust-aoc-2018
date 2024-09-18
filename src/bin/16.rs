use std::collections::{HashMap, HashSet};

use advent_of_code::{instructions, number_usize, Instruction, Opcode};
use enum_iterator::{all, cardinality};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, terminated},
    IResult,
};

advent_of_code::solution!(16);

type Registers = [usize; 4];

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

fn parse_instruction(i: &str) -> IResult<&str, UnknownInstruction> {
    let (i, instruction) = terminated(separated_list0(tag(" "), number_usize), newline)(i)?;
    if instruction.len() != 4 {
        dbg!(&i);
    }

    Ok((
        i,
        UnknownInstruction {
            opcode: instruction[0],
            input1: instruction[1],
            input2: instruction[2],
            output: instruction[3],
        },
    ))
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

    let (i, instruction) = parse_instruction(i)?;

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
            instruction,
            after,
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Capture>> {
    separated_list0(newline, parse_capture)(i)
}

fn parser_part_two(i: &str) -> IResult<&str, Vec<UnknownInstruction>> {
    many0(parse_instruction)(i)
}

instructions!(Registers);

fn possibilities(capture: &Capture) -> HashSet<Opcode> {
    let mut possibilities = HashSet::new();
    for opcode in all::<Opcode>() {
        let output = execute_instruction(
            capture.before,
            Instruction {
                opcode,
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
pub fn part_one(input: &str) -> Option<usize> {
    let (_, captures) = parser(input).unwrap();

    Some(
        captures
            .iter()
            .filter(|&c| possibilities(c).len() >= 3)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (remainder, captures) = parser(input).unwrap();
    let (_, instructions) = parser_part_two(remainder.trim_start()).unwrap();

    // what opcode could each number be?
    let mut guesses: HashMap<usize, HashSet<Opcode>> =
        captures
            .iter()
            .fold(HashMap::new(), |mut lookups, capture| {
                lookups
                    .entry(capture.instruction.opcode)
                    .and_modify(|s| {
                        let current_possiblities = possibilities(capture);
                        s.retain(|item| current_possiblities.contains(item));
                    })
                    .or_insert(possibilities(capture));
                lookups
            });

    let mut opcode_mappings: HashMap<usize, Opcode> = HashMap::new();

    // process of elimination to find each one with only one option
    for _ in 0..cardinality::<Opcode>() {
        let (number, name_set) = guesses
            .iter()
            .find(|a| a.1.len() == 1)
            .expect("incomplete information");
        let name = *name_set.iter().next().unwrap();

        opcode_mappings.insert(*number, name);
        guesses.remove(&number.clone());
        for (_, possiblities) in guesses.iter_mut() {
            possiblities.remove(&name);
        }
    }

    // finally, run the instructions
    let mut registers = [0, 0, 0, 0];
    for instruction in instructions {
        let decoded = Instruction {
            opcode: opcode_mappings[&instruction.opcode],
            input1: instruction.input1,
            input2: instruction.input2,
            output: instruction.output,
        };

        registers = execute_instruction(registers, decoded);
    }

    Some(registers[0])
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
