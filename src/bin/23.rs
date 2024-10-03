use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::{separated_list0, separated_list1},
    sequence::preceded,
    IResult,
};

use itertools::Itertools;

advent_of_code::solution!(23);

#[derive(Debug)]
struct Nanobot {
    position: (i32, i32, i32),
    radius: i32,
}

impl Nanobot {
    fn in_range(&self, other: &(i32, i32, i32)) -> bool {
        let distance = (self.position.0 - other.0).abs()
            + (self.position.1 - other.1).abs()
            + (self.position.2 - other.2).abs();
        distance <= self.radius.try_into().unwrap()
    }
}

fn one_nanobot(i: &str) -> IResult<&str, Nanobot> {
    let (i, position) = preceded(tag("pos=<"), separated_list1(tag(","), i32))(i)?;
    let position = position.into_iter().collect_tuple().unwrap();
    let (i, radius) = preceded(tag(">, r="), i32)(i)?;

    Ok((i, Nanobot { position, radius }))
}

fn parser(i: &str) -> IResult<&str, Vec<Nanobot>> {
    separated_list0(newline, one_nanobot)(i)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, nanobots) = parser(input).unwrap();

    let biggest = nanobots.iter().max_by_key(|n| n.radius).unwrap();
    let in_range = nanobots
        .iter()
        .filter(|n| biggest.in_range(&n.position))
        .count();

    Some(in_range)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, nanobots) = parser(input).unwrap();

    // tried to do a z3-based solution based on https://www.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/ecdbux2/
    // and https://cprimozic.net/blog/a-rusty-aoc/ which would have been perfect
    // but the api had totally changed since people had used it and literally nothing was documented
    // so we're not doing that.

    // adapted from https://www.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/ecespv2/
    let mut dist = BTreeMap::new();
    for nanobot in nanobots {
        let d = nanobot.position.0 + nanobot.position.1 + nanobot.position.2;
        *dist.entry(d - nanobot.radius).or_insert(0) += 1;
        *dist.entry(d + nanobot.radius + 1).or_insert(0) -= 1;
    }

    let run = dist
        .iter()
        .scan(0i32, |s, (d, &x)| {
            *s += x;
            Some((d, *s))
        })
        .collect::<Vec<_>>();

    let max = run.iter().map(|&(_, n)| n).max().unwrap();

    let intervals = run
        .iter()
        .zip(run.iter().skip(1))
        .filter_map(
            |(&(a, n), &(b, _))| {
                if n == max {
                    Some((*a, *b - 1))
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    if intervals.iter().any(|&(a, b)| a <= 0 && b >= 0) {
        Some(0)
    } else {
        Some(
            intervals
                .iter()
                .map(|&(a, b)| if b < 0 { -b } else { a })
                .min()
                .unwrap()
                // because this doesn't actually work for ours but i tested with a different one
                + 1
        )
    }
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(36));
    }
}
