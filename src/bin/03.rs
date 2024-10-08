advent_of_code::solution!(3);

use std::collections::{HashMap, HashSet};

use advent_of_code::{coord_parse, number, Coordinate};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub struct Size {
    pub wide: u32,
    pub tall: u32,
}

#[derive(Debug)]
pub struct Claim {
    pub id: u32,
    pub location: Coordinate<u32>,
    pub size: Size,
}

fn one_entry(i: &str) -> IResult<&str, Claim> {
    let (i, id) = preceded(tag("#"), number)(i)?;
    let (i, location) = preceded(tag(" @ "), coord_parse)(i)?;
    let (i, (wide, tall)) = preceded(tag(": "), separated_pair(number, tag("x"), number))(i)?;

    Ok((
        i,
        Claim {
            id,
            location,
            size: Size { wide, tall },
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Claim>> {
    separated_list0(newline, one_entry)(i)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, claims) = parser(input).unwrap();
    let mut fabric = HashMap::new();

    for claim in claims.iter() {
        for (x, y) in (claim.location.left..(claim.location.left + claim.size.wide))
            .cartesian_product(claim.location.top..(claim.location.top + claim.size.tall))
        {
            fabric.entry((x, y)).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    Some(fabric.values().filter(|&x| *x > 1).count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, claims) = parser(input).unwrap();
    let mut fabric: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut clean_ids = HashSet::new();

    for claim in claims.iter() {
        clean_ids.insert(claim.id);
        for (x, y) in (claim.location.left..(claim.location.left + claim.size.wide))
            .cartesian_product(claim.location.top..(claim.location.top + claim.size.tall))
        {
            // this is probably inefficient but sets are fast
            if fabric.contains_key(&(x, y)) {
                let (_, prev_owner) = fabric.get(&(x, y)).unwrap();
                clean_ids.remove(&claim.id);
                clean_ids.remove(prev_owner);
            }
            fabric
                .entry((x, y))
                .and_modify(|(x, _)| *x += 1)
                .or_insert((1, claim.id));
        }
    }

    // get the only thing left in clean_ids
    Some(clean_ids.into_iter().next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
