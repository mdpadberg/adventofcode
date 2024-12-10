use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("11").unwrap()));
}

fn solve(input: String) -> u128 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| Monkey::parse(monkey))
        .collect();
    let modulus: u128 = monkeys.iter().map(|m| m.test.divisible).product();

    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let mut new_situation = monkeys.clone();
            let current_monkey = monkeys.get(i).unwrap();
            let new_items = current_monkey
                .starting_items
                .iter()
                .map(|item| current_monkey.operation.perform(item) % modulus)
                .collect::<Vec<_>>();
            for item in &new_items {
                let new_monkey = if item % current_monkey.test.divisible == 0 {
                    new_situation
                        .get_mut(current_monkey.test.if_true_new_monkey as usize)
                        .unwrap()
                } else {
                    new_situation
                        .get_mut(current_monkey.test.if_false_new_monkey as usize)
                        .unwrap()
                };
                new_monkey.starting_items.push(*item);
            }
            new_situation.get_mut(i).unwrap().items_inspected += new_items.len() as u128;
            new_situation.get_mut(i).unwrap().starting_items.clear();
            monkeys = new_situation;
        }
    }
    monkeys
        .iter()
        .sorted_by(|a, b| b.items_inspected.cmp(&a.items_inspected))
        .take(2)
        .map(|m| m.items_inspected)
        .product()
}

#[derive(Debug, Clone)]
struct Monkey {
    number: u128,
    starting_items: Vec<u128>,
    operation: Operation,
    test: Test,
    items_inspected: u128,
}

impl Monkey {
    fn parse(input: &str) -> Monkey {
        let lines: Vec<String> = input.split("\n").map(|str| str.to_string()).collect();
        Monkey {
            number: lines[0]
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .replace(":", "")
                .parse::<u128>()
                .unwrap(),
            starting_items: lines[1]
                .chars()
                .skip(18)
                .collect::<String>()
                .split(",")
                .map(|value: &str| value.trim().parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
            operation: Operation::parse(lines[2].chars().skip(23).collect()),
            test: Test::parse(&lines[3..]),
            items_inspected: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Amount {
    value(u128),
    old,
}

#[derive(Debug, Clone)]
enum Operation {
    Addition(Amount),
    Subtraction(Amount),
    Multiplication(Amount),
    Division(Amount),
    Modulus(Amount),
}

impl Operation {
    fn parse(input: String) -> Operation {
        match input.split_ascii_whitespace().collect_tuple() {
            Some(("+", "old")) => Operation::Addition(Amount::old),
            Some(("-", "old")) => Operation::Subtraction(Amount::old),
            Some(("*", "old")) => Operation::Multiplication(Amount::old),
            Some(("/", "old")) => Operation::Division(Amount::old),
            Some(("%", "old")) => Operation::Modulus(Amount::old),
            Some(("+", i)) => Operation::Addition(Amount::value(i.parse::<u128>().unwrap())),
            Some(("-", i)) => Operation::Subtraction(Amount::value(i.parse::<u128>().unwrap())),
            Some(("*", i)) => Operation::Multiplication(Amount::value(i.parse::<u128>().unwrap())),
            Some(("/", i)) => Operation::Division(Amount::value(i.parse::<u128>().unwrap())),
            Some(("%", i)) => Operation::Modulus(Amount::value(i.parse::<u128>().unwrap())),
            _ => panic!("AAAAAAAAH!"),
        }
    }

    fn perform(&self, input: &u128) -> u128 {
        match self {
            Operation::Addition(a) => match a {
                Amount::value(i) => input + i,
                Amount::old => input + input,
            },
            Operation::Subtraction(a) => match a {
                Amount::value(i) => input - i,
                Amount::old => input - input,
            },
            Operation::Multiplication(a) => match a {
                Amount::value(i) => input * i,
                Amount::old => input * input,
            },
            Operation::Division(a) => match a {
                Amount::value(i) => input / i,
                Amount::old => input / input,
            },
            Operation::Modulus(a) => match a {
                Amount::value(i) => input % i,
                Amount::old => input % input,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    divisible: u128,
    if_true_new_monkey: u128,
    if_false_new_monkey: u128,
}

impl Test {
    fn parse(input: &[String]) -> Test {
        Test {
            divisible: input[0]
                .chars()
                .skip(21)
                .collect::<String>()
                .parse::<u128>()
                .unwrap(),
            if_true_new_monkey: input[1]
                .chars()
                .skip(29)
                .collect::<String>()
                .parse::<u128>()
                .unwrap(),
            if_false_new_monkey: input[2]
                .chars()
                .skip(30)
                .collect::<String>()
                .parse::<u128>()
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(2713310158, solve(read_test_data_for_day("11-0").unwrap()));
    }
}
