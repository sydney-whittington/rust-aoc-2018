use std::collections::HashMap;

use advent_of_code::Coordinate;
use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Debug, PartialEq, Eq, Hash)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

type Location<'a> = (&'a Coordinate<usize>, &'a Acre);

type AreaMap = HashMap<Coordinate<usize>, Acre>;

fn parser(i: &str) -> AreaMap {
    let mut map = AreaMap::new();
    // 1-indexing to make adjacency checks easier
    for (line, top) in i.lines().zip(1..) {
        for (character, left) in line.chars().zip(1..) {
            match character {
                '.' => {
                    map.insert(Coordinate { left, top }, Acre::Open);
                }
                '|' => {
                    map.insert(Coordinate { left, top }, Acre::Trees);
                }
                '#' => {
                    map.insert(Coordinate { left, top }, Acre::Lumberyard);
                }
                _ => {
                    panic!("unexpected character")
                }
            }
        }
    }

    map
}

fn next_state(location: Location, map: &AreaMap) -> Acre {
    let nearby = location
        .0
        .adjacents()
        .iter()
        .filter_map(|c| map.get(c))
        .counts();

    match location.1 {
        Acre::Open => {
            if nearby.get(&Acre::Trees).is_some_and(|c| *c >= 3) {
                Acre::Trees
            } else {
                Acre::Open
            }
        }
        Acre::Trees => {
            if nearby.get(&Acre::Lumberyard).is_some_and(|c| *c >= 3) {
                Acre::Lumberyard
            } else {
                Acre::Trees
            }
        }
        Acre::Lumberyard => {
            if nearby.get(&Acre::Lumberyard).is_some_and(|c| *c >= 1)
                && nearby.get(&Acre::Trees).is_some_and(|c| *c >= 1)
            {
                Acre::Lumberyard
            } else {
                Acre::Open
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map: AreaMap = parser(input);

    let mut next_map = AreaMap::new();
    for _ in 0..10 {
        for location in map.iter() {
            next_map.insert(*location.0, next_state(location, &map));
        }

        map = next_map;
        next_map = AreaMap::new();
    }

    let counts = map.iter().map(|c| c.1).counts();
    Some(counts.get(&Acre::Trees).unwrap()*counts.get(&Acre::Lumberyard).unwrap())
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
        assert_eq!(result, Some(1147));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
