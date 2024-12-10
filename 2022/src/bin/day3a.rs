use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("3").unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .lines()
        .into_iter()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .flat_map(|compartment| {
            compartment
                .0
                .chars()
                .filter(|item_in_rucksack| compartment.1.contains(*item_in_rucksack))
                .dedup()
                .map(|item_in_rucksack| {
                    if item_in_rucksack.is_lowercase() {
                        item_in_rucksack as u32 - 96
                    } else {
                        item_in_rucksack as u32 - 38
                    }
                })
                .collect::<Vec<u32>>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use aoc2022::util::read_test_data_for_day;

    use super::*;

    #[test]
    fn solvetest() {
        assert_eq!(157, solve(read_test_data_for_day("3-0").unwrap()));
    }
}
