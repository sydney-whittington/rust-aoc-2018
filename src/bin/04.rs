advent_of_code::solution!(4);

use std::{collections::HashMap, iter, ops::Range};

use chrono::{NaiveDateTime, Timelike};

use advent_of_code::number;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::newline,
    combinator::value,
    multi::separated_list0,
    sequence::delimited,
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

fn shift_minutes(start: NaiveDateTime, finish: NaiveDateTime) -> Range<u32> {
    let start = start.minute();
    let finish = finish.minute();

    // TODO wrapping?
    start..finish
}

fn timing(i: &Vec<Observation>) -> HashMap<u32, HashMap<u32, u32>> {
    // is there a way to initialize these better? we know it will be written to before reading
    let mut current_guard = 0;
    let mut nap_start =
        NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
    let mut shifts: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    for obs in i {
        match obs.event {
            Event::Begin(g) => current_guard = g,
            Event::Sleep => nap_start = obs.timestamp,
            Event::Wake => {
                shifts
                    .entry(current_guard)
                    // add one to every entry in the current shift
                    .and_modify(|h| {
                        shift_minutes(nap_start, obs.timestamp).for_each(|m| {
                            h.entry(m).and_modify(|e| *e += 1).or_insert(1);
                        })
                    })
                    // or start every entry at one for a new guard
                    .or_insert(HashMap::from_iter(
                        shift_minutes(nap_start, obs.timestamp).zip(iter::once(1).cycle()),
                    ));
            }
        }
    }

    shifts
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut observations) = parser(input).unwrap();
    observations.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
    let shifts = timing(&observations);

    let (guard, schedule) = shifts
        .iter()
        .max_by_key(|(_, minutes)| minutes.values().sum::<u32>())
        .unwrap();
    let (sleepy_minute, _) = schedule
        .iter()
        .max_by_key(|&(_minute, count)| count)
        .unwrap();

    Some(guard * sleepy_minute)
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
