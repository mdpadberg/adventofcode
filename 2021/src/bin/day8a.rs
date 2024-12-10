use std::cmp::Ordering;
use aoc2021::util::read_file_line_by_line_to_string;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_one(read_file_line_by_line_to_string("8"))
    );
}

fn solve_part_one(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| line.split(" | ").collect::<Vec<&str>>())
        .map(|split| split[1])
        .map(|str| str.to_string())
        .map(|part_after_delimiter| {
            part_after_delimiter
                .split_whitespace()
                .filter(seven_segment_display_is_one_four_seven_eight)
                .collect::<Vec<&str>>()
                .len()
        })
        .sum()
}

fn seven_segment_display_is_one_four_seven_eight(str: &&str) -> bool {
    let length = str.len();
    length == 2 || length == 4 || length == 3 || length == 7
}

#[derive(Debug)]
struct Signal {
    segment: String,
    number: usize,
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Signal {}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.segment == other.segment
    }
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;

    #[test]
    fn one() {
        assert_eq!(26, solve_part_one(read_file_line_by_line_to_string_test("8-2")));
    }
}
