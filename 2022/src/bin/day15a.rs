use aoc2022::util::read_data_for_day;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    println!("{}", solve(read_data_for_day("15").unwrap(), 2000000));
}

fn solve(input: String, row: i32) -> usize {
    let sensors: Vec<Sensor> = input
        .split("\n")
        .map(|str| Sensor::new(str))
        .collect::<Vec<Sensor>>();

    let beacons: HashSet<Coordinates> = sensors
        .iter()
        .map(|sensor| sensor.closest_beacon)
        .collect::<HashSet<Coordinates>>();

    let points: HashSet<Coordinates> = sensors
        .iter()
        .map(|sensor| {
            let radius = sensor.coordinates.x.abs_diff(sensor.closest_beacon.x) as i32
                + sensor.coordinates.y.abs_diff(sensor.closest_beacon.y) as i32;
            Diamond::new(sensor.coordinates, radius)
        })
        .flat_map(|diamond| diamond.all_points_in_row(row))
        .collect::<HashSet<Coordinates>>();

    points
        .iter()
        .filter(|point| point.y == row && !beacons.contains(point))
        .count()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    radius: i32,
}

impl Diamond {
    fn new(center: Coordinates, radius: i32) -> Diamond {
        Diamond { center, radius }
    }

    /// ...#...
    /// ..###..
    /// .#####.
    /// ###X###
    /// .#####.    
    /// ..###..
    /// ...#...
    fn all_points_in_row(&self, row: i32) -> HashSet<Coordinates> {
        let mut points = HashSet::new();
        let distance_to_center = self.center.y.abs_diff(row) as i32;
        let left = self.center.x - (self.radius - distance_to_center);
        let right = self.center.x + (self.radius - distance_to_center);
        for x in left..=right {
            points.insert(Coordinates { x, y: row });
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
        assert_eq!(26, solve(read_test_data_for_day("15-0").unwrap(), 10));
    }

    #[test]
    fn diamondtest() {
        let diamond = Diamond::new(Coordinates { y: 3, x: 3 }, 3);

        assert_eq!(
            diamond.all_points_in_row(3),
            HashSet::from_iter(vec![
                Coordinates { y: 3, x: 0 },
                Coordinates { y: 3, x: 1 },
                Coordinates { y: 3, x: 2 },
                Coordinates { y: 3, x: 3 },
                Coordinates { y: 3, x: 4 },
                Coordinates { y: 3, x: 5 },
                Coordinates { y: 3, x: 6 },
            ])
        );
    }
}
