use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("{}", solve(read_data_for_day("5").unwrap()));
}

fn solve(input: String) -> String {
    let moves: Vec<Move> = input
        .split("\n")
        .filter(|line| line.contains("move"))
        .map(|line| parse_moves(line))
        .collect();

    let mut current_crate_situation: Vec<Vec<char>> = input
        .split("\n")
        .take_while(|line| line.contains("["))
        .enumerate()
        .flat_map(|(line_number, line)| parse_crate(line_number, line))
        .sorted_by(|a, b| a.location_x.cmp(&b.location_x))
        .group_by(|a| a.location_x)
        .into_iter()
        .map(|(_, group)| {
            group
                .into_iter()
                .map(|char| char.content.unwrap())
                .filter(|char| *char != ' ')
                .into_iter()
                .collect::<Vec<char>>()
                .into_iter()
                .rev()
                .collect::<Vec<char>>()
        })
        .collect();
    for m in moves {
        for _ in 0..m.amount {
            let crate_that_needs_to_be_moved = current_crate_situation[m.from-1].pop().unwrap();
            current_crate_situation[m.to-1].push(crate_that_needs_to_be_moved);
        }
    }
    current_crate_situation.into_iter()
        .map(|mut a| a.pop().unwrap())
        .collect::<String>()
}

///Input format
///    [D]    
///[N] [C]    
///[Z] [M] [P]
fn parse_crate(line_number: usize, line: &str) -> Vec<Crate> {
    line.chars()
        .chunks(4)
        .into_iter()
        .enumerate()
        .map(|(column, mut line)| Crate {
            location_x: column,
            location_y: line_number,
            content: line.nth(1),
        })
        .collect::<Vec<Crate>>()
}

lazy_static! {
    static ref MOVE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

///move 1 from 2 to 1
///move 3 from 1 to 3
///move 2 from 2 to 1
fn parse_moves(line: &str) -> Move {
    *MOVE
        .captures_iter(line)
        .map(|values| Move {
            amount: values[1].parse::<usize>().unwrap(),
            from: values[2].parse::<usize>().unwrap(),
            to: values[3].parse::<usize>().unwrap(),
        })
        .collect::<Vec<Move>>()
        .first()
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone, Copy)]
struct Crate {
    location_x: usize,
    location_y: usize,
    content: Option<char>,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(String::from("CMZ"), solve(read_test_data_for_day("5-0").unwrap()));
    }
}
