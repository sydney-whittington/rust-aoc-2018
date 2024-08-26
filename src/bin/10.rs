advent_of_code::solution!(10);

use std::{collections::HashMap, i32};
use std::iter::repeat;

use advent_of_code::{coord_signed_parse, CoordinateSigned};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list0,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, Copy, Clone)]
struct Light {
    position: CoordinateSigned,
    velocity: CoordinateSigned,
}

#[derive(Debug)]
struct Corners {
    top_left: CoordinateSigned,
    bottom_right: CoordinateSigned,
}

impl Corners {
    fn xs(&self) -> impl Iterator<Item = i32> {
        self.top_left.x..=self.bottom_right.x
    }

    fn ys(&self) -> impl Iterator<Item = i32> {
        self.top_left.y..=self.bottom_right.y
    }

    fn size(&self) -> i64 {
        (self.bottom_right.x - self.top_left.x) as i64 * (self.bottom_right.y - self.top_left.y) as i64
    }
}

#[derive(Debug)]
struct Sky {
    space: HashMap<(i32, i32), char>,
    corners: Corners,
}

fn one_entry(i: &str) -> IResult<&str, Light> {
    let (i, position) = preceded(tag("position=<"), coord_signed_parse)(i)?;
    let (i, velocity) = delimited(tag("> velocity=<"), coord_signed_parse, tag(">"))(i)?;

    Ok((i, Light { position, velocity }))
}

fn parser(i: &str) -> IResult<&str, Vec<Light>> {
    separated_list0(newline, one_entry)(i)
}

fn get_corners(lights: &Vec<Light>) -> Corners {
    let (min_x, max_x) = lights
        .iter()
        .map(|l| l.position.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = lights
        .iter()
        .map(|l| l.position.y)
        .minmax()
        .into_option()
        .unwrap();

    Corners {
        top_left: CoordinateSigned { x: min_x, y: min_y },
        bottom_right: CoordinateSigned { x: max_x, y: max_y },
    }
}

fn skygaze(sky: &Sky) -> () {
    for column in sky.corners.ys() {
        for row in sky.corners.xs() {
            print!("{}", sky.space.get(&(row, column)).unwrap());
        }
        println!();
    }
    println!();
}

fn advance(lights: &mut Vec<Light>) -> () {
    // no vector iterators for us here (https://stackoverflow.com/questions/49143770/efficiently-mutate-a-vector-while-also-iterating-over-the-same-vector)
    for i in 0..lights.len() {
        lights[i].position.x += lights[i].velocity.x;
        lights[i].position.y += lights[i].velocity.y;
    }
}

// for when we overshoot so we don't have to store the whole array at each step just in case
fn retreat(lights: &mut Vec<Light>) -> () {
    for i in 0..lights.len() {
        lights[i].position.x -= lights[i].velocity.x;
        lights[i].position.y -= lights[i].velocity.y;
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (_, mut lights) = parser(input).unwrap();

    // find where the bounding box is the smallest
    let mut previous_size = get_corners(&lights).size();
    loop {
        advance(&mut lights);
        let current_size = get_corners(&lights).size();
        if current_size < previous_size {
            previous_size = current_size;
        }
        else {
            retreat(&mut lights);
            break;
        }
    }

    // and finally, display the sky
    let corners = get_corners(&lights);
    let mut sky = Sky {
        space: HashMap::from_iter(
            (corners.xs())
                // the iterator can't be cloned but the collected vector can
                .cartesian_product(corners.ys().collect::<Vec<_>>())
                .zip(repeat('.')),
        ),
        corners,
    };

    // put stars in the sky
    for light in lights.iter() {
        sky.space
            .entry((light.position.x, light.position.y))
            .and_modify(|e| *e = '#');
    }

    skygaze(&sky);

    Some("hi".to_string())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // there's not really a way to programmatically test this without a human involved or doing extra recognition work
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("hi".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
