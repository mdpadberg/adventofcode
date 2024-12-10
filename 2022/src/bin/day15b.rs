use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("{}", solve(read_data_for_day("15").unwrap(), 4000000));
}

fn solve(input: String, max_x_and_y: i64) -> i64 {
    let sensors: Vec<Sensor> = input
        .split("\n")
        .map(|str| Sensor::new(str))
        .collect::<Vec<Sensor>>();
    let diamonds: Vec<Diamond> = sensors
        .iter()
        .map(|sensor| {
            let radius = sensor.coordinates.x.abs_diff(sensor.closest_beacon.x) as i64
                + sensor.coordinates.y.abs_diff(sensor.closest_beacon.y) as i64;
            Diamond::new(sensor.coordinates, radius)
        })
        .collect::<Vec<Diamond>>();
    let (_, distress_beacon) = diamonds
        .iter()
        .flat_map(|diamond| diamond.all_adjacent_points(max_x_and_y))
        .sorted()
        .dedup_with_count()
        .sorted_by_key(|a| a.0)
        .last()
        .unwrap();
    distress_beacon.x * 4000000 + distress_beacon.y
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coordinates {
    x: i64,
    y: i64,
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
                x: values.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                y: values.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            },
            closest_beacon: Coordinates {
                x: values.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                y: values.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Diamond {
    center: Coordinates,
    radius: i64,
}

impl Diamond {
    fn new(center: Coordinates, radius: i64) -> Diamond {
        Diamond { center, radius }
    }

    /// ....A....
    /// ...A#A...
    /// ..A###A..
    /// .A#####A.
    /// A###X###A
    /// .A#####A.   
    /// ..A###A..
    /// ...A#A...
    /// ....A....
    fn all_adjacent_points(&self, max_x_and_y: i64) -> Vec<Coordinates> {
        let mut points: Vec<Coordinates> = Vec::new();
        let (min_y, max_y) = (
            self.center.y - (self.radius + 1),
            self.center.y + (self.radius + 1),
        );
        for y in min_y..=max_y {
            if y < 0 || y > max_x_and_y {
                continue;
            }
            if self.center.x < 0 || self.center.x > max_x_and_y {
                continue;
            }
            if y == min_y || y == max_y {
                points.push(Coordinates {
                    y,
                    x: self.center.x,
                });
            } else {
                let left_x = self.center.x - self.radius - 1 + (self.center.y - y).abs();
                let right_x = self.center.x + self.radius + 1 - (self.center.y - y).abs();
                if left_x < 0 || left_x > max_x_and_y || right_x < 0 || right_x > max_x_and_y {
                    continue;
                }
                points.push(Coordinates { y, x: left_x });
                points.push(Coordinates { y, x: right_x });
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
        assert_eq!(56000011, solve(read_test_data_for_day("15-0").unwrap(), 20));
    }

    #[test]
    fn diamondtest() {
        let diamond = Diamond::new(Coordinates { y: 4, x: 4 }, 3);
        assert_eq!(
            diamond.all_adjacent_points(20),
            vec![
                Coordinates { y: 0, x: 4 },
                Coordinates { y: 1, x: 3 },
                Coordinates { y: 1, x: 5 },
                Coordinates { y: 2, x: 2 },
                Coordinates { y: 2, x: 6 },
                Coordinates { y: 3, x: 1 },
                Coordinates { y: 3, x: 7 },
                Coordinates { y: 4, x: 0 },
                Coordinates { y: 4, x: 8 },
                Coordinates { y: 5, x: 1 },
                Coordinates { y: 5, x: 7 },
                Coordinates { y: 6, x: 2 },
                Coordinates { y: 6, x: 6 },
                Coordinates { y: 7, x: 3 },
                Coordinates { y: 7, x: 5 },
                Coordinates { y: 8, x: 4 },
            ]
        );
    }
}
