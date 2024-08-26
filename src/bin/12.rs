use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace1, newline},
    combinator::value,
    multi::{count, many1, separated_list1},
    sequence::{pair, preceded},
    IResult,
};

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pot {
    P, //lant
    E, //mpty
}

impl fmt::Display for Pot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pot::P => write!(f, "#"),
            Pot::E => write!(f, "."),
        }
    }
}

type Rule = (Pot, Pot, Pot, Pot, Pot);

fn pot(i: &str) -> IResult<&str, Pot> {
    alt((value(Pot::P, char('#')), value(Pot::E, char('.'))))(i)
}

fn one_rule(i: &str) -> IResult<&str, (Rule, Pot)> {
    let (i, (head, tail)) = pair(count(pot, 5), preceded(tag(" => "), pot))(i)?;

    Ok((i, (head.into_iter().collect_tuple().unwrap(), tail)))
}

fn parser(i: &str) -> IResult<&str, (Vec<Pot>, HashSet<Rule>)> {
    let (i, initial) = preceded(tag("initial state: "), many1(pot))(i)?;
    // add extra elements to represent the infinite pots off into the distance at the start and finish
    let initial = vec![Pot::E, Pot::E]
        .into_iter()
        .chain(initial.into_iter())
        .chain(vec![Pot::E, Pot::E])
        .collect_vec();

    let (i, rules) = preceded(multispace1, separated_list1(newline, one_rule))(i)?;

    let rules = HashSet::from_iter(
        rules
            .into_iter()
            .filter(|(_, p)| matches!(p, Pot::P))
            .map(|(r, _)| r),
    );
    // remove all rules that yield empty pots so that if it's in the dictionary, it's a pot rule
    Ok((i, (initial, rules)))
}

fn next_state(current_state: &Vec<Pot>, rules: &HashSet<Rule>) -> Vec<Pot> {
    let mut next: Vec<Pot> = vec![Pot::E, Pot::E];
    // TODO: is this really the best way to do this lookup? that's a lot of derefs
    for (a, b, c, d, e) in current_state.iter().tuple_windows() {
        next.push(match rules.contains(&(*a, *b, *c, *d, *e)) {
            true => Pot::P,
            false => Pot::E,
        });
    }

    // TODO: jk it can just keep growing i guess
    next.extend(vec![Pot::E, Pot::E]);
    next
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, (mut state, rules)) = parser(input).unwrap();
    dbg!(&state, &rules);

    for _ in 0..20 {
        state = next_state(&state, &rules);
        println!("{}", state.iter().join(""));
    }

    // count pot indices after removing our placeholder ones at the front
    Some(
        state
            .iter()
            .enumerate()
            .filter(|(_, &p)| matches!(p, Pot::P))
            .map(|(i, _)| i - 1)
            .sum(),
    )
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
        assert_eq!(result, Some(325));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
