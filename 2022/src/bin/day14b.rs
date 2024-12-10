use std::collections::HashMap;

use aoc2022::util::read_data_for_day;
use itertools::Itertools;

fn main() {
    println!("{}", solve(read_data_for_day("14").unwrap()));
}

fn solve(input: String) -> usize {
    let mut grid: HashMap<Point, Value> = to_grid(input);
    let floor = grid.iter().map(|(key, _)| key.y).max().unwrap() + 2;
    loop {
        let mut sand = Point { x: 500, y: 0 };
        loop {
            let is_falling = sand.fall(&grid);
            if is_falling && sand.y < floor - 1 {
                continue;
            } else {
                break;
            }
        }
        if sand == (Point { x: 500, y: 0 }) {
            break;
        }
        grid.insert(sand, Value::SAND);
    }
    grid.iter()
        .filter(|(_, value)| value == &&Value::SAND)
        .count()
        + 1
}

fn to_grid(input: String) -> HashMap<Point, Value> {
    let lines: Vec<Vec<&str>> = input
        .split("\n")
        .map(|line| line.split(" -> ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut grid: HashMap<Point, Value> = HashMap::new();
    for line in lines {
        for pairs in line.windows(2) {
            let (left_x, left_y) = pairs[0]
                .split(",")
                .map(|a| a.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap();
            let (right_x, right_y) = pairs[1]
                .split(",")
                .map(|a| a.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap();
            //Horizontal line
            if left_x != right_x {
                for x in if left_x < right_x {
                    left_x..=right_x
                } else {
                    right_x..=left_x
                } {
                    grid.insert(Point { x, y: left_y }, Value::ROCK);
                }
            //Vertical line
            } else {
                for y in if left_y < right_y {
                    left_y..=right_y
                } else {
                    right_y..=left_y
                } {
                    grid.insert(Point { x: left_x, y }, Value::ROCK);
                }
            }
        }
    }
    grid
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Value {
    ROCK,
    SAND,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    y: i64,
    x: i64,
}

impl Point {
    fn fall(&mut self, grid: &HashMap<Point, Value>) -> bool {
        if !grid.contains_key(&Point {
            x: self.x,
            y: self.y + 1,
        }) {
            //down is free
            self.y += 1;
            true
        } else if !grid.contains_key(&Point {
            x: self.x - 1,
            y: self.y + 1,
        }) {
            //down-left is free
            self.x -= 1;
            self.y += 1;
            true
        } else if !grid.contains_key(&Point {
            x: self.x + 1,
            y: self.y + 1,
        }) {
            //down-right is free
            self.x += 1;
            self.y += 1;
            true
        } else {
            //unit comes to rest
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;
    #[test]
    fn solvetest() {
        assert_eq!(93, solve(read_test_data_for_day("14-0").unwrap()));
    }
}
