advent_of_code::solution!(14);

use std::{collections::VecDeque, fmt};

use itertools::{enumerate, Itertools};

struct Recipes {
    elf_1_idx: usize,
    elf_2_idx: usize,
    pub board: Vec<usize>,
    cached: VecDeque<usize>
}

impl Iterator for Recipes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cached.len() > 0 {
            return self.cached.pop_front()
        }

        let scores = self.board[self.elf_1_idx] + self.board[self.elf_2_idx];
        // https://stackoverflow.com/questions/41536479/how-do-i-split-an-integer-into-individual-digits
        let digits: Vec<usize> = scores.to_string().chars().map(|d| d.to_digit(10).unwrap().try_into().unwrap()).collect();   

        self.board.extend(digits.iter());
        self.elf_1_idx = (1 + self.board[self.elf_1_idx] + self.elf_1_idx) % self.board.len();
        self.elf_2_idx = (1 + self.board[self.elf_2_idx] + self.elf_2_idx) % self.board.len();
        
        self.cached.extend(digits);
        Some(self.cached.pop_front().unwrap())
    }
}

impl fmt::Display for Recipes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, val) in enumerate(self.board.iter()) {
            if i == self.elf_1_idx {
                write!(f, "({}) ", val).unwrap();
            }
            else if i == self.elf_2_idx {
                write!(f, "[{}] ", val).unwrap();
            }
            else {
                write!(f, " {}  ", val).unwrap();
            }
        }
        writeln!(f)
    }
}

fn new_recipes() -> Recipes {
    Recipes { elf_1_idx: 0, elf_2_idx: 1, board: vec![3, 7, 1, 0], cached: vec![3, 7, 1, 0].into()}
}

pub fn part_one(input: &str) -> Option<String> {    
    let num_recipes = input.trim().parse::<usize>().unwrap();
    let recipes = new_recipes();

    Some(recipes.skip(num_recipes).take(10).join(""))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("9");
        assert_eq!(result, Some("5158916779".to_string()));
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one("5");
        assert_eq!(result, Some("0124515891".to_string()));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one("18");
        assert_eq!(result, Some("9251071085".to_string()));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one("2018");
        assert_eq!(result, Some("5941429882".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
