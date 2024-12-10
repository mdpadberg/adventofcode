use aoc2022::util::read_data_for_day;
use std::collections::HashMap;

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
/// X for Rock, Y for Paper, and Z for Scissors
/// Shape you selected (1 for X, 2 for Y, and 3 for Z)
/// Plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
fn rules(round_input: (char, char)) -> u64 {
    let scores: HashMap<(char, char), u64> = HashMap::from([
        (('A', 'X'), 4),
        (('B', 'X'), 1),
        (('C', 'X'), 7),
        (('A', 'Y'), 8),
        (('B', 'Y'), 5),
        (('C', 'Y'), 2),
        (('A', 'Z'), 3),
        (('B', 'Z'), 9),
        (('C', 'Z'), 6),
    ]);
    scores.get(&round_input).unwrap().to_owned()
}

#[cfg(test)]
mod test {
    use aoc2022::util::read_test_data_for_day;

    use super::*;

    #[test]
    fn solvetest() {
        assert_eq!(15, solve(read_test_data_for_day("2-0").unwrap()));
    }
}
