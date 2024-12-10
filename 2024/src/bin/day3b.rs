use aoc2024::util::read_data_for_day;
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

static REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap());

fn main() {
    println!("{:?}", solve(read_data_for_day("3").unwrap()));
}

fn solve(input: String) -> u32 {
    REGEX
        .captures_iter(&input)
        .fold((0, false), |(sum, skip), capture| match &capture[0] {
            "do()" => (sum, false),
            "don't()" => (sum, true),
            _ => {
                if skip {
                    (sum, skip)
                } else {
                    let add =
                        capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap();
                    (sum + add, skip)
                }
            }
        })
        .0
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(48, solve(read_test_data_for_day("3-1").unwrap()));
    }
}
