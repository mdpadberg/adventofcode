use aoc2025::util::read_data_for_day;
use itertools::Itertools;
use thiserror::Error;

fn main() -> Result<(), MyError> {
    let result = solve(read_data_for_day("1").ok_or(MyError::ParsingInputDataError)?)?;
    println!("{result}");
    Ok(())
}

fn solve(input: String) -> Result<i16, MyError> {
    let mut safe_dial = 50;
    let mut clicks = 0;
    for direction in parse(input)?.iter() {
        let rotation = match direction {
            RotationDirection::R(rotation) => *rotation,
            RotationDirection::L(rotation) => *rotation,
        };
        for _ in 0..rotation {
            let new_safe_dial: i16 = match direction {
                RotationDirection::R(_) => safe_dial + 1,
                RotationDirection::L(_) => safe_dial - 1,
            };
            safe_dial = new_safe_dial.rem_euclid(100);
            if safe_dial == 0 {
                clicks += 1;
            }
        }
    }
    Ok(clicks)
}

fn parse(input: String) -> Result<Vec<RotationDirection>, MyError> {
    input
        .split("\n")
        .map(|line| line.split_at(1))
        .map(|(first, second)| {
            if first == "L" {
                Ok(RotationDirection::L(second.parse()?))
            } else {
                Ok(RotationDirection::R(second.parse()?))
            }
        })
        .collect::<Result<Vec<RotationDirection>, MyError>>()
}

#[derive(Debug)]
enum RotationDirection {
    R(i16),
    L(i16),
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
        assert_eq!(6, solve(read_test_data_for_day("1-0").unwrap()).unwrap());
    }
}
