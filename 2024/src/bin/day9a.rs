use aoc2024::util::read_data_for_day;
use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    println!("{:?}", solve(read_data_for_day("9").unwrap()));
}

fn solve(input: String) -> u128 {
    let mut blocks = input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(i, number)| {
            (0..number)
                .map(|_| {
                    if i % 2 == 0 {
                        (i / 2).to_string()
                    } else {
                        String::from(".")
                    }
                })
                .collect::<VecDeque<String>>()
        })
        .collect::<VecDeque<String>>();
    let mut checksum = vec![];
    while let Some(first_block) = blocks.pop_front() {
        let block = if first_block != "." {
            first_block
        } else {
            get_last(&mut blocks)
        };
        checksum.push(block);
    }
    checksum
        .into_iter()
        .enumerate()
        .map(|(i, block)| i as u128 * block.parse::<u128>().unwrap())
        .sum()
}

fn get_last(blocks: &mut VecDeque<String>) -> String {
    loop {
        let last_block = blocks.pop_back().unwrap();
        if last_block != "." {
            return last_block;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(1928, solve(read_test_data_for_day("9-0").unwrap()));
    }
}
