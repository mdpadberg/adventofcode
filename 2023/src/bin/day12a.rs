use std::collections::{HashMap, VecDeque};

use aoc2023::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(#+)").unwrap();
}

fn main() {
    println!("{:?}", solve(read_data_for_day("12").unwrap()));
}

fn solve(input: String) -> usize {
    let records = input
        .split("\n")
        .map(|line| {
            let lines = line.split(" ").collect::<Vec<&str>>();
            Record {
                data: RecordData(lines[0].to_string()),
                group_numbers: RecordNumbers(lines[1].to_string()),
            }
        })
        .collect::<Vec<Record>>();
    records
        .iter()
        .map(|record| per_line(record.to_owned()))
        .sum()
}

fn per_line(record: Record) -> usize {
    let mut cache: HashMap<RecordData, RecordNumbers> = HashMap::new();
    let mut todo: VecDeque<Record> = VecDeque::new();
    todo.push_front(record);
    while let Some(record) = todo.pop_front() {
        if !cache.contains_key(&record.data) {
            cache.insert(record.data.clone(), record.group_numbers.clone());
            if record.data.0.contains("?") {
                todo.push_back(Record {
                    data: RecordData(record.data.0.replacen("?", "#", 1)),
                    group_numbers: record.group_numbers.clone(),
                });
                todo.push_back(Record {
                    data: RecordData(record.data.0.replacen("?", ".", 1)),
                    group_numbers: record.group_numbers.clone(),
                });
            }
        }
    }
    cache
        .iter()
        .filter(|(a, b)| !a.0.contains("?"))
        .filter(|(a, b)| calculate_group_numbers(&a.0) == b.0)
        .count()
}

fn calculate_group_numbers(line: &str) -> String {
    REGEX
        .captures_iter(line)
        .flat_map(|captures| {
            captures
                .iter()
                .skip(1)
                .filter_map(|value| value.and_then(|a| Some(a.as_str())))
                .collect::<Vec<_>>()
        })
        .map(|group| group.len())
        .join(",")
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct RecordData(String);

#[derive(Debug, PartialEq, Eq, Clone)]
struct RecordNumbers(String);

#[derive(Debug, Clone)]
struct Record {
    data: RecordData,
    group_numbers: RecordNumbers,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn calculate_groups_test() {
        let input = r#"#.#.###
.#...#....###.
.#.###.#.######
####.#...#...
#....######..#####.
.###.##....#"#;
        assert_eq!(
            vec![
                String::from("1,1,3"),
                String::from("1,1,3"),
                String::from("1,3,1,6"),
                String::from("4,1,1"),
                String::from("1,6,5"),
                String::from("3,2,1"),
            ],
            input
                .split("\n")
                .map(|line| calculate_group_numbers(line))
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn solvetest() {
        assert_eq!(21, solve(read_test_data_for_day("12-1").unwrap()));
    }
}
