use aoc2025::util::read_data_for_day;
use itertools::Itertools;
use std::collections::HashMap;
use thiserror::Error;

fn main() -> Result<(), MyError> {
    let result = solve(read_data_for_day("4").ok_or(MyError::ParsingInputDataError)?)?;
    println!("{result}");
    Ok(())
}

fn solve(input: String) -> Result<usize, MyError> {
    let grid = parse(input)?;
    Ok(grid
        .iter()
        .filter(|point| point.1 == &Value::PAPER)
        .filter(|point| {
            point
                .0
                .get_neighbors(&grid)
                .iter()
                .filter(|point| point.1 == Value::PAPER)
                .count()
                < 4
        })
        .count())
}

fn parse(input: String) -> Result<Grid, MyError> {
    let mut grid: Grid = HashMap::new();
    let lines = input.split("\n");
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let value = match char {
                '@' => Ok(Value::PAPER),
                '.' => Ok(Value::EMPTY),
                _ => Err(MyError::ParsingInputDataError),
            }?;
            grid.insert(
                Coordinates {
                    x: x as isize,
                    y: y as isize,
                },
                value,
            );
        }
    }
    Ok(grid)
}

type Grid = HashMap<Coordinates, Value>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    PAPER,
    EMPTY,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Coordinates {
    y: isize,
    x: isize,
}

impl Coordinates {
    pub fn get_neighbors(&self, grid: &Grid) -> Vec<(Coordinates, Value)> {
        let neighbors = vec![
            Coordinates {
                y: self.y - 1,
                x: self.x - 1,
            },
            Coordinates {
                y: self.y - 1,
                x: self.x,
            },
            Coordinates {
                y: self.y - 1,
                x: self.x + 1,
            },
            Coordinates {
                y: self.y,
                x: self.x - 1,
            },
            Coordinates {
                y: self.y,
                x: self.x + 1,
            },
            Coordinates {
                y: self.y + 1,
                x: self.x - 1,
            },
            Coordinates {
                y: self.y + 1,
                x: self.x,
            },
            Coordinates {
                y: self.y + 1,
                x: self.x + 1,
            },
        ];
        let mut result = vec![];
        for neighbor in neighbors {
            if let Some(value) = grid.get(&neighbor) {
                result.push((neighbor, value.clone()));
            }
        }
        result
    }
}

#[derive(Debug, Error)]
enum MyError {
    #[error("ParseIntError")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParsingInputDataError")]
    ParsingInputDataError,
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2025::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(13, solve(read_test_data_for_day("4-0").unwrap()).unwrap());
    }
}
