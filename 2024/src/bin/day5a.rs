use std::collections::HashMap;

use aoc2024::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("5").unwrap()));
}

fn solve(input: String) -> u32 {
    let (first, second) = input.split_once("\n\n").unwrap();
    let rules: HashMap<u32, Vec<u32>> = first
        .split("\n")
        .map(|line| line.split("|").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .into_group_map();
    let updates: Vec<Vec<u32>> = second
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|value| value.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    updates
        .iter()
        .filter(|update| is_valid_update(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_valid_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut is_valid = true;
    let mut printed_pages: Vec<&u32> = vec![];
    for page_number in update {
        if let Some(some) = rules.get(&page_number) {
            if some.iter().any(|value| printed_pages.contains(&value)) {
                is_valid = false;
                break;
            }
        }
        printed_pages.push(page_number);
    }
    is_valid
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(143, solve(read_test_data_for_day("5-0").unwrap()));
    }
}
