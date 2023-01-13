use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    println!("{}", solve(read_data_for_day(15).unwrap()));
}

fn solve(input: String) -> usize {
    input
        .split("\n")
        .map(|str| Sensor::new(str))
        .collect::<Vec<Sensor>>();
    0
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
struct Diamond {
    center: Coordinates,
}

impl Diamond {
    fn new(center: Coordinates) -> Diamond {
        Diamond { center }
    }

    /// ...#...
    /// ..###..
    /// .#####.
    /// ###X###
    /// .#####.    
    /// ..###..
    /// ...#...
    fn all_points(&self, radius: i32) -> HashSet<Coordinates> {
        let mut points = HashSet::new();
        for y in self.center.y - radius..=self.center.y + radius {
            let distance_to_center = self.center.y.abs_diff(y) as i32;
            let left = self.center.x - (radius - distance_to_center);
            let right = self.center.x + (radius - distance_to_center);
            for x in left..=right {
                points.insert(Coordinates { x, y });
            }
        }
        points
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

    #[test]
    fn diamondtest() {
        let diamond = Diamond::new(Coordinates { y: 3, x: 3 });

        assert_eq!(
            diamond.all_points(3),
            HashSet::from_iter(vec![
                Coordinates { y: 0, x: 3 },
                Coordinates { y: 1, x: 2 },
                Coordinates { y: 1, x: 3 },
                Coordinates { y: 1, x: 4 },
                Coordinates { y: 2, x: 1 },
                Coordinates { y: 2, x: 2 },
                Coordinates { y: 2, x: 3 },
                Coordinates { y: 2, x: 4 },
                Coordinates { y: 2, x: 5 },
                Coordinates { y: 3, x: 0 },
                Coordinates { y: 3, x: 1 },
                Coordinates { y: 3, x: 2 },
                Coordinates { y: 3, x: 3 },
                Coordinates { y: 3, x: 4 },
                Coordinates { y: 3, x: 5 },
                Coordinates { y: 3, x: 6 },
                Coordinates { y: 4, x: 1 },
                Coordinates { y: 4, x: 2 },
                Coordinates { y: 4, x: 3 },
                Coordinates { y: 4, x: 4 },
                Coordinates { y: 4, x: 5 },
                Coordinates { y: 5, x: 2 },
                Coordinates { y: 5, x: 3 },
                Coordinates { y: 5, x: 4 },
                Coordinates { y: 6, x: 3 },
            ])
        );
    }
}
