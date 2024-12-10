use aoc2023::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"((\d+) (\w+))+").unwrap();
}

fn main() {
    println!("{:?}", solve(read_data_for_day("2").unwrap()));
}

fn solve(input: String) -> usize {
    input
        .split("\n")
        .enumerate()
        .map(|(game_number, game_data)| {
            let valid_chucks = REGEX
                .captures_iter(game_data)
                .map(|values| {
                    (
                        values[2].parse::<u32>().unwrap(),
                        values[3].parse::<String>().unwrap(),
                    )
                })
                .map(|(amount, color)| valid_game(amount, &color))
                .collect::<Vec<bool>>();
            if !valid_chucks.contains(&false) {
                game_number + 1
            } else {
                0
            }
        })
        .sum()
}

fn valid_game(amount: u32, color: &str) -> bool {
    match color {
        "red" if amount <= 12 => true,
        "green" if amount <= 13 => true,
        "blue" if amount <= 14 => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(8, solve(read_test_data_for_day("2-0").unwrap()));
    }
}
