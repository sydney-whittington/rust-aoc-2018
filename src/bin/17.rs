use std::{collections::HashMap, iter::once};

use advent_of_code::{Coordinate, number};
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, character::complete::newline, multi::separated_list0, sequence::{preceded, separated_pair}, IResult};

advent_of_code::solution!(17);

type Vein = Vec<Coordinate<u32>>;
type Reservoir = HashMap<Coordinate<u32>, Dirt>;

#[derive(Debug, Hash, Clone, Copy)]
enum Dirt {
    Sand,
    Clay,
    Spring
}

fn x_vein(i: &str) -> IResult<&str, Vein> {
    let (i, x) = preceded(tag("x="), number)(i)?;
    let (i, (y1, y2)) = preceded(tag(", y="), separated_pair(number, tag(".."), number))(i)?;

    Ok((i, once(x).cycle().zip(y1..=y2).map(|(x, y)| Coordinate {left: x, top: y}).collect()))
}

fn y_vein(i: &str) -> IResult<&str, Vein> {
    let (i, y) = preceded(tag("y="), number)(i)?;
    let (i, (x1, x2)) = preceded(tag(", x="), separated_pair(number, tag(".."), number))(i)?;

    Ok((i, (x1..=x2).zip(once(y).cycle()).map(|(x, y)| Coordinate {left: x, top: y}).collect()))
}

fn parser(i: &str) -> IResult<&str, Vec<Vein>> {
    separated_list0(newline, alt((x_vein, y_vein)))(i)
}

fn print_reservoir(reservoir: &Reservoir) -> () {
    let (min_y, max_y) = reservoir.keys().map(|c| c.left).minmax().into_option().unwrap();
    let max_x = reservoir.keys().map(|c| c.top).max().unwrap();

    for row in 0..=max_x {
        for col in min_y..=max_y {
            let character = match reservoir.get(&Coordinate { left: col, top: row }).or(Some(&Dirt::Sand)).unwrap() {
                Dirt::Sand => ".",
                Dirt::Clay => "#",
                Dirt::Spring => "+",
            }; 
            print!("{}", character);
        }
        println!();
    }
    println!();
    
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, veins) = parser(input).unwrap();
    let mut reservoir: Reservoir  = HashMap::from_iter(veins.into_iter().flatten().zip(once(Dirt::Clay).cycle()));
    reservoir.insert(Coordinate {left: 500, top: 0}, Dirt::Spring);

    print_reservoir(&reservoir);

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
        assert_eq!(result, Some(57));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
