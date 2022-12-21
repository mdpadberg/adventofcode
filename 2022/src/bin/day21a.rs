use std::collections::{HashMap, VecDeque};

use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day(21).unwrap()));
}

fn solve(input: String) -> i64 {
    let monkeys: HashMap<Monkey, Job> = input
        .split("\n")
        .map(|str| {
            let split_position = str.chars().find_position(|c| c == &':').unwrap();
            (
                Monkey(str.chars().take(split_position.0).collect()),
                Job::from(str.chars().skip(split_position.0 + 2).collect()),
            )
        })
        .collect();

    let mut todo: VecDeque<(Monkey, Job)> = monkeys
        .iter()
        .filter(|m| !m.1.is_number())
        .map(|m| (m.0.clone(), m.1.clone()))
        .collect();
    let mut result: HashMap<Monkey, i64> = monkeys
        .iter()
        .filter(|m| m.1.is_number())
        .map(|m| {
            (
                m.0.clone(),
                match m.1 {
                    Job::Number(number) => number.to_owned(),
                    Job::Operation(_, _, _) => panic!("Should not happen, AAAAAAAAAAH"),
                },
            )
        })
        .collect();
    while let Some((monkey, job)) = todo.pop_front() {
        match &job {
            Job::Operation(monkey_one, math, monkey_two) => {
                match (result.get(&monkey_one), result.get(&monkey_two)) {
                    (Some(a), Some(b)) => {
                        result.insert(monkey, math.perform(a, b));
                    },
                    (_, _) => todo.push_back((monkey, job)),
                }
            },
            Job::Number(_) => panic!("Should not happen, AAAAAAAAAAH"),
        }
    }
    result.into_iter().find(|(monkey, value)| monkey == &Monkey(String::from("root"))).unwrap().1
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Monkey(String);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Job {
    Number(i64),
    Operation(Monkey, Math, Monkey),
}

impl Job {
    fn from(string: String) -> Job {
        match string.contains(' ') {
            false => Job::Number(string.parse::<i64>().unwrap()),
            true => {
                let values = string.split_ascii_whitespace().collect::<Vec<&str>>();
                Job::Operation(
                    Monkey(values[0].to_string()),
                    Math::from(values[1].to_string()),
                    Monkey(values[2].to_string()),
                )
            }
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Job::Number(_) => true,
            Job::Operation(_, _, _) => false,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Math {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Math {
    fn from(string: String) -> Math {
        match string.as_str() {
            "+" => Math::Addition,
            "-" => Math::Subtraction,
            "*" => Math::Multiplication,
            "/" => Math::Division,
            _ => panic!("Should not happen, AAAAAAAAAAH"),
        }
    }

    fn perform(&self, a: &i64, b: &i64) -> i64 {
        match self {
            Math::Addition => a + b,
            Math::Subtraction => a - b,
            Math::Multiplication => a * b,
            Math::Division => a / b,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(152, solve(read_test_data_for_day(21).unwrap()));
    }
}
