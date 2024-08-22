use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Step {
    pre: char,
    post: char,
}

fn parser(i: &str) -> IResult<&str, Vec<Step>> {
    separated_list0(
        newline,
        preceded(
            tag("Step "),
            terminated(
                separated_pair(anychar, tag(" must be finished before step "), anychar)
                    .map(|(pre, post)| Step { pre, post }),
                tag(" can begin."),
            ),
        ),
    )(i)
}

fn check_next_step(graph: &HashMap<char, HashSet<char>>) -> Option<char> {
    let available_steps = graph.iter().filter(|(_, prereqs)| prereqs.len() == 0);
    let next_step = available_steps
        .sorted_by(|(step1, _), (step2, _)| step1.cmp(step2))
        .next();

    match next_step {
        Some((c, _)) => Some(*c),
        None => None,
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (_, steps) = parser(input).unwrap();

    let mut graph = HashMap::new();
    let mut step_sequence: Vec<char> = Vec::new();

    for step in steps {
        graph
            .entry(step.post)
            .and_modify(|c: &mut HashSet<char>| {
                c.insert(step.pre);
            })
            .or_insert(HashSet::from([step.pre]));
        // make sure there's a key for things that don't have prereqs
        graph.entry(step.pre).or_insert(HashSet::new());
    }

    while let Some(c) = check_next_step(&graph) {
        step_sequence.push(c);
        graph.remove(&c);
        for step in graph.iter_mut() {
            step.1.remove(&c);
        }
    }

    Some(step_sequence.iter().collect())
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
        assert_eq!(result, Some("CABDFE".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
