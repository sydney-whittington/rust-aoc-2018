use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
};

use advent_of_code::{Coordinate, Output};
use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug)]
enum Feature {
    FsCorner,
    BsCorner,
    Intersection,
    // no need for straight tracks since carts follow them anyway
}

#[derive(Debug, Clone, Copy)]
enum Cycle {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Cart {
    Up(Cycle),
    Down(Cycle),
    Left(Cycle),
    Right(Cycle),
}

type CartMap = HashMap<Coordinate<usize>, Feature>;

type AllCarts = HashMap<Coordinate<usize>, Cart>;

#[derive(Debug, Clone, Copy)]
struct CartCrash(Coordinate<usize>);

impl Error for CartCrash {}

impl fmt::Display for CartCrash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "crash detected at {:?}", self.0)
    }
}

// not really a token based parser so just doing it with string manipulation instead of nom
fn parser(i: &str) -> (CartMap, AllCarts) {
    let mut map = CartMap::new();
    let mut state = AllCarts::new();
    for (top, line) in i.lines().enumerate() {
        for (left, character) in line.chars().enumerate() {
            match character {
                '/' => {
                    map.insert(Coordinate { left, top }, Feature::FsCorner);
                }
                '\\' => {
                    map.insert(Coordinate { left, top }, Feature::BsCorner);
                }
                '+' => {
                    map.insert(Coordinate { left, top }, Feature::Intersection);
                }
                '^' => {
                    state.insert(Coordinate { left, top }, Cart::Up(Cycle::Left));
                }
                '<' => {
                    state.insert(Coordinate { left, top }, Cart::Left(Cycle::Left));
                }
                '>' => {
                    state.insert(Coordinate { left, top }, Cart::Right(Cycle::Left));
                }
                'v' => {
                    state.insert(Coordinate { left, top }, Cart::Down(Cycle::Left));
                }
                _ => {
                    // includes track characters and empty space
                }
            }
        }
    }

    (map, state)
}

fn new_location(coord: &Coordinate<usize>, facing: &Cart) -> Coordinate<usize> {
    match facing {
        Cart::Up(_) => Coordinate {
            left: coord.left,
            top: coord.top - 1,
        },
        Cart::Down(_) => Coordinate {
            left: coord.left,
            top: coord.top + 1,
        },
        Cart::Left(_) => Coordinate {
            left: coord.left - 1,
            top: coord.top,
        },
        Cart::Right(_) => Coordinate {
            left: coord.left + 1,
            top: coord.top,
        },
    }
}

fn new_facing(coord: &Coordinate<usize>, cart: &Cart, map: &CartMap) -> Cart {
    match map.get(coord) {
        Some(Feature::FsCorner) => match cart {
            Cart::Up(c) => Cart::Right(*c),
            Cart::Down(c) => Cart::Left(*c),
            Cart::Left(c) => Cart::Down(*c),
            Cart::Right(c) => Cart::Up(*c),
        },
        Some(Feature::BsCorner) => match cart {
            Cart::Up(c) => Cart::Left(*c),
            Cart::Down(c) => Cart::Right(*c),
            Cart::Left(c) => Cart::Up(*c),
            Cart::Right(c) => Cart::Down(*c),
        },
        Some(Feature::Intersection) => match cart {
            Cart::Up(c) => match c {
                Cycle::Left => Cart::Left(Cycle::Straight),
                Cycle::Straight => Cart::Up(Cycle::Right),
                Cycle::Right => Cart::Right(Cycle::Left),
            },
            Cart::Down(c) => match c {
                Cycle::Left => Cart::Right(Cycle::Straight),
                Cycle::Straight => Cart::Down(Cycle::Right),
                Cycle::Right => Cart::Left(Cycle::Left),
            },
            Cart::Left(c) => match c {
                Cycle::Left => Cart::Down(Cycle::Straight),
                Cycle::Straight => Cart::Left(Cycle::Right),
                Cycle::Right => Cart::Up(Cycle::Left),
            },
            Cart::Right(c) => match c {
                Cycle::Left => Cart::Up(Cycle::Straight),
                Cycle::Straight => Cart::Right(Cycle::Right),
                Cycle::Right => Cart::Down(Cycle::Left),
            },
        },
        // if we don't have anything in the map, it keeps its previous facing
        _ => *cart,
    }
}

fn next_state(state: &AllCarts, map: &CartMap) -> Result<AllCarts, CartCrash> {
    let mut next_state = AllCarts::new();

    // sort by multiple fields lexicographically (https://stackoverflow.com/questions/70193935/how-to-sort-a-vec-of-structs-by-2-or-multiple-fields)
    for (coord, cart) in state
        .iter()
        .sorted_unstable_by_key(|&a| (a.0.top, a.0.left))
    {
        let new_coord = new_location(coord, &cart);
        let new_cart = new_facing(&new_coord, cart, map);

        // hit where it was previously
        if state.contains_key(&new_coord) {
            return Err(CartCrash(new_coord));
        }

        // hit something that's there this tick
        if let Some(_) = next_state.insert(new_coord, new_cart) {
            return Err(CartCrash(new_coord));
        }
    }

    Ok(next_state)
}

fn next_state_removal(state: &AllCarts, map: &CartMap) -> AllCarts {
    let mut next_state = AllCarts::new();
    let mut known_collisions: HashSet<Coordinate<usize>> = HashSet::new();

    // sort by multiple fields lexicographically (https://stackoverflow.com/questions/70193935/how-to-sort-a-vec-of-structs-by-2-or-multiple-fields)
    for (coord, cart) in state
        .iter()
        .sorted_unstable_by_key(|&a| (a.0.top, a.0.left))
    {
        if known_collisions.contains(coord) {
            // if we already collided this time step, don't add it to next round
            // but if they collide then the intersection is clear so the third cart is fine
            known_collisions.remove(coord);
            continue;
        }

        let new_coord = new_location(coord, &cart);
        let new_cart = new_facing(&new_coord, cart, map);

        if state.contains_key(&new_coord) {
            // it hits it only if, in sorting order, it has moved first
            // otherwise it's already out of the way
            if (coord.top, coord.left) < (new_coord.top, new_coord.left) {
                // we've collided before it moved, skip adding this one
                known_collisions.insert(new_coord);
                continue;
            }
        }

        if let Some(_) = next_state.insert(new_coord, new_cart) {
            next_state.remove(&new_coord);
        }
    }

    next_state
}

pub fn part_one(input: &str) -> Option<Output<usize>> {
    let (map, mut state) = parser(input);

    loop {
        match next_state(&state, &map) {
            Ok(s) => state = s,
            Err(CartCrash(c)) => return Some(Output(c.left, c.top)),
        }
    }
}

pub fn part_two(input: &str) -> Option<Output<usize>> {
    let (map, mut state) = parser(input);

    loop {
        state = next_state_removal(&state, &map);
        if state.len() == 1 {
            let (only_element, _) = state.drain().take(1).next().unwrap();
            return Some(Output(only_element.left, only_element.top));
        }
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Output;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(Output(7, 3)));
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(Output(0, 3)));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(Output(6, 4)));
    }
}
