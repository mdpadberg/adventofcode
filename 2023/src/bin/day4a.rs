use std::collections::HashSet;

use aoc2023::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"Card\s+(\d+):(.*)\|(.*)").unwrap();
}

fn main() {
    println!("{:?}", solve(read_data_for_day("4").unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .split("\n")
        .flat_map(|line| {
            REGEX
                .captures_iter(line)
                .map(|values| Card {
                    winning_numbers: values[2]
                        .parse::<String>()
                        .unwrap()
                        .split_ascii_whitespace()
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>(),
                    numbers_on_elf_card: values[3]
                        .parse::<String>()
                        .unwrap()
                        .split_ascii_whitespace()
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>(),
                })
                .collect::<Vec<Card>>()
        })
        .map(|card| card.amount_of_winning_numbers_on_elf_card())
        .filter(|amount_of_winning_numbers_on_elf_card| amount_of_winning_numbers_on_elf_card > &0)
        .map(|amount_of_winning_numbers_on_elf_card| {
            (2 as u32).pow((amount_of_winning_numbers_on_elf_card - 1) as u32)
        })
        .sum::<u32>()
}

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    numbers_on_elf_card: HashSet<u32>,
}

impl Card {
    fn amount_of_winning_numbers_on_elf_card(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|winning_number| self.numbers_on_elf_card.contains(&winning_number))
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(13, solve(read_test_data_for_day("4-0").unwrap()));
    }
}
