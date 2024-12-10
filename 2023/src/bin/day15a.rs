use aoc2023::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{:?}", solve(read_data_for_day("15").unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .split(",")
        .map(|value| {
            value.as_bytes().iter().fold(0, |acc, char_as_byte| {
                ((acc + *char_as_byte as u32) * 17) % 256
            })
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(1320, solve(read_test_data_for_day("15-0").unwrap()));
    }
}
