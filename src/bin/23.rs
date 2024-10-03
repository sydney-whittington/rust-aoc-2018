use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline, u32},
    multi::{separated_list0, separated_list1},
    sequence::preceded,
    IResult,
};

use itertools::Itertools;

advent_of_code::solution!(23);

#[derive(Debug)]
struct Nanobot {
    position: (i32, i32, i32),
    radius: u32,
}

impl Nanobot {
    fn in_range(&self, other: &Nanobot) -> bool {
        let distance = (self.position.0 - other.position.0).abs()
            + (self.position.1 - other.position.1).abs()
            + (self.position.2 - other.position.2).abs();
        distance <= self.radius.try_into().unwrap()
    }
}

fn one_nanobot(i: &str) -> IResult<&str, Nanobot> {
    let (i, position) = preceded(tag("pos=<"), separated_list1(tag(","), i32))(i)?;
    let position = position.into_iter().collect_tuple().unwrap();
    let (i, radius) = preceded(tag(">, r="), u32)(i)?;

    Ok((i, Nanobot { position, radius }))
}

fn parser(i: &str) -> IResult<&str, Vec<Nanobot>> {
    separated_list0(newline, one_nanobot)(i)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, nanobots) = parser(input).unwrap();

    let biggest = nanobots.iter().max_by_key(|n| n.radius).unwrap();
    let in_range = nanobots.iter().filter(|n| biggest.in_range(n)).count();

    Some(in_range)
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
