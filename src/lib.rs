pub mod template;

// Use this file to add helper functions and additional modules.

use std::str::FromStr;

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res,
    sequence::separated_pair, IResult,
};

// https://blog.adamchalmers.com/nom-chars/
pub fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(i)
}

pub fn number_usize(i: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(i)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub left: u32,
    pub top: u32,
}

pub fn coord_parse(i: &str) -> IResult<&str, Coordinate> {
    let (i, (left, top)) = separated_pair(number, tag(", "), number)(i)?;
    Ok((i, Coordinate { left, top }))
}
