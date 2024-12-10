use aoc2024::util::read_data_for_day;
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

static REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

fn main() {
    println!("{:?}", solve(read_data_for_day("3").unwrap()));
}

fn solve(input: String) -> u32 {
    REGEX
        .captures_iter(&input)
        .map(|capture| {
            let a = capture[1].parse::<u32>().unwrap();
            let b = capture[2].parse::<u32>().unwrap();
            a * b
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(161, solve(read_test_data_for_day("3-0").unwrap()));
    }
}
