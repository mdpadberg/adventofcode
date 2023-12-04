use anyhow::Context;
use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day(1).unwrap()));
}

fn solve(input: String) -> anyhow::Result<u32> {
    Ok(input
        .split("\n")
        .map(|line| {
            (
                line.chars().find(|char| char.is_digit(10)),
                line.chars().rfind(|char| char.is_digit(10)),
            )
        })
        .map(|(first_number, last_number)| {
            Ok(format!(
                "{}{}",
                first_number.context("no first number")?,
                last_number.context("no last number")?
            ))
        })
        .map_ok(|two_digit_number| two_digit_number.parse::<u32>().unwrap())
        .sum::<anyhow::Result<u32>>()?)
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(142, solve(read_test_data_for_day("1a.txt").unwrap()).unwrap());
    }
}
