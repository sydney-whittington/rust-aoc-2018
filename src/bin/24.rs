use std::{
    collections::{HashMap, HashSet},
    hash::RandomState,
    str::FromStr,
};

use advent_of_code::number;
use frozenset::FrozenSet;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, newline, u32},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

advent_of_code::solution!(24);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Attack {
    damage: u32,
    damage_type: DamageType,
    initiative: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Defense {
    Weakness(DamageType),
    Immunity(DamageType),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Side {
    Immune,
    Infection,
    Unassigned,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Group {
    units: u32,
    hit_points: u32,
    defenses: FrozenSet<Defense>,
    attack: Attack,
    side: Side,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units * self.attack.damage
    }

    fn potential_damage(&self, other: &Group) -> u32 {
        if other
            .defenses
            .contains(&Defense::Weakness(self.attack.damage_type))
        {
            self.effective_power() * 2
        } else if other
            .defenses
            .contains(&Defense::Immunity(self.attack.damage_type))
        {
            0
        } else {
            self.effective_power()
        }
    }
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

fn parse_defenses(i: &str) -> IResult<&str, FrozenSet<Defense>> {
    if i.starts_with("(") {
        let (i, defenses) = delimited(
            tag("("),
            many1(separated_list1(
                tag("; "),
                alt((parse_weaknesses, parse_immunities)),
            )),
            tag(") "),
        )(i)?;
        let combined = defenses.iter().flatten().flatten().map(|c| c.to_owned());

        Ok((i, FrozenSet::from_iter(combined)))
    } else {
        Ok((i, FrozenSet::new()))
    }
}

fn parse_attack(i: &str) -> IResult<&str, Attack> {
    let (i, damage) = preceded(tag("with an attack that does "), number)(i)?;
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
    let (i, units) = terminated(u32, tag(" units each with "))(i)?;
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
            side: Side::Unassigned,
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Group>> {
    let (i, mut immune) = preceded(
        tuple((tag("Immune System:"), multispace1)),
        separated_list1(newline, parse_group),
    )(i)?;
    immune.iter_mut().for_each(|g| g.side = Side::Immune);
    let (i, mut infection) = preceded(
        tuple((multispace1, tag("Infection:"), multispace1)),
        separated_list1(newline, parse_group),
    )(i)?;
    infection.iter_mut().for_each(|g| g.side = Side::Infection);

    Ok((i, immune.into_iter().chain(infection).collect()))
}

fn fight(groups: &HashMap<usize, Group>, debug: bool) -> HashMap<usize, Group> {
    // target selection
    let mut immune_targets: HashSet<(&usize, &Group), RandomState> =
        HashSet::from_iter(groups.iter().filter(|(_, g)| g.side == Side::Immune));
    let mut infection_targets: HashSet<(&usize, &Group), RandomState> =
        HashSet::from_iter(groups.iter().filter(|(_, g)| g.side == Side::Infection));

    if debug {
        println!("Immune System:");
        for (i, group) in immune_targets.iter(){
            println!("Group {} contains {} units", i, group.units)
        }
        println!("Infection:");
        for (i, group) in infection_targets.iter(){
            println!("Group {} contains {} units", i, group.units)
        }
        println!();
    }

    let mut attacks = HashMap::new();

    for (i, group) in groups.iter().sorted_unstable_by_key(|(_, g)| (g.effective_power(), g.attack.initiative)).rev() {
        if matches!(group.side, Side::Immune) {
            if let Some(best_target) = infection_targets
                .iter()
                .max_by_key(|(_, g)| (group.potential_damage(g), g.effective_power(), g.attack.initiative))
                .cloned()
            {
                if debug {
                    println!("Immune System group {} would deal defending group {} {} damage", i, best_target.0, group.potential_damage(best_target.1));
                }
                attacks.insert((i, group), infection_targets.take(&best_target));
            }
        } else if matches!(group.side, Side::Infection) {
            if let Some(best_target) = immune_targets
                .iter()
                .max_by_key(|(_, g) | (group.potential_damage(g), g.effective_power(), g.attack.initiative))
                .cloned()
            {
                if debug {
                    println!("Immune System group {} would deal defending group {} {} damage", i, best_target.0, group.potential_damage(best_target.1));
                }
                attacks.insert((i, group), immune_targets.take(&best_target));
            }
        }
    }
    let mut survivors = groups.to_owned();

    for (attacker, defender) in attacks.iter().sorted_unstable_by_key(|((_, a), _)| a.attack.initiative).rev() {
        if defender.is_none() || !survivors.contains_key(attacker.0) {
            continue;
        }

        let (defender_id, defender) = defender.unwrap();
        let attacker_id = attacker.0;
        let attacker = survivors.get(attacker_id).unwrap();
        let damage = attacker.potential_damage(defender);
        let units_lost = damage / defender.hit_points;

        if debug {
            println!("{:?} group {} attacks defending group {}, killing {} units", attacker.side, attacker_id, defender_id, units_lost);
        }

        if units_lost < defender.units {
            let mut survivor = survivors.remove(defender_id).unwrap();
            survivor.units -= units_lost;
            survivors.insert(*defender_id, survivor);
        } else {
            survivors.remove(defender_id);
        }
    }

    survivors
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, battle) = parser(input).unwrap();
    let mut battle = HashMap::from_iter(battle.into_iter().enumerate());

    loop {
        let next_battle = fight(&battle, false);
        if next_battle != battle {
            battle = next_battle;
        }
        else {
            break;
        }
    }
    Some(battle.values().map(|g| g.units).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, battle) = parser(input).unwrap();
    let mut battle = HashMap::from_iter(battle.into_iter().enumerate());

    for i in 1.. {
        // increment the immune power by 1
        battle.iter_mut().filter(|(_, g)| matches!(g.side, Side::Immune)).for_each(|(_, g)| g.attack.damage += 1);
        let mut boosted_battle = battle.clone();
        loop {
            let next_battle = fight(&boosted_battle, false);
            if next_battle != boosted_battle {
                boosted_battle = next_battle;
            }
            else {
                break;
            }
        }
        let immune_won = boosted_battle.values().all(|g| matches!(g.side, Side::Immune));
        if immune_won {
            return Some(boosted_battle.values().map(|g| g.units).sum());
        }
        else {
            println!("tried boost of {}", i);
        }
    }
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
        assert_eq!(result, Some(51));
    }
}
