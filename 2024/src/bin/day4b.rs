use aoc2024::util::read_data_for_day;
use itertools::Itertools;
use std::{collections::HashMap, ops::Add};

fn main() {
    println!("{:?}", solve(read_data_for_day("4").unwrap()));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates {
    x: i8,
    y: i8,
}

impl Add for &Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn solve(input: String) -> u32 {
    let directions = vec![
        //Cross
        vec![Coordinates { x: 0, y: 0 }, Coordinates { x: 1, y: 1 }, Coordinates { x: 2, y: 2 }],
        //Cross
        vec![Coordinates { x: 2, y: 0 }, Coordinates { x: 1, y: 1 }, Coordinates { x: 0, y: 2 }],
    ];
    let grid: HashMap<Coordinates, char> = input
        .split("\n")
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, character)| {
                    (
                        Coordinates {
                            x: x as i8,
                            y: y as i8,
                        },
                        character,
                    )
                })
                .collect::<HashMap<Coordinates, char>>()
        })
        .collect();
    let mut result = 0;
    for (current_coordinates, _) in grid.iter() {
        let mut words = vec![];
        for direction in directions.iter() {
            let mut word = String::from("");
            for direction_coordinates in direction {
                let new_coordinates = current_coordinates + direction_coordinates;
                if let Some(some) = grid.get(&new_coordinates) {
                    word.push(*some);
                }
            }
            words.push(word);
        }
        if words.iter().all(|word| word == "MAS" || word == "SAM") {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2024::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(9, solve(read_test_data_for_day("4-1").unwrap()));
    }
}
