use std::collections::HashMap;

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
    let in_range = nanobots.iter().filter(|n| biggest.in_range(&n.position)).count();

    Some(in_range)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, nanobots) = parser(input).unwrap();

    let mut coordinates: HashMap<(i32, i32, i32), u32> = HashMap::new();

    for nanobot in nanobots {
        let candidate_x = nanobot.position.0-nanobot.radius..=nanobot.position.0+nanobot.radius;
        let candidate_y = nanobot.position.1-nanobot.radius..=nanobot.position.1+nanobot.radius;
        let candidate_z = nanobot.position.2-nanobot.radius..=nanobot.position.2+nanobot.radius;

        // hopefully this isn't super expensive...
        // (it totally is)
        let candidates = candidate_x.cartesian_product(candidate_y).cartesian_product(candidate_z).map(|((a, b), c)| (a, b, c));
        let good_candidates = candidates.filter(|&n| nanobot.in_range(&n));
        for candidate in good_candidates {
            coordinates.entry(candidate).and_modify(|e| *e+=1).or_insert(1);
        }
    }

    let (location, _in_range) = coordinates.iter().max_by_key(|(_, v)| **v).unwrap();

    Some(location.0.abs() + location.1.abs() + location.2.abs())
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(36));
    }
}
