advent_of_code::solution!(5);

use std::cmp::min;

// here lies a noble but aborted attempt to parse the string and reduce at the same time
// not realizing that you can't splice together the input string with &strs
// and also that there's probably an easier way to do it

fn can_react(a: &u8, b: &u8) -> bool {
    a.eq_ignore_ascii_case(&b) && a != b
}

fn polymerize<'a>(i: impl Iterator<Item = &'a u8>) -> Vec<u8> {
    // somewhat based on https://www.reddit.com/r/adventofcode/comments/a3912m/2018_day_5_solutions/eb4fkwu/
    i.fold(Vec::new(), |mut s, c| {
        match s.last() {
            None => s.push(*c),
            Some(&p) => {
                if can_react(&c, &p) {
                    s.pop();
                } else {
                    s.push(*c);
                }
            }
        };
        s
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let polymer = polymerize(input.trim().as_bytes().iter());
    Some(polymer.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let polymer = polymerize(input.trim().as_bytes().iter());
    let mut smallest = usize::MAX;
    for letter in b'a'..b'z' {
        let filtered = polymer
            .iter()
            .filter(|c| !c.eq_ignore_ascii_case(&letter));
        smallest = min(smallest, polymerize(filtered).len())
    }
    Some(smallest)
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
        assert_eq!(result, Some(4));
    }
}
