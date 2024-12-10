use aoc2024::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("1").unwrap()));
}

fn solve(input: String) -> u32 {
    let tuple_per_line: (Vec<_>, Vec<_>) = input
        .split("\n")
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip();
    tuple_per_line.0
        .iter()
        .sorted()
        .zip(tuple_per_line.1.iter().sorted())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(11, solve(read_test_data_for_day("1-0").unwrap()));
    }
}
