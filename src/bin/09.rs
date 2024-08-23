use std::collections::{HashMap, VecDeque};

use advent_of_code::number_usize;
use nom::{
    bytes::complete::tag,
    sequence::{terminated, tuple},
    IResult,
};

advent_of_code::solution!(9);

fn parser(i: &str) -> IResult<&str, (usize, usize)> {
    tuple((
        terminated(number_usize, tag(" players; last marble is worth ")),
        terminated(number_usize, tag(" points")),
    ))(i)
}

fn play_the_game(players: usize, points: usize) -> Option<usize> {
    let mut circle = VecDeque::from([0, 1]);
    let mut scores = HashMap::new();

    for i in 2..=points {
        if i % 23 == 0 {
            circle.rotate_left(7);
            let bonus = circle.pop_back().unwrap();
            scores
                .entry(i % players)
                .and_modify(|p| *p += i + bonus)
                .or_insert(i + bonus);
        } else {
            circle.rotate_right(2);
            circle.push_back(i);
        }
    }
    scores.values().max().copied()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, (players, points)) = parser(input).unwrap();

    play_the_game(players, points)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, (players, points)) = parser(input).unwrap();

    play_the_game(players, points*100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8317));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(37305));
    }

    #[test]
    fn test_part_two() {
        // no new task, just a brute force countermeasure for part 2
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
        assert!(true);
    }
}
