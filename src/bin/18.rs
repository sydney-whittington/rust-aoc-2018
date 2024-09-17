use std::collections::{HashMap, HashSet};

use advent_of_code::Coordinate;
use frozenset::{Freeze, FrozenSet};
use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

fn next_minute(map: &AreaMap) -> AreaMap {
    let mut next_map = AreaMap::new();
    for location in map.iter() {
        next_map.insert(*location.0, next_state(location, &map));
    }
    next_map
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map: AreaMap = parser(input);

    for _ in 0..10 {
        map = next_minute(&map);
    }

    let counts = map.iter().map(|c| c.1).counts();
    Some(counts.get(&Acre::Trees).unwrap()*counts.get(&Acre::Lumberyard).unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map: AreaMap = parser(input);
    let mut seen_states: HashMap<FrozenSet<(Coordinate<usize>, Acre)>, i32> = HashMap::new();

    //https://stackoverflow.com/questions/70789954/how-to-skip-forward-multiple-times-in-a-loop
    let mut timer = 0..1_000_000_000;
    while let Some(i) = timer.next() {
        map = next_minute(&map);

        let thing: FrozenSet<(Coordinate<usize>, Acre)> = HashSet::from_iter(map.iter().map(|(a, b)| (a.clone(), *b))).freeze();
        if let std::collections::hash_map::Entry::Vacant(e) = seen_states.entry(thing.clone())
        {
            e.insert(i);
        }
        else {
            // only do one big skip and manually run from there
            if i < 9000000 {
                let cycle_length = i - seen_states.get(&thing).unwrap();
                let remaining_steps = 1_000_000_000 - i;
                // integer division to get us close
                let safe_to_skip = remaining_steps / cycle_length;
                // subtract 1 to allow for the next timer.next call to line up
                timer.nth((safe_to_skip * cycle_length - 1).try_into().unwrap());
            }
        }
    }

    let counts = map.iter().map(|c| c.1).counts();
    Some(counts.get(&Acre::Trees).unwrap()*counts.get(&Acre::Lumberyard).unwrap())
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
