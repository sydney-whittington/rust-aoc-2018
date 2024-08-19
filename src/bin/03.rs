advent_of_code::solution!(3);

use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub struct Coordinate {
    pub left: u32,
    pub top: u32,
}

#[derive(Debug)]
pub struct Size {
    pub wide: u32,
    pub tall: u32,
}

#[derive(Debug)]
pub struct Claim {
    pub id: u32,
    pub location: Coordinate,
    pub size: Size,
}

// https://blog.adamchalmers.com/nom-chars/
fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(i)
}

fn one_entry(i: &str) -> IResult<&str, Claim> {
    let (i, id) = preceded(tag("#"), number)(i)?;
    let (i, (left, top)) = preceded(tag(" @ "), separated_pair(number, tag(","), number))(i)?;
    let (i, (wide, tall)) = preceded(tag(": "), separated_pair(number, tag("x"), number))(i)?;

    Ok((
        i,
        Claim {
            id,
            location: Coordinate { left, top },
            size: Size { wide, tall },
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Claim>> {
    separated_list0(newline, one_entry)(i)
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("{:?}", parser(input));
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
