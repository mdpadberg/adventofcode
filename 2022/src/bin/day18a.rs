use std::collections::HashSet;

use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("18").unwrap()));
}

fn solve(input: String) -> u32 {
    let points = input
        .split("\n")
        .map(|s| Point::from(s))
        .collect::<HashSet<Point>>();

    points
        .iter()
        .flat_map(|p| {
            p.neighbours()
                .iter()
                .map(|neighbour| if points.contains(neighbour) { 0 } else { 1 })
                .collect::<Vec<u32>>()
        })
        .sum::<u32>()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

type Neighbours = [Point; 6];

impl Point {
    fn from(str: &str) -> Point {
        let (x, y, z) = str
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple::<(i32, i32, i32)>()
            .unwrap();
        Point { x, y, z }
    }

    fn neighbours(&self) -> Neighbours {
        [
            Point {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Point {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Point {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Point {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Point {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Point {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;
    #[test]
    fn solvetest() {
        assert_eq!(64, solve(read_test_data_for_day("18-0").unwrap()));
    }
}
