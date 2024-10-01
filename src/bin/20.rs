advent_of_code::solution!(20);

use std::collections::HashMap;

use advent_of_code::CoordinateSigned;
use petgraph::{algo::k_shortest_path, graph::NodeIndex, Graph, Undirected};

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Debug)]
enum Step {
    Cardinal(Direction),
    Start,
    End,
    OptionsStart,
    OptionsEnd,
    Or,
}

fn parser(input: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    for c in input.chars() {
        steps.push(match c {
            'N' => Step::Cardinal(Direction::North),
            'E' => Step::Cardinal(Direction::East),
            'S' => Step::Cardinal(Direction::South),
            'W' => Step::Cardinal(Direction::West),
            '^' => Step::Start,
            '$' => Step::End,
            '(' => Step::OptionsStart,
            ')' => Step::OptionsEnd,
            '|' => Step::Or,
            _ => unreachable!(),
        });
    }

    steps
}

fn make_graph(steps: Vec<Step>) -> Graph<CoordinateSigned, u32, Undirected> {
    let mut graph = Graph::<CoordinateSigned, u32, Undirected>::new_undirected();
    let mut location = CoordinateSigned { x: 0, y: 0 };
    let mut coord_to_idx: HashMap<CoordinateSigned, NodeIndex> = HashMap::new();

    let mut stack_frame = Vec::new();

    for step in steps {
        match step {
            Step::Start | Step::End => (),
            Step::Cardinal(direction) => {
                let prev_idx = *coord_to_idx
                    .entry(location)
                    .or_insert_with(|| graph.add_node(location));

                match direction {
                    Direction::North => location.y += 1,
                    Direction::East => location.x += 1,
                    Direction::South => location.y -= 1,
                    Direction::West => location.x -= 1,
                };
                let current_idx = *coord_to_idx
                    .entry(location)
                    .or_insert_with(|| graph.add_node(location));

                graph.update_edge(prev_idx, current_idx, 1);
            }
            Step::OptionsStart => {
                stack_frame.push(location);
            }
            Step::OptionsEnd => {
                location = stack_frame.pop().unwrap();
            }
            Step::Or => {
                // restart the stack from the current reference frame
                location = stack_frame.pop().unwrap();
                stack_frame.push(location);
            }
        }
    }

    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let steps = parser(input);
    let graph = make_graph(steps);

    // 0-indexed value is always our start
    let shortest_paths = k_shortest_path(&graph, 0.into(), None, 1, |_| 1);
    let longest_shortest_path = *shortest_paths.values().max().unwrap();

    Some(longest_shortest_path)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one("^WNE$");
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one_d() {
        let result = part_one("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
        assert_eq!(result, Some(23));
    }
    #[test]
    fn test_part_one_e() {
        let result = part_one("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
