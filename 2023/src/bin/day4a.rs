use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day(4).unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .split("\n")
        .map(|line| {
            line.split(":")
                .filter(|part| !part.contains("Card"))
                .filter_map(|card| {
                    card.split("|")
                        .map(|numbers| {
                            numbers
                                .split_whitespace()
                                .map(|number| number.parse::<u32>().unwrap())
                                .collect::<Vec<u32>>()
                        })
                        .collect_tuple()
                })
                .map(|(winning_numbers, numbers_elf)| {
                    winning_numbers
                        .iter()
                        .filter(|winning_number| numbers_elf.contains(winning_number))
                        .count()
                })
                .filter(|amount_of_winning_numbers| amount_of_winning_numbers >= &1)
                .map(|amount_of_winning_numbers| {
                    (2 as u32).pow((amount_of_winning_numbers - 1) as u32)
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(13, solve(read_test_data_for_day("4.txt").unwrap()));
    }
}
