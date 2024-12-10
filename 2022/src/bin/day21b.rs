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
        .filter(|(monkey, job)| monkey != &Monkey(String::from("humn")))
        .collect();
    let root = monkeys
        .iter()
        .find(|(monkey, job)| monkey == &&Monkey(String::from("root")))
        .unwrap();
    let (left, right) = match root.1 {
        Job::Number(_) => panic!("Root monkey cannot be a number, AAAAAAAAAAAAH"),
        Job::Operation(left, _, right) => (left.to_owned(), right.to_owned()),
    };
    let (left, right) = (
        to_arithmetic(left, &monkeys),
        to_arithmetic(right, &monkeys),
    );
    if left.contains("x") {
        println!("{:?}", vec![eval_int(&right).unwrap().to_string(), " = ".to_string(), left].concat().as_str());
    } else {
        println!("{:?}", vec![eval_int(&left).unwrap().to_string(), " = ".to_string(), right].concat().as_str());
    }
    println!("go to https://quickmath.com to find x");
    0
}

fn to_arithmetic(start_monkey: Monkey, monkeys: &HashMap<Monkey, Job>) -> String {
    let mut monkeys_to_check: VecDeque<Monkey> = VecDeque::from_iter(vec![start_monkey.clone()]);
    let mut result = monkeys.get(&start_monkey).unwrap().as_string();
    while let Some(monkey) = monkeys_to_check.pop_front() {
        if let Some(job) = monkeys.get(&monkey) {
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
        } else {
            //replace unknown monkies to x (for example humn monkey)
            result = result.replace(&monkey.0, "x");
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
    fn solvetest_one() {
        assert_eq!(0, solve(read_test_data_for_day("21-0").unwrap()));
    }
}
