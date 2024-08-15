advent_of_code::solution!(1);

use std::{cell::Cell, collections::HashSet};

use nom::{
    character::complete::{i32, newline}, multi::separated_list0, IResult
};

fn parser(i: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(newline, i32)(i)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, nums) = parser(input).unwrap();
    Some(nums.iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, nums) = parser(input).unwrap();
    let mut set = HashSet::new();
    // initial value
    set.insert(0);
    let result = Cell::new(0);

    nums.iter()
        .cycle()
        .scan(result, |state, n| {
            state.set(state.get() + n);
            Some(state.get())
        })
        .find(|n| !set.insert(*n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(0));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(-6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(0));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(10));
    }
}
