advent_of_code::solution!(8);

use advent_of_code::{number, number_usize};
use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    multi::count,
    sequence::{pair, terminated},
    IResult,
};

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn sum_metadata(&self) -> u32 {
        self.metadata.iter().sum::<u32>()
            + self
                .children
                .iter()
                .fold(0, |acc, c| c.sum_metadata() + acc)
    }
}

fn node(i: &str) -> IResult<&str, Node> {
    let (i, (child_count, metadata_count)) = (pair(
        terminated(number_usize, tag(" ")),
        terminated(number_usize, tag(" ")),
    ))(i)?;
    let (i, children) = count(node, child_count)(i)?;
    // last entry is a newline so look for space or newline
    let (i, metadata) = count(terminated(number, multispace1), metadata_count)(i)?;

    Ok((i, Node { children, metadata }))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, node) = node(input).unwrap();
    Some(node.sum_metadata())
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
        assert_eq!(result, Some(138));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
