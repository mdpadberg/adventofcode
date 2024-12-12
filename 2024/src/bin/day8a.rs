use aoc2024::util::read_data_for_day;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("{:?}", solve(read_data_for_day("8").unwrap()));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates {
    x: i32,
    y: i32,
}

type Frequency = char;

fn solve(input: String) -> usize {
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let grid: HashMap<Frequency, Vec<Coordinates>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            max_y += 1;
            max_x += 1;
            line.chars()
                .enumerate()
                .filter_map(|(x, char)| {
                    if char.is_ascii_alphabetic() || char.is_ascii_digit() {
                        Some((
                            char,
                            Coordinates {
                                x: x as i32,
                                y: y as i32,
                            },
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .into_group_map();
    grid.values()
        .flat_map(|antennas| {
            antennas
                .iter()
                .tuple_combinations()
                .flat_map(|(first_antenna, second_antenna)| {
                    [
                        antinode(first_antenna, second_antenna, 2),
                        antinode(first_antenna, second_antenna, -1),
                    ]
                })
        })
        .filter(|antinode| within_grid(antinode, max_x, max_y))
        .unique()
        .count()
}

fn antinode(first_antenna: &Coordinates, second_antenna: &Coordinates, n: i32) -> Coordinates {
    Coordinates {
        x: second_antenna.x + n * (first_antenna.x - second_antenna.x),
        y: second_antenna.y + n * (first_antenna.y - second_antenna.y),
    }
}

fn within_grid(coordinates: &Coordinates, max_x: i32, max_y: i32) -> bool {
    coordinates.y >= 0 && coordinates.x >= 0 && coordinates.x < max_x && coordinates.y < max_y
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(14, solve(read_test_data_for_day("8-0").unwrap()));
    }
}