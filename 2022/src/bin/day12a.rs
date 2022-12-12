use std::collections::HashSet;

use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day(11).unwrap()));
}

fn solve(input: String) -> u64 {
    let map = Map::parse(input);
    //data parsing is done, now dijkstra??
    0
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Coordinates {
    x: u32,
    y: u32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Location {
    coordinates: Coordinates,
    height: u32,
}

#[derive(Debug)]
struct Map {
    locations: HashSet<Location>,
    start: Coordinates,
    end: Coordinates,
}

impl Map {
    fn parse(input: String) -> Map {
        let mut start = Coordinates { x: 0, y: 0 };
        let mut end = Coordinates { x: 0, y: 0 };
        let mut locations: HashSet<Location> = HashSet::new();
        for (y, line) in input.split("\n").enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coordinates = Coordinates {
                    x: x as u32,
                    y: y as u32,
                };
                if c == 'S' {
                    start = coordinates.clone();
                    locations.insert(Location {
                        coordinates,
                        height: 10,
                    });
                } else if c == 'E' {
                    end = coordinates.clone();
                    locations.insert(Location {
                        coordinates,
                        height: 35,
                    });
                } else {
                    locations.insert(Location {
                        coordinates,
                        height: c.to_digit(36).unwrap(),
                    });
                }
            }
        }
        Map {
            locations,
            start,
            end,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(31, solve(read_test_data_for_day(12).unwrap()));
    }
}
