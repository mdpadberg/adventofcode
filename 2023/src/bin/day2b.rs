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

fn solve(input: String) -> u32 {
    input
        .split("\n")
        .map(|game_data| {
            REGEX
                .captures_iter(game_data)
                .map(|values| {
                    (
                        values[2].parse::<u32>().unwrap(),
                        values[3].parse::<String>().unwrap(),
                    )
                })
                .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
                .group_by(|(amount, color)| color.clone())
                .into_iter()
                .map(|(color, group)| (color, group.map(|(count, color)| count).max()))
                .filter_map(|(_, max)| max)
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(2286, solve(read_test_data_for_day("2-0").unwrap()));
    }
}
