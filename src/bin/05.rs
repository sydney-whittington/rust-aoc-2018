advent_of_code::solution!(5);

use itertools::Itertools;

// here lies a noble but aborted attempt to parse the string and reduce at the same time
// not realizing that you can't splice together the input string with &strs
// and also that there's probably an easier way to do it

fn can_react(a: &char, b: &char) -> bool {
    a.eq_ignore_ascii_case(&b) && a != b
}

pub fn part_one(input: &str) -> Option<u32> {
    // somewhat based on https://www.reddit.com/r/adventofcode/comments/a3912m/2018_day_5_solutions/eb4fkwu/
    let polymer = input.trim().chars().fold(Vec::new(), |mut s, c| {
        match s.last() {
            None => s.push(c),
            Some(&p) => {
                if can_react(&c, &p) {
                    s.pop();
                } else {
                    s.push(c);
                }
            }
        };
        s
    });
    Some(polymer.len().try_into().unwrap())
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
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
