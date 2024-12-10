use std::cmp::Ordering;

use aoc2021::util::read_file_line_by_line_to_string;
use itertools::Itertools;

fn main() {
    println!(
        "solve_part_one -> {:#?}",
        solve_part_two(read_file_line_by_line_to_string("8"))
    );
}

fn solve_part_two(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| line.split(" | ").collect::<Vec<&str>>())
        .map(|split| map_seven_segment_display_line_to_output(split[0], split[1]))
        .sum()
}

fn seven_segment_display_is_one_four_seven_eight(str: &&str) -> bool {
    let length = str.len();
    length == 2 || length == 4 || length == 3 || length == 7
}

fn map_seven_segment_display_line_to_output(
    before_delimiter: &str,
    after_delimiter: &str,
) -> usize {
    let before_delimiter: Vec<Signal> =
        map_seven_segment_display_before_delimiter_to_numbers(before_delimiter);
    after_delimiter
        .split_whitespace()
        .map(|str| str.chars().sorted().collect::<Vec<char>>())
        .flat_map(|chars| {
            before_delimiter
                .iter()
                .filter(|signal| chars.eq(&signal.segment.chars().sorted().collect::<Vec<char>>()))
                .map(|signal| signal.number)
                .collect::<Vec<usize>>()
        })
        .join("")
        .parse()
        .unwrap()
}

///   0:      1:      2:      3:      4:
///  dddd    ....    dddd    dddd    ....
/// e    a  .    a  .    a  .    a  e    a
/// e    a  .    a  .    a  .    a  e    a
///  ....    ....    ffff    ffff    ffff
/// g    b  .    b  g    .  .    b  .    b
/// g    b  .    b  g    .  .    b  .    b
///  cccc    ....    cccc    cccc    ....
///
///   5:      6:      7:      8:      9:
///  dddd    dddd    dddd    dddd    dddd
/// e    .  e    .  .    a  e    a  e    a
/// e    .  e    .  .    a  e    a  e    a
///  ffff    ffff    ....    ffff    ffff
/// .    b  g    b  .    b  g    b  .    b
/// .    b  g    b  .    b  g    b  .    b
///  cccc    cccc    ....    cccc    cccc
///
/// based on amount of segments 1,4,7,8 are a sure thing
/// 5 segments can be
///     - 2 if 1 same segment as 1 && if 2 same segments as 4 && if 2 same segments as 7
///     - 3 if 2 same segments as 1 && if 3 same segments as 4 && if 3 same segments as 7
///     - 5 if 1 same segment as 1 && if 3 same segments as 4 && if 2 same segments as 7
/// 6 segements can be
///     - 0 if 2 same segements as 1 && if 3 same segements as 4 && if 3 same segements as 7
///     - 6 if 1 same segements as 1 && if 3 same segements as 4 && if 2 same segements as 7
///     - 9 if 2 same segements as 1 && if 4 same segements as 4 && if 3 same segements as 7
///
/// Input for this function: acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab
fn map_seven_segment_display_before_delimiter_to_numbers(before_delimiter: &str) -> Vec<Signal> {
    let words: Vec<&str> = before_delimiter.split_whitespace().collect::<Vec<&str>>();
    let (one_four_seven_eight, zero_two_three_five_six_nine): (Vec<&str>, Vec<&str>) = words
        .into_iter()
        .partition(seven_segment_display_is_one_four_seven_eight);
    let mut known_numbers: Vec<Signal> = one_four_seven_eight
        .into_iter()
        .map(|segment| segment.to_string())
        .map(map_known_seven_segment_display_to_usize)
        .collect();
    known_numbers.sort();
    let unknown_numbers: Vec<Signal> = zero_two_three_five_six_nine
        .into_iter()
        .map(|segment| segment.to_string())
        .map(|segment| {
            map_unknown_seven_segments_based_on_known_seven_segments_to_usize(
                segment,
                &known_numbers,
            )
        })
        .collect();
    known_numbers.extend(unknown_numbers);
    known_numbers
}

fn map_known_seven_segment_display_to_usize(segment: String) -> Signal {
    match segment.len() {
        2 => Signal { segment, number: 1 },
        3 => Signal { segment, number: 7 },
        4 => Signal { segment, number: 4 },
        7 => Signal { segment, number: 8 },
        _ => panic!("problem with seven_segment_display_is_one_four_seven_eight filter"),
    }
}

fn map_unknown_seven_segments_based_on_known_seven_segments_to_usize(
    segment: String,
    known: &Vec<Signal>,
) -> Signal {
    let one = &known[0].segment;
    let four = &known[1].segment;
    let seven = &known[2].segment;
    let eight = &known[3].segment;
    match segment.len() {
        5 => {
            if segment.chars().filter(|&char| one.contains(char)).count() == 2 {
                return Signal { segment, number: 3 };
            } else if segment.chars().filter(|&char| four.contains(char)).count() == 3 {
                return Signal { segment, number: 5 };
            } else {
                return Signal { segment, number: 2 };
            }
        }
        6 => {
            if segment.chars().filter(|&char| four.contains(char)).count() == 4 {
                return Signal { segment, number: 9 };
            } else if segment.chars().filter(|&char| seven.contains(char)).count() == 2 {
                return Signal { segment, number: 6 };
            } else {
                return Signal { segment, number: 0 };
            }
        }
        _ => panic!("problem with seven_segment_display_is_one_four_seven_eight filter"),
    }
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
    fn two() {
        assert_eq!(61229, solve_part_two(read_file_line_by_line_to_string_test("8-2")));
    }

    #[test]
    fn map_seven_segment_display_line_to_output_test() {
        assert_eq!(
            5353,
            map_seven_segment_display_line_to_output(
                &"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
                &"cdfeb fcadb cdfeb cdbaf"
            )
        );
    }

    #[test]
    fn map_seven_segment_display_before_delimiter_to_numbers_test() {
        let mut expected_output = vec![
            Signal {
                segment: String::from("acedgfb"),
                number: 8,
            },
            Signal {
                segment: String::from("cdfbe"),
                number: 5,
            },
            Signal {
                segment: String::from("gcdfa"),
                number: 2,
            },
            Signal {
                segment: String::from("fbcad"),
                number: 3,
            },
            Signal {
                segment: String::from("dab"),
                number: 7,
            },
            Signal {
                segment: String::from("cefabd"),
                number: 9,
            },
            Signal {
                segment: String::from("cdfgeb"),
                number: 6,
            },
            Signal {
                segment: String::from("eafb"),
                number: 4,
            },
            Signal {
                segment: String::from("cagedb"),
                number: 0,
            },
            Signal {
                segment: String::from("ab"),
                number: 1,
            },
        ];
        let mut actual_output = map_seven_segment_display_before_delimiter_to_numbers(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
        );
        expected_output.sort();
        actual_output.sort();

        assert_eq!(expected_output, actual_output);
    }
}
