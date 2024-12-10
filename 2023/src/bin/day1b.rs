use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("1").unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let first_number = options()
                .into_iter()
                .map(|(string_version_of_number, number)| (line.find(string_version_of_number), number))
                .filter(|(position_of_number, number)| position_of_number.is_some())
                .min_by_key(|(position_of_number, number)| position_of_number.to_owned())
                .unwrap()
                .1;
            let last_number = options()
                .into_iter()
                .map(|(string_version_of_number, number)| (line.rfind(string_version_of_number), number))
                .filter(|(position_of_number, number)| position_of_number.is_some())
                .max_by_key(|(position_of_number, number)| position_of_number.to_owned())
                .unwrap()
                .1;
            format!("{}{}", first_number, last_number)
        })
        .map(|two_digit_number| two_digit_number.parse::<u32>().unwrap())
        .sum::<u32>()
}

fn options() -> Vec<(&'static str, u32)> {
    vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(281, solve(read_test_data_for_day("1-1").unwrap()));
    }
}
