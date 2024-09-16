advent_of_code::solution!(11);

use std::{cmp::max, collections::HashMap, fmt, str::FromStr};

use advent_of_code::Output;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Output3(u32, u32, u32);

impl fmt::Display for Output3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

fn cell_power(x: u32, y: u32, serial: u32) -> i32 {
    let rack_id = x + 10;
    let power = rack_id * y;
    let power = power + serial;
    let power = power * rack_id;
    let powerstring = power.to_string();
    let hundreds = powerstring.get(powerstring.len() - 3..powerstring.len() - 2);
    let power = hundreds.map(|c| i32::from_str(c).unwrap()).unwrap_or(0);

    power - 5
}

fn powersquare(x: u32, y: u32, grid: &HashMap<(u32, u32), i32>, size: u32) -> i32 {
    (x..x + size)
        .cartesian_product(y..y + size)
        .map(|k| grid.get(&k).unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<Output<u32>> {
    let serial = u32::from_str(input.trim_end()).unwrap();
    let powergrid: HashMap<(u32, u32), i32> = HashMap::from_iter(
        (1..=300)
            .cartesian_product(1..=300)
            .map(|(x, y)| ((x, y), cell_power(x, y, serial))),
    );

    let (x, y) = ((1..=298)
        .cartesian_product(1..=298)
        .max_by_key(|(x, y)| powersquare(*x, *y, &powergrid, 3)))
    .unwrap();
    Some(Output(x, y))
}

pub fn part_two(input: &str) -> Option<Output3> {
    let serial = u32::from_str(input.trim_end()).unwrap();
    let powergrid: HashMap<(u32, u32), i32> = HashMap::from_iter(
        (1..=300)
            .cartesian_product(1..=300)
            .map(|(x, y)| ((x, y), cell_power(x, y, serial))),
    );

    // today i learned about https://en.wikipedia.org/wiki/Summed-area_table
    let mut summed_area: HashMap<(u32, u32), i32> = HashMap::new();
    // sorted by minimum sum which will give numbers closer to the top left corner first
    for (x, y) in (1..=300)
        .cartesian_product(1..=300)
        .sorted_by_key(|(x, y)| x + y)
    {
        summed_area.insert(
            (x, y),
            powergrid.get(&(x, y)).unwrap()
                + summed_area.get(&(x, y - 1)).unwrap_or(&0)
                + summed_area.get(&(x - 1, y)).unwrap_or(&0)
                - summed_area.get(&(x - 1, y - 1)).unwrap_or(&0),
        );
    }

    let mut best = Output3(1, 1, 1);
    let mut best_power = 0;

    for (x, y) in (1..=300).cartesian_product(1..=300) {
        for s in 1..(300 - max(x, y)) {
            let power = summed_area.get(&(x, y)).unwrap()
                + summed_area.get(&(x + s, y + s)).unwrap()
                - summed_area.get(&(x, y + s)).unwrap()
                - summed_area.get(&(x + s, y)).unwrap();
            if power > best_power {
                best_power = power;
                // not totally sure why we're off by one here but meh
                best = Output3(x + 1, y + 1, s);
            }
        }
    }
    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(Output(33, 45)));
    }

    #[test]
    fn test_part_one_a() {
        let result = cell_power(3, 5, 8);
        assert_eq!(result, 4);
        let result = cell_power(122, 79, 57);
        assert_eq!(result, -5);
        let result = cell_power(217, 196, 39);
        assert_eq!(result, 0);
        let result = cell_power(101, 153, 71);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(Output(21, 61)));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(Output3(90, 269, 16)));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(Output3(232, 251, 12)));
    }
}
