use std::collections::HashMap;

use advent_of_code::{coord_parse_usize, number_usize, Coordinate};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    sequence::{preceded, terminated},
    IResult,
};
use petgraph::{algo::dijkstra, Graph, Undirected};

advent_of_code::solution!(22);

#[derive(Debug)]
struct Cave {
    depth: usize,
    target: Coordinate<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Equipment {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

impl Terrain {
    fn from_usize(value: usize) -> Terrain {
        match value % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => panic!("not a region"),
        }
    }
}

type Position = (Coordinate<usize>, Equipment);

fn parser(i: &str) -> IResult<&str, Cave> {
    let (i, depth) = terminated(preceded(tag("depth: "), number_usize), newline)(i)?;
    let (i, target) = preceded(tag("target: "), coord_parse_usize)(i)?;

    Ok((i, Cave { depth, target }))
}

fn _print_cavern(cave: &Cave, cavern: &HashMap<Coordinate<usize>, usize>) {
    for row in 0..=cave.target.top {
        for col in 0..=cave.target.left {
            let x = cavern
                .get(&Coordinate {
                    left: col,
                    top: row,
                })
                .unwrap();
            let character = if row == cave.target.top && col == cave.target.left {
                "T"
            } else if x % 3 == 0 {
                "."
            } else if x % 3 == 1 {
                "="
            } else if x % 3 == 2 {
                "|"
            } else {
                unreachable!()
            };
            print!("{}", character);
        }
        println!();
    }
    println!();
}

fn build_cavern(cave: &Cave, buffer: usize) -> HashMap<Coordinate<usize>, usize> {
    let mut cavern: HashMap<Coordinate<usize>, usize> = HashMap::new();
    // sorted by minimum sum which will give numbers closer to the top left corner first
    for (left, top) in (0..=cave.target.left + buffer)
        .cartesian_product(0..=cave.target.top + buffer)
        .sorted_by_key(|(x, y)| x + y)
    {
        let geologic_index = match (left, top) {
            (0, 0) => 0,
            (x, y) if x == cave.target.left && y == cave.target.top => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => {
                cavern
                    .get(&Coordinate {
                        left: x - 1,
                        top: y,
                    })
                    .unwrap()
                    * cavern
                        .get(&Coordinate {
                            left: x,
                            top: y - 1,
                        })
                        .unwrap()
            }
        };
        // move the modulo part up since otherwise we'll quickly overflow
        // since we're modding everything before using it, it's equivalent
        // (i tried bigints too and it was not fast enough lol)
        cavern.insert(
            Coordinate { left, top },
            (geologic_index + cave.depth) % 20183,
        );
    }

    cavern
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, cave) = parser(input).unwrap();

    let cavern = build_cavern(&cave, 0);

    Some(
        (0..=cave.target.left)
            .cartesian_product(0..=cave.target.top)
            .map(|(left, top)| cavern.get(&Coordinate { left, top }).unwrap() % 3)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, cave) = parser(input).unwrap();

    // experimentally determined, sigh
    let buffer = 30;
    let cavern = build_cavern(&cave, buffer);

    let mut coord_to_idx: HashMap<Position, _> = HashMap::new();
    let mut traversal: Graph<Position, u32, Undirected> = Graph::new_undirected();
    macro_rules! get_or_make {
        ($x:expr) => {
            *coord_to_idx
                .entry($x)
                .or_insert_with(|| traversal.add_node($x))
        };
    }

    for (left, top) in
        (0..=cave.target.left + buffer - 1).cartesian_product(0..=cave.target.top + buffer - 1)
    {
        let coords = Coordinate { left, top };
        let terrain = Terrain::from_usize(*cavern.get(&coords).unwrap());

        // self transitions
        match terrain {
            Terrain::Rocky => {
                let e1 = get_or_make!((coords, Equipment::ClimbingGear));
                let e2 = get_or_make!((coords, Equipment::Torch));
                traversal.add_edge(e1, e2, 7);
            }
            Terrain::Wet => {
                let e1 = get_or_make!((coords, Equipment::ClimbingGear));
                let e2 = get_or_make!((coords, Equipment::Neither));
                traversal.add_edge(e1, e2, 7);
            }
            Terrain::Narrow => {
                let e1 = get_or_make!((coords, Equipment::Torch));
                let e2 = get_or_make!((coords, Equipment::Neither));
                traversal.add_edge(e1, e2, 7);
            }
        };

        // is this really the best we could do? it's so big
        // also is it good form to define macros in the middle like this?
        macro_rules! add_edges {
            ($x: expr, $y: expr) => {
                let neighbor_coords = Coordinate { left: $x, top: $y };
                let neighbor_terrain = Terrain::from_usize(*cavern.get(&neighbor_coords).unwrap());
                match (terrain.clone(), neighbor_terrain) {
                    (Terrain::Rocky, Terrain::Rocky) => {
                        let e1 = get_or_make!((coords, Equipment::ClimbingGear));
                        let e2 = get_or_make!((neighbor_coords, Equipment::ClimbingGear));
                        traversal.update_edge(e1, e2, 1);
                        let e1 = get_or_make!((coords, Equipment::Torch));
                        let e2 = get_or_make!((neighbor_coords, Equipment::Torch));
                        traversal.update_edge(e1, e2, 1);
                    }
                    (Terrain::Rocky, Terrain::Wet) => {
                        let e1 = get_or_make!((coords, Equipment::ClimbingGear));
                        let e2 = get_or_make!((neighbor_coords, Equipment::ClimbingGear));
                        traversal.update_edge(e1, e2, 1);
                    }
                    (Terrain::Rocky, Terrain::Narrow) => {
                        let e1 = get_or_make!((coords, Equipment::Torch));
                        let e2 = get_or_make!((neighbor_coords, Equipment::Torch));
                        traversal.update_edge(e1, e2, 1);
                    }
                    (Terrain::Wet, Terrain::Rocky) => {
                        let e1 = get_or_make!((coords, Equipment::ClimbingGear));
                        let e2 = get_or_make!((neighbor_coords, Equipment::ClimbingGear));
                        traversal.update_edge(e1, e2, 1);
                    }
                    (Terrain::Wet, Terrain::Wet) => {
                        let e1 = get_or_make!((coords, Equipment::ClimbingGear));
                        let e2 = get_or_make!((neighbor_coords, Equipment::ClimbingGear));
                        traversal.update_edge(e1, e2, 1);
                        let e1 = get_or_make!((coords, Equipment::Neither));
                        let e2 = get_or_make!((neighbor_coords, Equipment::Neither));
                        traversal.update_edge(e1, e2, 1);
                    }
                    (Terrain::Wet, Terrain::Narrow) => {
                        let e1 = get_or_make!((coords, Equipment::Neither));
                        let e2 = get_or_make!((neighbor_coords, Equipment::Neither));
                        traversal.update_edge(e1, e2, 1);
                    }
                    (Terrain::Narrow, Terrain::Rocky) => {
                        let e1 = get_or_make!((coords, Equipment::Torch));
                        let e2 = get_or_make!((neighbor_coords, Equipment::Torch));
                        traversal.update_edge(e1, e2, 1);
                    }
                    (Terrain::Narrow, Terrain::Wet) => {
                        let e1 = get_or_make!((coords, Equipment::Neither));
                        let e2 = get_or_make!((neighbor_coords, Equipment::Neither));
                        traversal.update_edge(e1, e2, 1);
                    }
                    (Terrain::Narrow, Terrain::Narrow) => {
                        let e1 = get_or_make!((coords, Equipment::Torch));
                        let e2 = get_or_make!((neighbor_coords, Equipment::Torch));
                        traversal.update_edge(e1, e2, 1);
                        let e1 = get_or_make!((coords, Equipment::Neither));
                        let e2 = get_or_make!((neighbor_coords, Equipment::Neither));
                        traversal.update_edge(e1, e2, 1);
                    }
                }
            };
        }

        // adjacent transitions
        if left != 0 {
            add_edges!(left - 1, top);
        }
        if top != 0 {
            add_edges!(left, top - 1);
        }
        add_edges!(left + 1, top);
        add_edges!(left, top + 1);
    }

    let start = get_or_make!((Coordinate { left: 0, top: 0 }, Equipment::Torch));
    let goal = get_or_make!((cave.target, Equipment::Torch));
    let path = dijkstra(&traversal, start, Some(goal), |e| *e.weight());

    Some(*path.get(&goal).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
