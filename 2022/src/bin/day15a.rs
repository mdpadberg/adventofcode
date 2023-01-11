use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("{}", solve(read_data_for_day(15).unwrap()));
}

fn solve(input: String) -> usize {
    input.split("\n").map(|str| Sensor::new(str)).collect();
    0
}

#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    coordinates: Coordinates,
    closest_beacon: Coordinates,
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)").unwrap();
}

impl Sensor {
    fn new(str: &str) -> Sensor {
        let values = REGEX.captures(str).unwrap();
        Sensor {
            coordinates: Coordinates {
                x: values.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                y: values.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            },
            closest_beacon: Coordinates {
                x: values.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                y: values.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;
    #[test]
    fn solvetest() {
        assert_eq!(26, solve(read_test_data_for_day(15).unwrap()));
    }
}
