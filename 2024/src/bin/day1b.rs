use std::collections::HashMap;

use aoc2024::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("1").unwrap()));
}

fn solve(input: String) -> u32 {
    let tuple_per_line: (Vec<_>, Vec<_>) = input
        .split("\n")
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip();
    let second_colomn_dedup_with_count = tuple_per_line
        .1
        .iter()
        .sorted()
        .dedup_with_count()
        .map(|(a, b)| (b, a as u32))
        .collect::<HashMap<&u32, u32>>();
    tuple_per_line
        .0
        .iter()
        .map(|a| match second_colomn_dedup_with_count.get(a) {
            Some(value) => value * a,
            None => 0 * a,
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(31, solve(read_test_data_for_day("1-0").unwrap()));
    }
}
