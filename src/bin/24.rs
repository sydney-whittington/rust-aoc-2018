use std::{collections::HashSet, str::FromStr};

use advent_of_code::number;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i32, multispace1, newline},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

advent_of_code::solution!(24);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum DamageType {
    Slashing,
    Bludgeoning,
    Fire,
    Cold,
    Radiation,
}

impl FromStr for DamageType {
    type Err = ();

    fn from_str(input: &str) -> Result<DamageType, Self::Err> {
        match input {
            "slashing" => Ok(DamageType::Slashing),
            "bludgeoning" => Ok(DamageType::Bludgeoning),
            "fire" => Ok(DamageType::Fire),
            "cold" => Ok(DamageType::Cold),
            "radiation" => Ok(DamageType::Radiation),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Attack {
    damage: u32,
    damage_type: DamageType,
    initiative: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Defense {
    Weakness(DamageType),
    Immunity(DamageType),
}

#[derive(Debug)]
struct Group {
    units: i32,
    hit_points: u32,
    defenses: HashSet<Defense>,
    attack: Attack,
}

#[derive(Debug)]
struct Battle {
    immune: Vec<Group>,
    infection: Vec<Group>,
}

fn parse_weaknesses(i: &str) -> IResult<&str, Vec<Defense>> {
    let (i, weaknesses) = preceded(tag("weak to "), separated_list1(tag(", "), alpha1))(i)?;

    Ok((
        i,
        weaknesses
            .into_iter()
            .map(|w| Defense::Weakness(DamageType::from_str(w).unwrap()))
            .collect(),
    ))
}

fn parse_immunities(i: &str) -> IResult<&str, Vec<Defense>> {
    let (i, immunities) = preceded(tag("immune to "), separated_list1(tag(", "), alpha1))(i)?;

    Ok((
        i,
        immunities
            .into_iter()
            .map(|w| Defense::Immunity(DamageType::from_str(w).unwrap()))
            .collect(),
    ))
}

fn parse_defenses(i: &str) -> IResult<&str, HashSet<Defense>> {
    if i.starts_with("(") {
        let (i, defenses) = delimited(
            tag("("),
            many1(separated_list1(
                tag("; "),
                alt((parse_weaknesses, parse_immunities)),
            )),
            tag(")"),
        )(i)?;
        let combined = defenses.iter().flatten().flatten().map(|c| c.to_owned());

        Ok((i, HashSet::from_iter(combined)))
    } else {
        Ok((i, HashSet::new()))
    }
}

fn parse_attack(i: &str) -> IResult<&str, Attack> {
    let (i, damage) = preceded(tag(" with an attack that does "), number)(i)?;
    let (i, damage_type) = preceded(tag(" "), alpha1)(i)?;
    let damage_type = DamageType::from_str(damage_type).unwrap();
    let (i, initiative) = preceded(tag(" damage at initiative "), number)(i)?;

    Ok((
        i,
        Attack {
            damage,
            damage_type,
            initiative,
        },
    ))
}

fn parse_group(i: &str) -> IResult<&str, Group> {
    let (i, units) = terminated(i32, tag(" units each with "))(i)?;
    let (i, hit_points) = terminated(number, tag(" hit points "))(i)?;
    let (i, defenses) = parse_defenses(i)?;
    let (i, attack) = parse_attack(i)?;

    Ok((
        i,
        Group {
            units,
            hit_points,
            defenses,
            attack,
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Battle> {
    let (i, immune) = preceded(
        tuple((tag("Immune System:"), multispace1)),
        separated_list1(newline, parse_group),
    )(i)?;
    let (i, infection) = preceded(
        tuple((multispace1, tag("Infection:"), multispace1)),
        separated_list1(newline, parse_group),
    )(i)?;

    Ok((i, Battle { immune, infection }))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, battle) = parser(input).unwrap();
    dbg!(battle);
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
        assert_eq!(result, Some(5216));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
