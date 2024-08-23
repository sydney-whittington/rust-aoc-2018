advent_of_code::solution!(8);

use advent_of_code::number_usize;
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
    metadata: Vec<usize>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self
                .children
                .iter()
                .fold(0, |acc, c| c.sum_metadata() + acc)
    }

    fn sum_values(&self) -> usize {
        if self.children.len() > 0 {
            let mut value = 0;
            for pointer in self.metadata.iter() {
                // subtract 1 for 1-indexed data
                if let Some(n) = self.children.get(*pointer - 1) {
                    value += n.sum_values();
                }
            }
            value
        } else {
            self.sum_metadata()
        }
    }
}

fn node(i: &str) -> IResult<&str, Node> {
    let (i, (child_count, metadata_count)) = (pair(
        terminated(number_usize, tag(" ")),
        terminated(number_usize, tag(" ")),
    ))(i)?;
    let (i, children) = count(node, child_count)(i)?;
    // last entry is a newline so look for space or newline
    let (i, metadata) = count(terminated(number_usize, multispace1), metadata_count)(i)?;

    Ok((i, Node { children, metadata }))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, node) = node(input).unwrap();
    Some(node.sum_metadata())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, node) = node(input).unwrap();
    Some(node.sum_values())
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
        assert_eq!(result, Some(66));
    }
}
