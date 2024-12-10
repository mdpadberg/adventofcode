use std::collections::{HashMap, VecDeque};

use aoc2022::util::read_data_for_day;
use evalexpr::eval_int;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("21").unwrap()));
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
    let arithmetic_experssion = to_arithmetic(Monkey(String::from("root")), monkeys);
    eval_int(&arithmetic_experssion).unwrap()
}

fn to_arithmetic(start_monkey: Monkey, monkeys: HashMap<Monkey, Job>) -> String {
    let mut monkeys_to_check: VecDeque<Monkey> = VecDeque::from_iter(vec![start_monkey.clone()]);
    let mut result = monkeys.get(&start_monkey).unwrap().as_string();
    while let Some(monkey) = monkeys_to_check.pop_front() {
        let job = monkeys.get(&monkey).unwrap();
        match &job {
            Job::Number(_) => result = result.replace(&monkey.0, &job.as_string()),
            Job::Operation(left, _, right) => {
                result = result.replace(
                    &monkey.0,
                    vec!["(", &job.as_string(), ")"].concat().as_str(),
                );
                monkeys_to_check.push_back(left.to_owned());
                monkeys_to_check.push_back(right.to_owned());
            }
        }
    }
    result
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Monkey(String);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Job {
    Number(i64),
    Operation(Monkey, String, Monkey),
}

impl Job {
    fn from(string: String) -> Job {
        match string.contains(' ') {
            false => Job::Number(string.parse::<i64>().unwrap()),
            true => {
                let values = string.split_ascii_whitespace().collect::<Vec<&str>>();
                Job::Operation(
                    Monkey(values[0].to_string()),
                    values[1].to_string(),
                    Monkey(values[2].to_string()),
                )
            }
        }
    }

    fn as_string(&self) -> String {
        match self {
            Job::Number(i) => i.to_string(),
            Job::Operation(left, operation, right) => {
                vec![left.0.to_owned(), operation.to_owned(), right.0.to_owned()]
                    .concat()
                    .to_string()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(152, solve(read_test_data_for_day("21-0").unwrap()));
    }
}
