use std::{collections::HashMap};

use aoc2021::util::read_file_line_by_line_to_string;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("14"))
    );
}

fn solve_part_one(input: Vec<String>) -> usize {
    let mut polymer = Polymer::create(input);
    for _ in 0..10 {
        polymer.step();
    }
    let (min, max) = polymer
        .template
        .iter()
        .sorted()
        .dedup_with_count()
        .map(|(amount, char)| amount)
        .minmax()
        .into_option()
        .unwrap();
    max - min
}

#[derive(Debug, PartialEq, Eq)]
struct Polymer {
    template: Vec<char>,
    pair: HashMap<(char, char), char>,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\w)(\w) -> (\w)").unwrap();
}

impl Polymer {
    fn create(input: Vec<String>) -> Polymer {
        Polymer {
            template: input[0].to_owned().chars().collect_vec(),
            pair: input[2..input.len()]
                .iter()
                .map(|line| REGEX.captures(line).unwrap())
                .map(|captures| {
                    (
                        (
                            captures[1].parse::<char>().unwrap(),
                            captures[2].parse::<char>().unwrap(),
                        ),
                        captures[3].parse::<char>().unwrap(),
                    )
                })
                .collect::<HashMap<(char, char), char>>(),
        }
    }

    fn step(&mut self) {
        let mut result = Vec::<char>::new();
        for chars in self.template.windows(2) {
            if let Some(extra_letter) = self.pair.get(&(chars[0], chars[1])) {
                result.push(chars[0]);
                result.push(extra_letter.to_owned());
            } else {
                result.push(chars[0]);
            }
        }
        result.push(self.template.last().unwrap().to_owned());
        self.template = result;
    }
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;
    
    #[test]
    fn one() {
        assert_eq!(1588, solve_part_one(read_file_line_by_line_to_string_test("14-0")));
    }

    #[test]
    fn step_test() {
        let mut polymer = Polymer::create(vec![
            String::from("NNCB"),
            String::from(""),
            String::from("CH -> B"),
            String::from("HH -> N"),
            String::from("CB -> H"),
            String::from("NH -> C"),
            String::from("HB -> C"),
            String::from("HC -> B"),
            String::from("HN -> C"),
            String::from("NN -> C"),
            String::from("BH -> H"),
            String::from("NC -> B"),
            String::from("NB -> B"),
            String::from("BN -> B"),
            String::from("BB -> N"),
            String::from("BC -> B"),
            String::from("CC -> N"),
            String::from("CN -> C"),
        ]);
        polymer.step();
        assert_eq!(vec!['N', 'C', 'N', 'B', 'C', 'H', 'B'], polymer.template);
        polymer.step();
        assert_eq!(
            vec!['N', 'B', 'C', 'C', 'N', 'B', 'B', 'B', 'C', 'B', 'H', 'C', 'B'],
            polymer.template
        );
        polymer.step();
        assert_eq!(
            vec![
                'N', 'B', 'B', 'B', 'C', 'N', 'C', 'C', 'N', 'B', 'B', 'N', 'B', 'N', 'B', 'B',
                'C', 'H', 'B', 'H', 'H', 'B', 'C', 'H', 'B'
            ],
            polymer.template
        );
        polymer.step();
        assert_eq!(
            vec![
                'N', 'B', 'B', 'N', 'B', 'N', 'B', 'B', 'C', 'C', 'N', 'B', 'C', 'N', 'C', 'C',
                'N', 'B', 'B', 'N', 'B', 'B', 'N', 'B', 'B', 'B', 'N', 'B', 'B', 'N', 'B', 'B',
                'C', 'B', 'H', 'C', 'B', 'H', 'H', 'N', 'H', 'C', 'B', 'B', 'C', 'B', 'H', 'C',
                'B'
            ],
            polymer.template
        );
    }
}
