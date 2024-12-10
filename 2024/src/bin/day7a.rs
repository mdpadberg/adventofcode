use std::collections::VecDeque;

use aoc2024::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day(7).unwrap()));
}

fn solve(input: String) -> usize {
    let equations: Vec<(u32, Vec<u32>)> = input
        .split("\n")
        .map(|line| line.split(":").collect_tuple().unwrap())
        .map(|(calibration_result, values)| {
            (
                calibration_result.parse::<u32>().unwrap(),
                values
                    .split_whitespace()
                    .map(|value| value.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .collect();
    let mut values: VecDeque<u32> = equations[0].1.clone().into();
    let mut in_between_values = VecDeque::<u32>::new();
    while let Some(some) = values.pop_front() {
        if in_between_values.len() == 0 {
            in_between_values.push_back(some);
        } else {
            while let Some(some) =  {
                
            }
        }
    }
    println!("{in_between_values:?}");
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(3267, solve(read_test_data_for_day("7.txt").unwrap()));
    }
}
