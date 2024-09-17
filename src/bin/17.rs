use core::fmt;
use std::{collections::HashMap, fs::File, io::Write, iter::once};

use advent_of_code::{number, Coordinate};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

advent_of_code::solution!(17);

type Vein = Vec<Coordinate<u32>>;
type ReservoirContents = HashMap<Coordinate<u32>, Ground>;

// add bounds metadata so we don't have to keep recalculating
struct Reservoir {
    contents: ReservoirContents,
    left_edge: u32,
    right_edge: u32,
    bottom_edge: u32,
    top_edge: u32,
}

impl Reservoir {
    fn below(&self, location: &Coordinate<u32>) -> Ground {
        *self
            .contents
            .get(&Coordinate {
                left: location.left,
                top: location.top + 1,
            })
            .unwrap_or(&Ground::Sand)
    }

    fn at(&self, location: &Coordinate<u32>) -> Ground {
        *self.contents.get(location).unwrap_or(&Ground::Sand)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Ground {
    Sand,
    Clay,
    Spring,
    Flooded,
    Wet,
}

#[derive(Debug, PartialEq, Eq)]
enum Filled {
    Filling,
    Overflowed,
}

impl fmt::Display for Reservoir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..=self.bottom_edge {
            for col in self.left_edge - 1..=self.right_edge {
                let character = match self.at(&Coordinate {
                    left: col,
                    top: row,
                }) {
                    Ground::Sand => ".",
                    Ground::Clay => "#",
                    Ground::Spring => "+",
                    Ground::Wet => "|",
                    Ground::Flooded => "~",
                };
                write!(f, "{}", character)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

// produce a series of points corresponding to the coordinates
fn x_vein(i: &str) -> IResult<&str, Vein> {
    let (i, x) = preceded(tag("x="), number)(i)?;
    let (i, (y1, y2)) = preceded(tag(", y="), separated_pair(number, tag(".."), number))(i)?;

    Ok((
        i,
        once(x)
            .cycle()
            .zip(y1..=y2)
            .map(|(x, y)| Coordinate { left: x, top: y })
            .collect(),
    ))
}

fn y_vein(i: &str) -> IResult<&str, Vein> {
    let (i, y) = preceded(tag("y="), number)(i)?;
    let (i, (x1, x2)) = preceded(tag(", x="), separated_pair(number, tag(".."), number))(i)?;

    Ok((
        i,
        (x1..=x2)
            .zip(once(y).cycle())
            .map(|(x, y)| Coordinate { left: x, top: y })
            .collect(),
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Vein>> {
    separated_list0(newline, alt((x_vein, y_vein)))(i)
}

// is the row we're looking at safe to fill upon?
fn check_stable(
    left_edge: &Coordinate<u32>,
    right_edge: &Coordinate<u32>,
    reservoir: &Reservoir,
) -> bool {
    let coords = ((left_edge.left + 1)..right_edge.left).map(|x| Coordinate {
        left: x,
        top: left_edge.top,
    });
    coords
        .map(|c| reservoir.below(&c))
        .all(|g| g.eq(&Ground::Clay) || g.eq(&Ground::Flooded))
}

fn fill_row(active: &Coordinate<u32>, reservoir: &mut Reservoir) -> Filled {
    let left_side = (reservoir.left_edge..active.left)
        .rev()
        .map(|y| {
            reservoir.contents.get_key_value(&Coordinate {
                top: active.top,
                left: y,
            })
        })
        .find_map(|c| match c {
            Some(c) if *c.1 == Ground::Clay => Some(c),
            _ => None,
        });

    let right_side = (active.left..=reservoir.right_edge)
        .map(|y| {
            reservoir.contents.get_key_value(&Coordinate {
                top: active.top,
                left: y,
            })
        })
        .find_map(|c| match c {
            Some(c) if *c.1 == Ground::Clay => Some(c),
            _ => None,
        });

    match left_side.zip(right_side) {
        // if the path between both edges is stable, fill
        Some((left_edge, right_edge)) => {
            if check_stable(left_edge.0, right_edge.0, reservoir) {
                for spot in (left_edge.0.left + 1)..right_edge.0.left {
                    reservoir.contents.insert(
                        Coordinate {
                            left: spot,
                            top: active.top,
                        },
                        Ground::Flooded,
                    );
                }
                Filled::Filling
            } else {
                Filled::Overflowed
            }
        }
        None => Filled::Overflowed,
    }
}

fn overflow_row(
    active: &Coordinate<u32>,
    reservoir: &mut Reservoir,
) -> (Option<Coordinate<u32>>, Option<Coordinate<u32>>) {
    // look left and right for where/if it should fall off
    let left_side = (reservoir.left_edge..=active.left - 1)
        .rev()
        .map(|y| Coordinate {
            left: y,
            top: active.top,
        })
        .take_while(|c| {
            reservoir.at(c).eq(&Ground::Sand)
                || (reservoir.at(c).eq(&Ground::Wet) && reservoir.below(c).eq(&Ground::Flooded))
        })
        .take_while_inclusive(|c| matches!(reservoir.below(c), Ground::Flooded | Ground::Clay))
        .collect_vec();
    let right_side = (active.left + 1..=reservoir.right_edge)
        .map(|y| Coordinate {
            left: y,
            top: active.top,
        })
        .take_while(|c| {
            reservoir.at(c).eq(&Ground::Sand)
                || (reservoir.at(c).eq(&Ground::Wet) && reservoir.below(c).eq(&Ground::Flooded))
        })
        .take_while_inclusive(|c| matches!(reservoir.below(c), Ground::Flooded | Ground::Clay))
        .collect_vec();

    let mut anything_changed = false;
    for coord in once(active).chain(left_side.iter().chain(right_side.iter())) {
        if reservoir.contents.insert(*coord, Ground::Wet).is_none() {
            anything_changed = true;
        }
    }

    if anything_changed {
        // the last of each is the outermost one, which may have new flows to descend
        (left_side.last().cloned(), right_side.last().cloned())
    } else {
        // otherwise we're looking at something we've already done and don't need to go further
        (None, None)
    }
}

fn fill(mut active: Coordinate<u32>, reservoir: &mut Reservoir) {
    // draw a line down until you can't
    active.top += 1;
    while matches!(reservoir.at(&active), Ground::Sand | Ground::Wet)
        && active.top <= reservoir.bottom_edge
    {
        reservoir.contents.insert(active, Ground::Wet);
        if active.top == reservoir.bottom_edge {
            return;
        } else {
            active.top += 1;
        }
    }
    // rewind back to the bottom of the container
    active.top -= 1;

    // fill left and right and repeat, moving up a row at a time
    loop {
        let state = fill_row(&active, reservoir);
        if matches!(state, Filled::Filling) {
            active.top -= 1;
        } else {
            // until there aren't walls on both sides
            break;
        }
    }

    // fill the top of the container with wet
    let (left_side, right_side) = overflow_row(&active, reservoir);

    // and then recurse where it spills down
    if let Some(left) = left_side {
        fill(left, reservoir);
    }
    if let Some(right) = right_side {
        fill(right, reservoir);
    }
}

fn make_reservoir(veins: Vec<Vein>) -> Reservoir {
    let mut contents: ReservoirContents =
        HashMap::from_iter(veins.into_iter().flatten().zip(once(Ground::Clay).cycle()));

    // calculate scoring bounds before we add our start point
    let (min_x, max_x) = contents
        .keys()
        .map(|c| c.left)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = contents
        .keys()
        .map(|c| c.top)
        .minmax()
        .into_option()
        .unwrap();

    contents.insert(Coordinate { left: 500, top: 0 }, Ground::Spring);
    contents.insert(Coordinate { left: 500, top: 1 }, Ground::Wet);

    Reservoir {
        contents,
        bottom_edge: max_y,
        top_edge: min_y,
        // side edges need to be expanded one column to allow for overflow
        left_edge: min_x - 1,
        right_edge: max_x + 1,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, veins) = parser(input).unwrap();
    let mut reservoir = make_reservoir(veins);

    let start = Coordinate { left: 500, top: 1 };
    fill(start, &mut reservoir);

    let mut w = File::create("test.txt").unwrap();
    write!(&mut w, "{}", reservoir).unwrap();

    Some(
        reservoir
            .contents
            .into_iter()
            // within the grid, since we're modeling water above their bounds
            .filter(|(c, _)| c.top >= reservoir.top_edge)
            .map(|(_, g)| g)
            .filter(|&g| g.eq(&Ground::Flooded) || g.eq(&Ground::Wet))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, veins) = parser(input).unwrap();
    let mut reservoir = make_reservoir(veins);

    let start = Coordinate { left: 500, top: 1 };
    fill(start, &mut reservoir);

    let mut w = File::create("test.txt").unwrap();
    write!(&mut w, "{}", reservoir).unwrap();

    Some(
        reservoir
            .contents
            .into_iter()
            .filter(|(c, _)| c.top >= reservoir.top_edge)
            .map(|(_, g)| g)
            // all that changes is we're only looking for flooded tiles
            .filter(|&g| g.eq(&Ground::Flooded))
            .count(),
    )
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
        assert_eq!(result, Some(29));
    }
}
