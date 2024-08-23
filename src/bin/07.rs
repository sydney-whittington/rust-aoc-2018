use std::{
    cmp::{max, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
};

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

fn make_graph(steps: Vec<Step>) -> HashMap<char, HashSet<char>> {
    let mut graph = HashMap::new();

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

    graph
}

fn check_next_step(graph: &HashMap<char, HashSet<char>>) -> Option<char> {
    let available_steps = graph.iter().filter(|(_, prereqs)| prereqs.len() == 0);
    dbg!(available_steps.clone().collect::<Vec<_>>());
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

    let mut graph = make_graph(steps);
    let mut step_sequence: Vec<char> = Vec::new();

    while let Some(c) = check_next_step(&graph) {
        step_sequence.push(c);
        graph.remove(&c);
        for step in graph.values_mut() {
            step.remove(&c);
        }
    }

    Some(step_sequence.iter().collect())
}

fn dispatch_job(c: &char, start_time: u32) -> u32 {
    // to_digit gives 0-indexed but numerals 0-9 are the values 0-9, so offset by minus 9
    char::to_digit(*c, 36).unwrap() - 9 + 0 + start_time
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, steps) = parser(input).unwrap();

    let mut graph = make_graph(steps);
    let mut start_times = HashMap::new();

    // settable number of workers
    let mut workers = BinaryHeap::from_iter(iter::repeat(Reverse(None)).take(2));
    let mut task_completions = Vec::new();

    // stubbornly not just looping over timesteps
    loop {
        while let Some(c) = check_next_step(&graph) {
            let new_job = match workers.pop().unwrap() {
                Reverse(Some(t)) => dispatch_job(&c, max(t, *start_times.entry(c).or_insert(0))),
                Reverse(None) => dispatch_job(&c, *start_times.entry(c).or_insert(0)),
            };
            workers.push(Reverse(Some(new_job)));
            dbg!(c, new_job);

            // remove it from tasks to be done
            graph.remove(&c);
            // but save the completion for when we don't have something else to launch now
            task_completions.push((c, new_job));
        }
        // finish the next task
        task_completions.sort_by_key(|t| t.1);
        match task_completions.pop() {
            Some((finished, next_job)) => {
                for (step, deps) in graph.iter_mut() {
                    // and remove it from its dependencies, giving them a new possible start time
                    if deps.contains(&finished) {
                        start_times
                            .entry(*step)
                            .and_modify(|t| *t = max(*t, next_job))
                            .or_insert(next_job);
                        deps.remove(&finished);
                    }
                }
            }
            // get the min since we've got a minheap and we want the biggest
            None => return workers.iter().min().unwrap().0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some("CABDFE".to_string()));
    // }

    #[test]
    fn test_part_two() {
        // needs 2 workers instead of 5 and an offset of 0 instead of 60 for job duration
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
        // assert!(true);
    }

    #[test]
    fn test_letter_value_fast() {
        assert_eq!(dispatch_job(&'a', 0), 1);
        assert_eq!(dispatch_job(&'z', 0), 26);
        assert_eq!(dispatch_job(&'a', 10), 11);
        assert_eq!(dispatch_job(&'z', 10), 36);
    }

    // #[test]
    // fn test_letter_value() {
    //     assert_eq!(dispatch_job(&'a', 0), 61);
    //     assert_eq!(dispatch_job(&'z', 0), 86);
    //     assert_eq!(dispatch_job(&'a', 10), 71);
    //     assert_eq!(dispatch_job(&'z', 10), 96);
    // }
}
