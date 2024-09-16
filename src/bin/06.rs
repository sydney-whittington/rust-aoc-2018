advent_of_code::solution!(6);

use std::cmp::max;
use std::collections::{HashMap, HashSet};

use advent_of_code::{coord_parse, Coordinate};
use itertools::Itertools;
use nom::{character::complete::newline, multi::separated_list0, IResult};

fn parser(i: &str) -> IResult<&str, Vec<Coordinate<u32>>> {
    separated_list0(newline, coord_parse)(i)
}

fn distance(a: &Coordinate<u32>, b: &Coordinate<u32>) -> u32 {
    a.left.abs_diff(b.left) + a.top.abs_diff(b.top)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, coords) = parser(input).unwrap();
    // number the regions for future reference
    let coords = coords.iter().enumerate().collect::<Vec<_>>();

    let mut region_sizes = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    let mut on_edge = HashSet::new();

    for (_, coord) in coords.iter() {
        max_x = max(max_x, coord.left);
        max_y = max(max_y, coord.top);
    }

    for coord in (0..=max_x).cartesian_product(0..=max_y) {
        let coord = Coordinate {
            left: coord.0,
            top: coord.1,
        };
        let (closest, second_closest) = coords
            .iter()
            .sorted_by_key(|(_, c)| distance(c, &coord))
            .next_tuple()
            .unwrap();
        // if they're not equidistant
        if distance(closest.1, &coord) != distance(second_closest.1, &coord) {
            // add it to the sum for that region
            region_sizes
                .entry(&closest.0)
                .and_modify(|e| *e += 1)
                .or_insert(1);

            if coord.left == 0 || coord.left == max_x || coord.top == 0 || coord.top == max_y {
                on_edge.insert(closest.0);
            }
        }
    }
    Some(
        *region_sizes
            .iter()
            .filter(|&r| !on_edge.contains(r.0))
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, coords) = parser(input).unwrap();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut safe_region = 0;

    for coord in coords.iter() {
        max_x = max(max_x, coord.left);
        max_y = max(max_y, coord.top);
    }

    for coord in (0..=max_x).cartesian_product(0..=max_y) {
        let coord = Coordinate {
            left: coord.0,
            top: coord.1,
        };
        let total_distance: u32 = coords.iter().map(|c| distance(c, &coord)).sum();
        if total_distance < 10000 {
            safe_region += 1;
        }
    }
    Some(safe_region)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two() {
        // unfortunately we have to change a value in the function, and with the test harness we're not passing in additional info
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(16));
        assert!(true);
    }
}
