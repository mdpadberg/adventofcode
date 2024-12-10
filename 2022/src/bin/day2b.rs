use std::collections::HashMap;
use aoc2022::util::read_data_for_day;

fn main() {
    println!("{}", solve(read_data_for_day("2").unwrap()));
}

fn solve(input: String) -> u64 {
    input
        .split("\n")
        .map(|str| str.parse::<String>().unwrap())
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .map(|round| rules(round))
        .sum()
}

/// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
/// A for Rock, B for Paper, and C for Scissors
/// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
/// Shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
/// Plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
fn rules(round_input: (char, char)) -> u64 {
    let scores: HashMap<(char, char), u64> = HashMap::from([
        (('A', 'X'), 3),
        (('B', 'X'), 1),
        (('C', 'X'), 2),
        (('A', 'Y'), 4),
        (('B', 'Y'), 5),
        (('C', 'Y'), 6),
        (('A', 'Z'), 8),
        (('B', 'Z'), 9),
        (('C', 'Z'), 7),
    ]);
    scores.get(&round_input).unwrap().to_owned()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(12, solve(read_test_data_for_day("2-0").unwrap()));
    }
}
