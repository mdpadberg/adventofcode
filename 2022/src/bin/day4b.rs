use std::ops::Range;

use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

fn main() {
    println!("{}", solve(read_data_for_day("4").unwrap()));
}

fn solve(input: String) -> usize {
    input
        .split("\n")
        .flat_map(|line| {
            REGEX
                .captures_iter(line)
                .map(|values| CampCleanUp {
                    cleaning_duty_elf_one: Range {
                        start: values[1].parse::<u64>().unwrap(),
                        end: values[2].parse::<u64>().unwrap(),
                    },
                    cleaning_duty_elf_two: Range {
                        start: values[3].parse::<u64>().unwrap(),
                        end: values[4].parse::<u64>().unwrap(),
                    },
                })
                .filter(|campcleanup| {
                    range_is_subset_of_other_range(
                        &campcleanup.cleaning_duty_elf_one,
                        &campcleanup.cleaning_duty_elf_two,
                    ) || range_overlaps_with_other_range(
                        &campcleanup.cleaning_duty_elf_one,
                        &campcleanup.cleaning_duty_elf_two,
                    )
                })
                .collect::<Vec<CampCleanUp>>()
        })
        .count()
}

/// 2-8, 3-7 range b is subset of range a
/// 3-7, 2-8 range a is subset of range b
/// 5-7, 7-9 range a is not a full subset of range b and also not the other way around
fn range_is_subset_of_other_range(a: &std::ops::Range<u64>, b: &std::ops::Range<u64>) -> bool {
    ((b.start <= a.end && b.start >= a.start) && (b.end >= a.start && b.end <= a.end))
        || ((a.start >= b.start && a.start <= b.end) && (a.end <= b.end && a.end >= b.start))
}

/// 5-7, 7-9 range a overlaps with range b at point 7
/// 7,9, 5-7 range a overlaps with range b at point 7
/// 2-4, 6-8 range a does not overlap with range b
fn range_overlaps_with_other_range(a: &std::ops::Range<u64>, b: &std::ops::Range<u64>) -> bool {
    (a.end >= b.start && a.end <= b.end) || (a.start <= b.end && a.start >= b.start)
}

#[derive(Debug)]
struct CampCleanUp {
    cleaning_duty_elf_one: std::ops::Range<u64>,
    cleaning_duty_elf_two: std::ops::Range<u64>,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(4, solve(read_test_data_for_day("4-0").unwrap()));
    }
}
