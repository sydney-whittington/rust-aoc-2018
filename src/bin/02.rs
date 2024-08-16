use itertools::Itertools;
use levenshtein::levenshtein;
use nom::{
    character::complete::{alpha1, newline},
    multi::separated_list0,
    IResult, InputIter,
};

advent_of_code::solution!(2);

fn parser(i: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(newline, alpha1)(i)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, boxes) = parser(input).unwrap();
    let two: u32 = boxes
        .iter()
        .map(|v| v.chars().counts().values().any(|&x| x == 2))
        .filter(|x| *x == true)
        .count()
        .try_into()
        .unwrap();
    let three: u32 = boxes
        .iter()
        .map(|v| v.chars().counts().values().any(|&x| x == 3))
        .filter(|x| *x == true)
        .count()
        .try_into()
        .unwrap();
    Some((two * three).into())
}

pub fn part_two<'a>(input: &'a str) -> Option<String> {
    let (_, boxes) = parser(input).unwrap();
    let closest_two = boxes
        .iter()
        .combinations(2)
        .find(|x| levenshtein(x[0], x[1]) == 1)
        .unwrap();
    let shared: String = closest_two[0]
        .chars()
        .zip(closest_two[1].chars())
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .collect();
    Some(shared)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("fgij".to_string()));
    }
}
