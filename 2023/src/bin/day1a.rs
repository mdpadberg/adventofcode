use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("1").unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .split("\n")
        .map(|line| {
            (
                line.chars().find(|char| char.is_digit(10)).unwrap(),
                line.chars().rfind(|char| char.is_digit(10)).unwrap(),
            )
        })
        .map(|(first_number, last_number)| format!("{}{}", first_number, last_number))
        .map(|two_digit_number| two_digit_number.parse::<u32>().unwrap())
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(142, solve(read_test_data_for_day("1-0").unwrap()));
    }
}
