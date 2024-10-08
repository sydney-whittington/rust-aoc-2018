use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, RandomState},
};

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::{separated_list0, separated_list1},
    IResult,
};

use itertools::Itertools;
advent_of_code::solution!(25);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct FourD(i32, i32, i32, i32);

fn distance(a: &FourD, b: &FourD) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn parse_fourd(i: &str) -> IResult<&str, FourD> {
    let (i, coords) = separated_list1(tag(","), i32)(i)?;
    let (&x, &y, &z, &q) = coords.iter().collect_tuple().unwrap();

    Ok((i, FourD(x, y, z, q)))
}

fn parser(i: &str) -> IResult<&str, Vec<FourD>> {
    separated_list0(newline, parse_fourd)(i)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, coords) = parser(input).unwrap();

    let mut constellations = HashMap::new();
    constellations.insert(0, HashSet::from([coords[0]]));

    for coord in &coords[1..] {
        let neighbors = constellations
            .iter()
            .filter(|(_, c)| c.iter().any(|s| distance(coord, s) <= 3))
            .collect::<Vec<_>>();

        // new constellation
        if neighbors.is_empty() {
            constellations.insert(
                constellations.keys().max().unwrap() + 1,
                HashSet::from([*coord]),
            );
        }
        // part of an existing constellation
        else if neighbors.len() == 1 {
            constellations.entry(*neighbors[0].0).and_modify(|c| {
                c.insert(*coord);
            });
        }
        // joining multiple constellations
        else {
            let marked_for_death = neighbors.iter().map(|(&i, _)| i).collect::<Vec<_>>();
            let mut new_constellation: HashSet<FourD, RandomState> =
                HashSet::from_iter(neighbors.iter().flat_map(|(_, n)| n.iter()).copied());
            new_constellation.insert(*coord);
            constellations.insert(constellations.keys().max().unwrap() + 1, new_constellation);

            marked_for_death.into_iter().for_each(|i| {
                constellations.remove(&i);
            });
        }
    }

    Some(constellations.len())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse_fourd("-1,2,2,0");
        assert_eq!(result, Ok(("", FourD(-1, 2, 2, 0))));
    }

    #[test]
    fn test_parse_2() {
        let result = parser("-1,2,2,0\n0,0,2,-2");
        assert_eq!(
            result,
            Ok(("", vec![FourD(-1, 2, 2, 0), FourD(0, 0, 2, -2)]))
        );
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_d() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
