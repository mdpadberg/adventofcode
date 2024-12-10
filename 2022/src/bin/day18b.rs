use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

fn main() {
    println!("{}", solve(read_data_for_day(18).unwrap()));
}

fn solve(input: String) -> u32 {
    let points = input
        .split("\n")
        .map(|s| Point::from(s))
        .collect::<HashSet<Point>>();
    let (min, max) = min_max_point_values(&points);
    let mut grid: HashMap<Point, Cube> = HashMap::new();
    for x in min-1..=max {
        for y in min-1..=max {
            for z in min-1..=max {
                if points.contains(&Point { x, y, z }) {
                    grid.insert(Point { x, y, z }, Cube::LAVA);
                } else {
                    grid.insert(Point { x, y, z }, Cube::UNKNOWN);
                }
            }
        }
    }
    floodfill(grid)
}

fn floodfill(grid: HashMap<Point, Cube>) -> u32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::from([Point { x: 0, y: 0, z: 0 }]);
    let mut score: u32 = 0;

    while let Some(point) = queue.pop_front() {
        for neighbour in point.neighbours() {
            //skip neighbours that are outside of the grid
            if !grid.contains_key(&neighbour) {
                continue;
            }

            //skip already visted points
            if visited.contains(&neighbour) {
                continue;
            } else {
                visited.insert(neighbour.clone());
                queue.push_back(neighbour.clone());
            }

            match (grid.get(&point).unwrap(), grid.get(&neighbour).unwrap()) {
                (Cube::LAVA, Cube::LAVA) => score += 1,
                (Cube::LAVA, Cube::UNKNOWN) => score += 1,
                (Cube::UNKNOWN, Cube::LAVA) => score += 1,
                (Cube::UNKNOWN, Cube::UNKNOWN) => {},
            };
        }
    }
    score
}

fn min_max_point_values(points: &HashSet<Point>) -> (i32, i32) {
    let minmax = vec![
        points.iter().map(|p| p.x).minmax(),
        points.iter().map(|p| p.y).minmax(),
        points.iter().map(|p| p.z).minmax(),
    ]
    .into_iter()
    .map(|minmax| match minmax {
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
        _ => panic!("AAAAAAAAAH!"),
    })
    .collect::<Vec<(i32, i32)>>();
    (
        minmax.iter().map(|values| values.0).min().unwrap() - 1,
        minmax.iter().map(|values| values.1).max().unwrap() + 1,
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cube {
    LAVA,
    UNKNOWN,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;
    #[test]
    fn solvetest() {
        assert_eq!(58, solve(read_test_data_for_day(18).unwrap()));
    }
}
