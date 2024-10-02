use std::collections::HashMap;

use advent_of_code::{coord_parse_usize, number_usize, Coordinate};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(22);

#[derive(Debug)]
struct Cave {
    depth: usize,
    target: Coordinate<usize>,
}

fn parser(i: &str) -> IResult<&str, Cave> {
    let (i, depth) = terminated(preceded(tag("depth: "), number_usize), newline)(i)?;
    let (i, target) = preceded(tag("target: "), coord_parse_usize)(i)?;

    Ok((i, Cave { depth, target }))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, cave) = parser(input).unwrap();
    dbg!(&cave);

    let mut cavern: HashMap<(usize, usize), usize> = HashMap::new();
    // sorted by minimum sum which will give numbers closer to the top left corner first
    for (x, y) in (0..=cave.target.left)
        .cartesian_product(0..=cave.target.top)
        .sorted_by_key(|(x, y)| x + y)
    {
        dbg!(&x, &y, &cavern);
        let geologic_index = match (x, y) {
            (0, 0) => 0,
            (x, y) if x == cave.target.left && y == cave.target.top => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => cavern.get(&(x - 1, y)).unwrap() * cavern.get(&(x, y - 1)).unwrap(),
        };
        cavern.insert((x, y), geologic_index);
    }

    Some(
        (1..=cave.target.left)
            .cartesian_product(1..=cave.target.top)
            .map(|(x, y)| ((cavern.get(&(x, y)).unwrap() + cave.depth) % 20183) % 3)
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
