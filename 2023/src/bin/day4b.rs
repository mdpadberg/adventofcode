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
    let cards = input
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
        .collect::<Vec<_>>();
    let mut amount_of_cards = vec![1u32; cards.len()];
    cards.into_iter().enumerate().for_each(|(i, card)| {
        let amount_of_card = amount_of_cards[i];
        let amount_of_winning_numbers_on_elf_card = card.amount_of_winning_numbers_on_elf_card();
        let next_card_indices = i + 1..=i + amount_of_winning_numbers_on_elf_card as usize;
        for next_card_index in next_card_indices {
            amount_of_cards[next_card_index] += amount_of_card;
        }
    });
    amount_of_cards.iter().sum()
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
        assert_eq!(30, solve(read_test_data_for_day("4-0").unwrap()));
    }
}
