use aoc2024::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
}

fn main() {
    println!("{:?}", solve(read_data_for_day(3).unwrap()));
}

fn solve(input: String) -> u32 {
    let mut result = 0;
    let mut skip = false;
    for capture in REGEX.captures_iter(&input) {
        match &capture[0] {
            "do()" => {
                skip = false;
            },
            "don't()" => {
                skip = true;
            },
            _ => {
                if !skip {
                    let a = capture[1].parse::<u32>().unwrap();
                    let b = capture[2].parse::<u32>().unwrap();
                    result += a * b;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(48, solve(read_test_data_for_day("3b.txt").unwrap()));
    }
}
