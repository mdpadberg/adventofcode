use std::collections::HashSet;
use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

fn main() {
    println!("{}", solve(read_data_for_day("6").unwrap()));
}

fn solve(input: String) -> usize {
    input.as_bytes()
        .windows(4)
        .fold_while(
            4,
            |acc, x| {
                let unique: HashSet<&u8> = HashSet::from_iter(x);
                if unique.len() != 4 {
                    Continue(acc + 1)
                } else {
                    Done(acc)
                }
            },
        )
        .into_inner()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(7, solve(read_test_data_for_day("6-0").unwrap()));
    }
}
