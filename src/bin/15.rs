advent_of_code::solution!(15);

use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt,
};

use advent_of_code::Coordinate;
use itertools::Itertools;

#[derive(Debug)]
enum CaveFeature {
    Wall,
    Open,
}

#[derive(Debug, Clone, Copy)]
enum Creature {
    Goblin(i32),
    Elf(i32),
}

type CaveMap = HashMap<Coordinate<usize>, CaveFeature>;

type Creatures = HashMap<Coordinate<usize>, Creature>;

#[derive(Debug, Clone, Copy)]
struct NoTargets(Creature);

impl fmt::Display for NoTargets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no enemies left for {:?}", self.0)
    }
}

impl Error for NoTargets {}

// not really a token based parser so just doing it with string manipulation instead of nom
fn parser(i: &str) -> (CaveMap, Creatures) {
    let mut map = CaveMap::new();
    let mut creatures = Creatures::new();
    for (top, line) in i.lines().enumerate() {
        for (left, character) in line.chars().enumerate() {
            match character {
                '#' => {
                    map.insert(Coordinate { left, top }, CaveFeature::Wall);
                }
                '.' => {
                    map.insert(Coordinate { left, top }, CaveFeature::Open);
                }
                'G' => {
                    map.insert(Coordinate { left, top }, CaveFeature::Open);
                    creatures.insert(Coordinate { left, top }, Creature::Goblin(200));
                }
                'E' => {
                    map.insert(Coordinate { left, top }, CaveFeature::Open);
                    creatures.insert(Coordinate { left, top }, Creature::Elf(200));
                }
                _ => {
                    // includes track characters and empty space
                }
            }
        }
    }

    (map, creatures)
}

fn find_targets(
    creature: &Creature,
    creatures: &VecDeque<(Coordinate<usize>, Creature)>,
) -> HashSet<Creature> {
    let targets = HashSet::new();

    targets
}

fn advance_round(map: &CaveMap, creatures: Creatures) -> Result<Creatures, NoTargets> {
    let mut creatures = VecDeque::from_iter(
        creatures
            .into_iter()
            .sorted_unstable_by_key(|&a| (a.0.top, a.0.left)),
    );

    for (coord, creature) in creatures.iter() {
        let targets = find_targets(&creature, &creatures);

        if targets.is_empty() {
            return Err(NoTargets(*creature));
        }
    }

    Ok(HashMap::from_iter(creatures.into_iter()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, creatures) = parser(input);
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
