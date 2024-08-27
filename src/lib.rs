pub mod template;

// Use this file to add helper functions and additional modules.

use std::{fmt, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, i32, multispace0},
    combinator::map_res,
    sequence::{preceded, separated_pair},
    IResult,
};

// https://blog.adamchalmers.com/nom-chars/
pub fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(i)
}

pub fn number_usize(i: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(i)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub left: u32,
    pub top: u32,
}

pub fn coord_parse(i: &str) -> IResult<&str, Coordinate> {
    let (i, (left, top)) = separated_pair(
        preceded(multispace0, number),
        tag(", "),
        preceded(multispace0, number),
    )(i)?;
    Ok((i, Coordinate { left, top }))
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CoordinateSigned {
    pub x: i32,
    pub y: i32,
}

pub fn coord_signed_parse(i: &str) -> IResult<&str, CoordinateSigned> {
    let (i, (x, y)) = separated_pair(
        preceded(multispace0, i32),
        tag(", "),
        preceded(multispace0, i32),
    )(i)?;
    Ok((i, CoordinateSigned { x, y }))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Output(pub u32, pub u32);

// since every result type has to be formattable and our tuple isn't
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
