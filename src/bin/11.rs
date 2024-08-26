advent_of_code::solution!(11);

use std::{collections::HashMap, fmt, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Output(u32, u32);

// since every result type has to be formattable and our tuple isn't
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn cell_power(x: u32, y: u32, serial: u32) -> i32 {
    let rack_id = x + 10;
    let power = rack_id * y;
    let power = power + serial;
    let power = power * rack_id;
    let powerstring = power.to_string();
    let hundreds = powerstring.get(powerstring.len() - 3..powerstring.len() - 2);
    let power = hundreds
        .and_then(|c| Some(i32::from_str(c).unwrap()))
        .or_else(|| Some(0))
        .unwrap();

    power - 5
}

fn powersquare(x: u32, y: u32, grid: &HashMap<(u32, u32), i32>) -> i32 {
    (x..=x+2).cartesian_product(y..=y+2).map(|k| grid.get(&k).unwrap()).sum()
}

pub fn part_one(input: &str) -> Option<Output> {
    let serial = u32::from_str(input.trim_end()).unwrap();
    let powergrid: HashMap<(u32, u32), i32> = HashMap::from_iter(
        (1..=300)
            .cartesian_product(1..=300)
            .map(|(x, y)| ((x, y), cell_power(x, y, serial))),
    );

    let (x, y) = ((1..=298).cartesian_product(1..=298).max_by_key(|(x, y)| powersquare(*x, *y, &powergrid))).unwrap();
    Some(Output(x, y))
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
        assert_eq!(result, None);
    }
}
