use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("9").unwrap()));
}

fn solve(input: String) -> i32 {
    input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|line| calculate_difference_between_every_number_in_the_array(line))
        .map(|sequences| {
            sequences
                .iter()
                .map(|sequence| sequence[sequence.len() - 1])
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn calculate_difference_between_every_number_in_the_array(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sequences = vec![input.to_owned()];
    while let Some(sequence) = sequences
        .last()
        .and_then(|sequence| sequence.iter().any(|value| value != &0).then_some(sequence))
    {
        sequences.push(
            sequence
                .windows(2)
                .map(|pairs| pairs[1] - pairs[0])
                .collect::<Vec<i32>>(),
        );
    }
    sequences
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(114, solve(read_test_data_for_day("9-0").unwrap()));
    }
}
