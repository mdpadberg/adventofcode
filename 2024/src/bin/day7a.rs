use std::collections::VecDeque;

use aoc2024::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("7").unwrap()));
}

fn solve(input: String) -> u128 {
    let equations: Vec<(u128, Vec<u128>)> = input
        .split("\n")
        .map(|line| line.split(":").collect_tuple().unwrap())
        .map(|(calibration_result, values)| {
            (
                calibration_result.parse::<u128>().unwrap(),
                values
                    .split_whitespace()
                    .map(|value| value.parse::<u128>().unwrap())
                    .collect::<Vec<u128>>(),
            )
        })
        .collect();
    equations
        .into_iter()
        .map(|equation| {
            let mut in_between_values = VecDeque::<u128>::new();
            for (i, number) in equation.1.iter().enumerate() {
                if i == 0 {
                    in_between_values.push_front(*number);
                } else {
                    for _ in 0..in_between_values.len() {
                        let value = in_between_values.pop_front().unwrap();
                        in_between_values.push_back(value * number);
                        in_between_values.push_back(value + number);
                    }
                }
            }
            if in_between_values.contains(&&equation.0) {
                equation.0
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(3749, solve(read_test_data_for_day("7-0").unwrap()));
    }
}
