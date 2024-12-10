use aoc2024::util::read_data_for_day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("{:?}", solve(read_data_for_day("6").unwrap()));
}

type Grid = HashMap<Coordinates, Space>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn next(&self, direction: &Direction) -> Coordinates {
        match direction {
            Direction::UP => Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            Direction::RIGHT => Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            Direction::DOWN => Coordinates {
                x: self.x,
                y: self.y + 1,
            },
            Direction::LEFT => Coordinates {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, PartialEq)]
enum Space {
    FREE,
    OBSTRUCTIONS,
    GAURD,
}

impl Space {
    fn new(value: char) -> Space {
        match value {
            '.' => Space::FREE,
            '#' => Space::OBSTRUCTIONS,
            _ => Space::GAURD,
        }
    }
}

#[derive(Debug)]
struct Gaurd {
    current_position: Coordinates,
    current_direction: Direction,
    steps: HashSet<Coordinates>,
}

impl Gaurd {
    fn hit(&self, grid: &Grid) -> bool {
        let next_coordinates = self.current_position.next(&self.current_direction);
        if let Some(some) = grid.get(&next_coordinates) {
            some == &Space::OBSTRUCTIONS
        } else {
            false
        }
    }

    fn walk(&mut self, grid: &Grid) -> bool {
        let next_coordinates = self.current_position.next(&self.current_direction);
        if let Some(_) = grid.get(&next_coordinates) {
            self.steps.insert(self.current_position);
            self.current_position = next_coordinates;
            true
        } else {
            false
        }
    }

    fn turn(&mut self) {
        match self.current_direction {
            Direction::UP => self.current_direction = Direction::RIGHT,
            Direction::RIGHT => self.current_direction = Direction::DOWN,
            Direction::DOWN => self.current_direction = Direction::LEFT,
            Direction::LEFT => self.current_direction = Direction::UP,
        }
    }
}

fn solve(input: String) -> usize {
    let grid = input
        .split("\n")
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| (Coordinates { x, y }, Space::new(char)))
                .collect::<Grid>()
        })
        .collect::<Grid>();
    let mut gaurd = Gaurd {
        current_position: grid
            .iter()
            .find(|value| value.1 == &Space::GAURD)
            .unwrap()
            .0
            .clone(),
        current_direction: Direction::UP,
        steps: HashSet::new(),
    };
    while gaurd.walk(&grid) {
        if gaurd.hit(&grid) {
            gaurd.turn();
        }
    }
    gaurd.steps.len() + 1
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(41, solve(read_test_data_for_day("6-0").unwrap()));
    }
}
