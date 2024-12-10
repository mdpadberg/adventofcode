use aoc2023::util::read_data_for_day;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("{:?}", solve(read_data_for_day("8").unwrap()));
}

fn solve(input: String) -> usize {
    let instructions = input
        .split("\n")
        .nth(0)
        .unwrap()
        .chars()
        .map(|char| {
            if char == 'R' {
                Direction::RIGHT
            } else {
                Direction::LEFT
            }
        })
        .collect::<Vec<Direction>>();
    let nodes = input
        .split("\n")
        .skip(2)
        .map(|line| {
            let values_in_line = line.split("=").collect::<Vec<&str>>();
            let starting_node = values_in_line.get(0).unwrap().trim();
            let next_nodes = values_in_line
                .get(1)
                .unwrap()
                .split(",")
                .map(|value| value.replace("(", " ").replace(")", " ").trim().to_string())
                .collect::<Vec<String>>();
            (
                StartingNode(starting_node.to_string()),
                NextElement((
                    next_nodes.get(0).unwrap().to_string(),
                    next_nodes.get(1).unwrap().to_string(),
                )),
            )
        })
        .collect::<HashMap<StartingNode, NextElement>>();
    let mut counter = 0;
    let mut current_node = nodes.get(&StartingNode("AAA".to_string())).unwrap();
    while let Some(direction) = instructions.get(counter % instructions.len()) {
        let next_node_to_take = match direction {
            Direction::LEFT => &current_node.0 .0,
            Direction::RIGHT => &current_node.0 .1,
        };
        current_node = nodes
            .get(&StartingNode(next_node_to_take.to_owned()))
            .unwrap();
        counter += 1;
        if next_node_to_take == "ZZZ" {
            break;
        }
    }
    counter
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct StartingNode(String);

#[derive(Debug)]
struct NextElement((String, String));

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(2, solve(read_test_data_for_day("8-0").unwrap()));
        assert_eq!(6, solve(read_test_data_for_day("8-1").unwrap()));
    }
}
