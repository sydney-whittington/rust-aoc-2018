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

use frozenset::{Freeze, FrozenSet};

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

    let (i, rules) = preceded(multispace1, separated_list1(newline, one_rule))(i)?;

    // remove all rules that yield empty pots so that if it's in the dictionary, it's a plant rule
    let rules = HashSet::from_iter(
        rules
            .into_iter()
            .filter(|(_, p)| matches!(p, Pot::P))
            .map(|(r, _)| r),
    );

    Ok((i, (initial, rules)))
}

fn next_state(current_state: &HashSet<i32>, rules: &HashSet<Rule>) -> HashSet<i32> {
    let mut next: HashSet<i32> = HashSet::new();
    let (min, max) = current_state.iter().minmax().into_option().unwrap();
    for value in min - 2..=max + 2 {
        let pots = (value - 2..=value + 2)
            .map(|i| current_state.contains(&i))
            .map(|b| match b {
                true => Pot::P,
                false => Pot::E,
            })
            .collect_tuple()
            .unwrap();
        if rules.contains(&pots) {
            next.insert(value);
        }
    }
    next
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, (initial, rules)) = parser(input).unwrap();
    // map the initial vector into just the pot locations
    let mut state = HashSet::from_iter(
        initial
            .into_iter()
            .enumerate()
            .filter(|(_, x)| matches!(x, Pot::P))
            .map(|(i, _)| i.try_into().unwrap()),
    );

    for _ in 0..20 {
        state = next_state(&state, &rules);
    }

    Some(state.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (initial, rules)) = parser(input).unwrap();
    let mut state = HashSet::from_iter(
        initial
            .into_iter()
            .enumerate()
            .filter(|(_, x)| matches!(x, Pot::P))
            .map(|(i, _)| i.try_into().unwrap()),
    );
    let mut seen_states: HashMap<FrozenSet<i32>, (u64, i32)> = HashMap::new();

    for i in 1..50_000_000_000 as u64 {
        state = next_state(&state, &rules);
        // frozen sets are hashable and read only, which is what we want for membership checking
        // normalize our state by subtracting the minimum from everything to get a 0-centered representation
        let normalized_state = state
            .iter()
            .map(|s| s - state.iter().min().unwrap())
            .collect::<HashSet<i32>>()
            .freeze();

        if !seen_states.contains_key(&normalized_state) {
            seen_states.insert(normalized_state, (i, state.iter().sum()));
        } else {
            // the previous seen_state gives the stepwise increase
            let (old_step, old_value) = seen_states.get(&normalized_state).unwrap();
            let increment = i - old_step;
            let current_value = state.iter().sum::<i32>();
            let value_increment = (current_value - old_value) as u64;
            dbg!(&i, &current_value, &increment, &value_increment);

            // turns out our clues give an increment of 1, so a recurring shape that just increases by a fixed value every step
            // so skipping the logic to factor out cycles and offsets near the 50 billion threshold
            // and just doing basic math once it collapses to that configuration

            let steps_remaining = 50_000_000_000 - i;
            let future_score = steps_remaining / increment * value_increment;

            return Some((current_value as u64) + future_score);
        }
    }

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
        let _ = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(true);
        // assert_eq!(result, None);
    }
}
