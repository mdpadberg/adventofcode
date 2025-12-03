use std::collections::HashSet;

use aoc2025::util::read_data_for_day;
use itertools::Itertools;
use thiserror::Error;

fn main() -> Result<(), MyError> {
    let result = solve(read_data_for_day("2").ok_or(MyError::ParsingInputDataError)?)?;
    println!("{result}");
    Ok(())
}

fn solve(input: String) -> Result<u64, MyError> {
    let parsed = parse(input)?;
    let mut invalid_values = HashSet::new();
    for product_id_ranges in parsed {
        for range in product_id_ranges.start..=product_id_ranges.end {
            let range_as_string = range.to_string();
            let amount_of_numbers = range_as_string.len();
            let mid = amount_of_numbers / 2;
            let (first, second) = range_as_string.split_at(mid);
            if first == second {
                invalid_values.insert(range);
            }
        }
    }
    Ok(invalid_values.iter().sum())
}

fn parse(input: String) -> Result<Vec<ProductIdRanges>, MyError> {
    input
        .split(",")
        .map(|line| {
            if let Some((first, second)) = line.split_once("-") {
                Ok(ProductIdRanges {
                    start: first.parse()?,
                    end: second.parse()?,
                })
            } else {
                Err(MyError::ParsingInputDataError)
            }
        })
        .collect::<Result<Vec<ProductIdRanges>, MyError>>()
}

#[derive(Debug)]
struct ProductIdRanges {
    start: u64,
    end: u64,
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
        assert_eq!(
            1227775554,
            solve(read_test_data_for_day("2-0").unwrap()).unwrap()
        );
    }
}
