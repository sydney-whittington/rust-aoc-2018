pub mod template;

// Use this file to add helper functions and additional modules.

use std::str::FromStr;

use nom::{character::complete::digit1, combinator::map_res, IResult};

// https://blog.adamchalmers.com/nom-chars/
pub fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(i)
}
