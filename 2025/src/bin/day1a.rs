use aoc2025::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("1").unwrap()));
}

fn solve(input: String) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2025::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(3, solve(read_test_data_for_day("1-0").unwrap()));
    }
}
