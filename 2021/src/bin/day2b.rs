use std::str::FromStr;

use aoc2021::util::read_file_line_by_line_to_string;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!(
        "solve_part_two -> {:#?}",
        solve_part_two(read_file_line_by_line_to_string("2"))
    );
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\w+)\s(\d+)").unwrap();
}

fn solve_part_two(data: Vec<String>) -> u64 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    data.iter()
        .map(|line| REGEX.captures(line).unwrap())
        .map(|captures| SubmarineMovement::from_str(&captures[1], &captures[2]))
        .for_each(|submarine_movement| {
            if submarine_movement.command == Command::FORWARD {
                horizontal_position += submarine_movement.units;
                depth += submarine_movement.units * aim;
            }
            if submarine_movement.command == Command::DOWN {
                aim += submarine_movement.units;
            }
            if submarine_movement.command == Command::UP {
                aim -= submarine_movement.units;
            }
        });
    horizontal_position * depth
}

#[derive(Debug)]
struct SubmarineMovement {
    command: Command,
    units: u64,
}

#[derive(Debug, PartialEq)]
enum Command {
    FORWARD,
    DOWN,
    UP,
}

impl SubmarineMovement {
    fn from_str(command: &str, units: &str) -> SubmarineMovement {
        match Command::from_str(command) {
            Ok(ok) => SubmarineMovement {
                command: ok,
                units: units.parse::<u64>().unwrap(),
            },
            Err(err) => panic!("Error in Submarine from_str {:#?}", err),
        }
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "forward" => Ok(Command::FORWARD),
            "down" => Ok(Command::DOWN),
            "up" => Ok(Command::UP),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc2021::util::read_file_line_by_line_to_string_test;

    use super::*;

    #[test]
    fn two() {
        assert_eq!(900, solve_part_two(read_file_line_by_line_to_string_test("2-0")));
    }
}
