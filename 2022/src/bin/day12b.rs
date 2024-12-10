use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn main() {
    println!("{}", solve(read_data_for_day("12").unwrap()));
}

fn solve(input: String) -> u32 {
    let map = Map::parse(input);
    bfs(map.start, map.end, map.locations)
}

fn bfs(start: Coordinates, end: Coordinates, locations: HashMap<Coordinates, Height>) -> u32 {
    let mut visited: HashMap<Coordinates, Score> = HashMap::new();
    let mut queue: VecDeque<(Coordinates, Score)> = VecDeque::from([(end, Score(0))]);

    while let Some((current_location, score)) = queue.pop_front() {
        if visited.contains_key(&current_location) {
            //skip already visited nodes
            continue;
        } else {
            //insert into visited nodes
            visited.insert(current_location.clone(), score.clone());
        }
        //if current_location score == 10 then we are at the place we want to be
        if &locations.get(&current_location).unwrap() == &&Height(10) {
            break;
        }

        let valid_neighbours: Vec<Coordinates> = current_location
            .neighbours()
            .into_iter()
            .filter(|c| locations.contains_key(c))
            // only allowed to make a step when it is at most one lower
            .filter(|c| {
                locations.get(&c).unwrap().0 >= &locations.get(&current_location).unwrap().0 - 1
            })
            .collect();

        for neighbour in valid_neighbours {
            queue.push_back((neighbour, Score(score.0 + 1)));
        }
    }
    visited.values().max().unwrap().0.to_owned()
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct Score(u32);

#[derive(Debug, Eq, PartialEq)]
struct Height(u32);

#[derive(Debug)]
struct Map {
    locations: HashMap<Coordinates, Height>,
    start: Coordinates,
    end: Coordinates,
}

impl Map {
    fn parse(input: String) -> Map {
        let mut start = Coordinates { x: 0, y: 0 };
        let mut end = Coordinates { x: 0, y: 0 };
        let mut locations: HashMap<Coordinates, Height> = HashMap::new();
        for (y, line) in input.split("\n").enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coordinates = Coordinates {
                    x: x as i32,
                    y: y as i32,
                };
                if c == 'S' {
                    start = coordinates.clone();
                    locations.insert(coordinates, Height(10));
                } else if c == 'E' {
                    end = coordinates.clone();
                    locations.insert(coordinates, Height(35));
                } else {
                    locations.insert(coordinates, Height(c.to_digit(36).unwrap()));
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

impl Coordinates {
    // S   = self
    // a-d = neighbours
    // . a .
    // b S c
    // . d .
    fn neighbours(&self) -> Vec<Coordinates> {
        vec![
            //a
            Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            //b
            Coordinates {
                x: self.x - 1,
                y: self.y,
            },
            //c
            Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            //d
            Coordinates {
                x: self.x,
                y: self.y + 1,
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
        assert_eq!(29, solve(read_test_data_for_day("12-0").unwrap()));
    }
}
