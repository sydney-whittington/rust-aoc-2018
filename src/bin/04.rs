advent_of_code::solution!(4);

use std::borrow::Borrow;

use chrono::{format::parse, NaiveDateTime};

use advent_of_code::number;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, newline},
    combinator::{map_res, value},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub enum Event {
    Begin(u32),
    Sleep,
    Wake,
}

#[derive(Debug)]
pub struct Observation {
    timestamp: NaiveDateTime,
    event: Event,
}

fn parse_begin(i: &str) -> IResult<&str, Event> {
    let (i, guard) = delimited(tag("Guard #"), number, tag(" begins shift"))(i)?;

    Ok((i, Event::Begin(guard)))
}

fn one_entry(i: &str) -> IResult<&str, Observation> {
    let (i, timestamp) = delimited(tag("["), is_not("]"), tag("] "))(i)?;
    let timestamp = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M").unwrap();

    let (i, event) = alt((
        parse_begin,
        value(Event::Sleep, tag("falls asleep")),
        value(Event::Wake, tag("wakes up")),
    ))(i)?;
    Ok((i, Observation { timestamp, event }))
}

fn parser(i: &str) -> IResult<&str, Vec<Observation>> {
    separated_list0(newline, one_entry)(i)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, observations) = parser(input).unwrap();
    None
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
        assert_eq!(result, Some(240));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
